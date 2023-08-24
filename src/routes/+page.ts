import type { ProjectCache, Config } from "$lib/types";
import { BaseDirectory, readTextFile } from "@tauri-apps/api/fs";

export async function load({}) {
  let config: Config = JSON.parse(await readTextFile('config.json', { dir: BaseDirectory.AppConfig }));
  let projectCache: ProjectCache = new Map(Object.entries(JSON.parse(await readTextFile('projects.json', { dir: BaseDirectory.AppConfig }))));

  return {
    config,
    projectCache
  }
}
