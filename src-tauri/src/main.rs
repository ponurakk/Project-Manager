// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::str::FromStr;

use serde::Serialize;
use once_cell::sync::Lazy;

mod commands;

static PROJECT_TYPES: Lazy<Vec<&str>> = Lazy::new(|| vec![
    ".git",
    "node_modules", // javascript
    "target", // rust, java, kotlin, scala
    "_build", // elixir, erlang
    "build", // c, c++,
    "dist", // python, typescript
    "vendor", // php
    "out",
    "zig-out", // zig
]);

#[derive(Serialize)]
struct ProjectDir {
    dir: ProjectTypes,
    size: u64,
}

#[derive(Serialize)]
enum ProjectTypes {
    Git,
    NodeModules,
    Target,
    _Build,
    Build,
    Dist,
    Vendor,
    Out,
    ZigOut,
}

impl FromStr for ProjectTypes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ".git" => Ok(Self::Git),
            "node_modules" => Ok(Self::NodeModules),
            "target" => Ok(Self::Target),
            "_build" => Ok(Self::_Build),
            "build" => Ok(Self::Build),
            "dist" => Ok(Self::Dist),
            "vendor" => Ok(Self::Vendor),
            "out" => Ok(Self::Out),
            "zig-out" => Ok(Self::ZigOut),
            _ => Err(())
        }
    }
}

#[derive(Serialize)]
pub struct Project {
    name: String,
    path: String,
    full_build_size: u64,
    has_build_dirs: bool,
    build_dirs: Vec<ProjectDir>,
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::projects::find_projects
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize)]
pub enum AppError {
    IoError(String),
    NoneError(()),
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value.to_string())
    }
}

impl From<()> for AppError {
    fn from(value: ()) -> Self {
        Self::NoneError(value)
    }
}
