use std::time::Duration;
use std::{collections::HashMap, str::FromStr};

use fs_extra::dir::get_size;
use serde::Serialize;
use tauri::{Window, State};
use walkdir::WalkDir;
use tokio_util::sync::CancellationToken;

use crate::{Project, AppError, PROJECT_TYPES, ProjectTypes, ProjectDir, CancelState};

#[derive(Serialize, Clone)]
struct SearchFilesPayload {
    message: String,
}

#[derive(Serialize, Clone)]
struct LoadingStatusPayload {
    status: bool,
    duration: Duration,
}

#[tauri::command]
pub async fn find_projects(project_dir: String, window: Window, cancel_state: State<'_, CancelState>) -> Result<Vec<Project>, AppError> {
    // create token for cancelation;
    let token = CancellationToken::new();
    let cloned_token = token.clone();

    // Start handle to finish as cancel token or desired function
    let handle = tokio::spawn(async move {
        tokio::select! {
            _ = cloned_token.cancelled() => Err(AppError::Cancel("Command was canceled".to_owned())),
            val = find_projects_handle(project_dir, window) => val
        }
    });

    let state_guard = &cancel_state.0;
    let state_clone = state_guard.clone();
    tokio::spawn(async move {
        // check every 100ms if state changed then cancel handle
        loop {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            let state = *state_clone.lock().unwrap();
            if state == true {
                *state_clone.lock().unwrap() = false;
                token.cancel();
                break;
            }
            drop(state);
        }
    });

    handle.await.unwrap()
}
pub async fn find_projects_handle(
    project_dir: String,
    window: Window,
) -> Result<Vec<Project>, AppError> {
    let start = std::time::Instant::now();
    window.emit("loading-status", LoadingStatusPayload { status: true, duration: Duration::from_secs(0) }).unwrap();
    window.emit("search-files", SearchFilesPayload { message: "Searching...".into() }).unwrap();

    let vector: Vec<String> = tokio::spawn(async move {
        list_projects(project_dir).await
    }).await.unwrap_or(Vec::new());
    let mut ret: Vec<Project> = vec![];
    let mut projects: HashMap<String, Vec<String>> = HashMap::new();

    for item in vector {
        // Path in vec. example:
        // /home/user/Projects -> ["home", "user", "Projects"]
        let mut dir_slice: Vec<String> = item.split("/").map(std::string::ToString::to_string).collect::<Vec<String>>();
        let dir_slice_clone: Vec<String> = dir_slice.clone();
        let arr: Vec<(usize, String)> = tokio::spawn(async move {
            get_project_type(dir_slice_clone).await
        }).await.unwrap_or(Vec::new());

        // index of the target folder and name of that folder
        let target_index: &(usize, String) = arr.get(0).unwrap();

        // remove target folder
        dir_slice.splice(target_index.0..dir_slice.len(), []);

        // Get project from project list 
        let projects_item = projects.get(&dir_slice.join("/"));

        // If project item does not exist add it
        if projects_item.is_none() {
            projects.insert(dir_slice.join("/"), vec![target_index.1.clone()]);
        } else {
            // Get list of target dirs to vec and add it to previous one
            let mut projects_item2 = projects_item.unwrap().to_vec();
            projects_item2.push(target_index.1.clone());
            projects.insert(dir_slice.join("/"), projects_item2.clone());
        }
    }

    for (key, value) in projects.into_iter() {
        let path = key.clone();
        let name = path.split('/').last().unwrap().to_owned();

        window.emit("search-files", SearchFilesPayload { message: format!("Validating file: '{}'...", name).into() }).unwrap();

        let mut has_build_dirs = false;
        // if project has only .git dir it means that it isn't built
        if value.ne(&[".git"]) {
            has_build_dirs = true;
        }

        let mut full_build_size: u64 = 0;

        // iter over build dirs
        window.emit("search-files", SearchFilesPayload { message: format!("Fetching build dirs from: '{}'...", name).into() }).unwrap();
        let build_dirs: Vec<ProjectDir> = value.iter().map(|v| {
            let size = get_size(format!("{}/{}", path, v)).expect("Failed to get size of directory");
            full_build_size += size;
            ProjectDir {
                dir: ProjectTypes::from_str(v).unwrap(),
                size,
            }
        }).collect();

        window.emit("search-files", SearchFilesPayload { message: format!("Getting language of: '{}'...", name).into() }).unwrap();
        let build_dirs_clone = build_dirs.clone();
        let language = tokio::spawn(async move {
            get_language_in_project(&key, build_dirs_clone).await.to_string()
        }).await.unwrap_or(loc::Lang::Unrecognized.to_string());

        ret.push(Project {
            name,
            path,
            full_build_size,
            has_build_dirs,
            build_dirs,
            language,
        });
    }

    window.emit("search-files", SearchFilesPayload { message: "Finished.".into() }).unwrap();
    window.emit("loading-status", LoadingStatusPayload { status: false, duration: start.elapsed() }).unwrap();
    Ok(ret)
}

