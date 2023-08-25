import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import { SortType, type Config, SortDirection, type Project, type ProjectCache } from "./types";
 
export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export function formatBytes(bytes: number, decimals: number = 2) {
  if (!+bytes) return '0 Bytes'

  const k = 1024
  const dm = decimals < 0 ? 0 : decimals
  const sizes = ['Bytes', 'KiB', 'MiB', 'GiB', 'TiB', 'PiB', 'EiB', 'ZiB', 'YiB']

  const i = Math.floor(Math.log(bytes) / Math.log(k))

  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`
}

export function selectDirCache(
  config: Config,
  projectDirs: Project[],
  projectCache: ProjectCache,
  selectedDir: string | null
): Project[] {
  if (selectedDir == null) return [];
  projectDirs = projectCache.get(selectedDir) || [];
  sortByType(config, projectDirs);
  return projectDirs;
}

export function sortByType(config: Config, projectDirs: Project[]) {
  if (config == null) return;
  switch (config.sortType) {
    case SortType.ByName:
      switch (config.sortDirection) {
        // Must add .toLowerCase() because it looks over ascii codes
        case SortDirection.ASC: projectDirs = projectDirs?.sort((a, b) => (a.name.toLowerCase() > b.name.toLowerCase()) ? 1 : ((b.name.toLowerCase() > a.name.toLowerCase()) ? -1 : 0)); break;
        case SortDirection.DESC: projectDirs = projectDirs?.sort((a, b) => (a.name.toLowerCase() < b.name.toLowerCase()) ? 1 : ((b.name.toLowerCase() < a.name.toLowerCase()) ? -1 : 0)); break;
      }
      break;
    case SortType.BySize:
      switch (config.sortDirection) {
        case SortDirection.ASC: projectDirs = projectDirs?.sort((a, b) => a.full_build_size - b.full_build_size); break;
        case SortDirection.DESC: projectDirs = projectDirs?.sort((a, b) => b.full_build_size - a.full_build_size); break;
      }
      break;
  }
}
