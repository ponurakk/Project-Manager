<script lang="ts">
  import { Button } from "$lib/components/ui/button"
  import * as Select from "$lib/components/ui/select";
  import { Input } from "$lib/components/ui/input";
  import * as Table from "$lib/components/ui/table";
  import * as Tooltip from "$lib/components/ui/tooltip";

  import { invoke } from "@tauri-apps/api/tauri";
  import { open } from '@tauri-apps/api/dialog';
  import { SortType, type Config, type Project, type ProjectCache, SortDirection } from "$lib/types";
  import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";

  import type { PageData } from "./$types";

  export let data: PageData;
  let config: Config = data.config;

  let projectCache: ProjectCache = data.projectCache;

  $: {
    console.log(projectCache);
  }

  let projectDirs: Project[] = [];
  let selectedDir: string | string[] | null;
  let totalSize: number = 0;
  let filteredProjectsNumber: number = 0;

  let search: string = "";

  async function selectDirDialog() {
    selectedDir = await open({
      directory: true,
      multiple: false,
    });

    try {
      console.log("project_time")
      console.time("project_time")
      projectDirs = await invoke("find_projects", { projectDir: selectedDir })
      getSortType(config);
      projectCache.set(selectedDir as string, projectDirs!);
      await writeTextFile('projects.json', JSON.stringify(Object.fromEntries(projectCache)), { dir: BaseDirectory.AppConfig });
    } catch(error) {
      console.error(error)
    }
  }
  async function selectDirCache() {
    projectDirs = projectCache.get(selectedDir as string) || [];
    getSortType(config);
  }
  $: {
    console.log(projectDirs)
    console.timeEnd("project_time")
  }

  // function shorten_string(str: string, max_length: number): string {
  //   return str.substring(0, max_length) + "/.../" + str.substring(str.length - max_length, str.length);
  // }

  function formatBytes(bytes: number, decimals: number = 2) {
    if (!+bytes) return '0 Bytes'

    const k = 1024
    const dm = decimals < 0 ? 0 : decimals
    const sizes = ['Bytes', 'KiB', 'MiB', 'GiB', 'TiB', 'PiB', 'EiB', 'ZiB', 'YiB']

    const i = Math.floor(Math.log(bytes) / Math.log(k))

    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`
  }

  function getSortType(config: Config) {
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

  $: {
    getSortType(config)
    if (projectDirs.length > 0) {
      // Set default
      totalSize = 0;
      filteredProjectsNumber = 0;

      for (const projectDir of projectDirs) {
        if (projectDir.path.toLowerCase().includes(search.toLowerCase())) {
          totalSize += projectDir.full_build_size;
          filteredProjectsNumber += 1;
        }
      }
      // totalSize = projectDirs.reduce((prev, curr) => prev.full_build_size += curr.full_build_size);
      console.log(totalSize)
    }
  }
</script>

<div>
  <div class="m-auto w-max">
    <div class="inline-block">
      <Select.Root bind:value={selectedDir} onValueChange={selectDirCache}>
        <Select.Trigger class="w-[300px]">
          <Select.Value placeholder="Choose cached dir..." />
        </Select.Trigger>
        <Select.Content>
          {#each [...projectCache] as [key, _]}
            <Select.Item value="{key}">{key}</Select.Item>
          {/each}
        </Select.Content>
      </Select.Root>
    </div>

    <div class="inline-block mt-3">
      <Select.Root bind:value={config.sortType}>
        <Select.Trigger class="w-[150px]">
          <Select.Value placeholder="Select Sort Type" />
        </Select.Trigger>
        <Select.Content>
          <Select.Item value="{SortType.ByName}">ByName</Select.Item>
          <Select.Item value="{SortType.BySize}">BySize</Select.Item>
        </Select.Content>
      </Select.Root>
    </div>

    <div class="inline-block mb-3">
      <Select.Root bind:value={config.sortDirection}>
        <Select.Trigger class="w-[150px]">
          <Select.Value placeholder="Select Sort Type" />
        </Select.Trigger>
        <Select.Content>
          <Select.Item value="{SortDirection.ASC}" aria-selected>Ascending</Select.Item>
          <Select.Item value="{SortDirection.DESC}">Descending</Select.Item>
        </Select.Content>
      </Select.Root>
    </div>
  </div>

  <div class="m-auto mb-3 w-max">
    <div class="inline-block">
      <Input type="text" placeholder="Search name of the project..." class="max-w-xs" bind:value={search} />
    </div>
    <Button variant="default" on:click={selectDirDialog} class="inline-block">Select</Button>
  </div>
  <Table.Root class="w-max m-auto">
    <Table.Caption>Number of projects: {filteredProjectsNumber} of total size: {formatBytes(totalSize)}</Table.Caption>
    <Table.Header>
      <Table.Row>
        <Table.Head class="w-[200px]">Name</Table.Head>
        <Table.Head>Total Size</Table.Head>
        <Table.Head class="w-[200px]">Build Dirs</Table.Head>
        <Table.Head class="text-right">Actions</Table.Head>
      </Table.Row>
    </Table.Header>
    <Table.Body>
      {#each projectDirs as projectDir}
        {#if projectDir.path.toLowerCase().includes(search.toLowerCase())}
          <Table.Row>
            <Table.Cell class="font-medium">
              <Tooltip.Root>
                <Tooltip.Trigger>{projectDir.name}</Tooltip.Trigger>
                <Tooltip.Content>
                  <p>{projectDir.path}</p>
                </Tooltip.Content>
              </Tooltip.Root>
            </Table.Cell>
            <Table.Cell>{formatBytes(projectDir.full_build_size)}</Table.Cell>
            <Table.Cell>
              {#each projectDir.build_dirs as dir}
                <p>{dir.dir} - <strong>{formatBytes(dir.size)}</strong></p>
              {/each}
            </Table.Cell>
            <Table.Cell class="text-right">
              <Button variant="default">Manage</Button>
            </Table.Cell>
          </Table.Row>
        {/if}
      {/each}
    </Table.Body>
  </Table.Root>
</div>
