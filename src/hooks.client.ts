import { writeTextFile, exists, BaseDirectory, createDir } from '@tauri-apps/api/fs';
import { appConfigDir } from '@tauri-apps/api/path';
import { SortDirection, SortType, type Config, type ProjectCache } from "$lib/types";

async function setup() {
  if (!exists(await appConfigDir())) {
    await createDir(await appConfigDir());
  }
  await createConfig();
  await createProjectCache();

}

setup()

async function createConfig() {
  const configExists = await exists('config.json', { dir: BaseDirectory.AppConfig });
  if (!configExists) {
    let content: Config = {
      last_searched_path: null,
      sortDirection: SortDirection.ASC,
      sortType: SortType.ByName,
    };
    await writeTextFile('config.json', JSON.stringify(content), { dir: BaseDirectory.AppConfig })
  }
}

async function createProjectCache() {
  const projectCache = await exists('projects.json', { dir: BaseDirectory.AppConfig });
  if (!projectCache) {
    let content: ProjectCache = new Map();
    await writeTextFile('projects.json', JSON.stringify(content), { dir: BaseDirectory.AppConfig })
  }
}
