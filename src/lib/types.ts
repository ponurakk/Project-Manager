export interface Project {
  name: string;
  path: string;
  full_build_size: number;
  has_build_dirs: boolean;
  build_dirs: ProjectDir[];
}

export interface ProjectDir {
  dir: ProjectTypes,
  size: number,
}

export enum ProjectTypes {
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

export type ProjectCache = Map<string, Project[]>;

export enum SortDirection {
  ASC,
  DESC
}

export enum SortType {
  ByName,
  BySize
}

export interface Config {
  last_searched_path: string | null;
  sortDirection: SortDirection;
  sortType: SortType;
}