async fn list_projects(project_dir: String) -> Vec<String> {
    let mut vector: Vec<String> = Vec::new();

    // Iterate over selected dir
    for entry in WalkDir::new(project_dir) {
        // Convert `DirEntry` to path
        let entry = entry.unwrap();
        let path = entry.path().display().to_string();

        // Check if it is top path and doesn't repeat target directory
        if PROJECT_TYPES.iter().any(|v| path.ends_with(&format!("/{}", v)) && !has_repeating_elements(path.as_ref())) {
            vector.push(path);
        }
    }
    vector
}

async fn get_project_type(dir_slice: Vec<String>) -> Vec<(usize, String)> {
    let mut arr: Vec<(usize, String)> = Vec::new();
    PROJECT_TYPES.iter().all(|v| {
        let index = dir_slice.iter().position(|p| p == v);
        if index.is_some() {
            arr.push((index.unwrap(), v.to_string()));
        }
        true
    });
    arr
}

fn has_repeating_elements(input: &str) -> bool {
    let elements: Vec<&str> = PROJECT_TYPES.clone().into();
    
    // split is needed in case of string in string ex: out, zig-out
    let input_vec: Vec<_> = input.split("/").collect();
    for i in 0..elements.len() {
        // check if input contains project type
        if input_vec.contains(&elements[i]) {
            // check if appears more than once
            if input.matches(&elements[i]).count() >= 2 {
                return true;
            }

            // iter over the rest of elements
            for j in i + 1..elements.len() {
                if input_vec.contains(&elements[j]) {
                    return true;
                }
            }
        }
    }
    false
}

async fn get_language_in_project(dir: &String, build_dirs: Vec<ProjectDir>) -> loc::Lang {
    let mut total: Vec<loc::Lang> = Default::default();
    let full_project_dirs: Vec<String> = build_dirs.iter().map(|v| v.dir.to_string()).collect();

    // iter over project
    for entry in WalkDir::new(dir) {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.display().to_string();
        // check if is in build dir
        let contains_dir = full_project_dirs.iter()
            .any(|v| path_str.split("/")
                .map(std::borrow::ToOwned::to_owned)
                .collect::<Vec<String>>().contains(v)
            );
        if !path.is_dir() && !contains_dir {
            let lang = loc::lang_from_ext(&path_str);
            total.push(lang);
        }
    }

    let mut total_count: HashMap<loc::Lang, usize> = HashMap::new();
    // count languages
    for value in total {
        *total_count.entry(value).or_default() += 1;
    }
    let language = total_count
        .iter()
        .filter(|(lang, _count)| **lang != loc::Lang::Unrecognized)
        .max_by_key(|entry| entry.1).unwrap_or((&loc::Lang::Unrecognized, &0));

    *language.0
}
