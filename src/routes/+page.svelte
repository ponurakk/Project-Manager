<script lang="ts">
  // ui components
  import { Button } from "$lib/components/ui/button"
  import { Input } from "$lib/components/ui/input";
  import * as Table from "$lib/components/ui/table";
  import * as Tooltip from "$lib/components/ui/tooltip";
  import { Separator } from "$lib/components/ui/separator";

  // components
  import ManageDialog from "$lib/components/ManageDialog.svelte";
  import ProjectChoose from "$lib/components/ProjectChoose.svelte";
  import Pagination from "$lib/components/Pagination.svelte";

  // utils
  import { formatBytes, sortByType } from "$lib/utils";

  // tauri functions
  import { invoke } from "@tauri-apps/api/tauri";
  import { open } from '@tauri-apps/api/dialog';
  import { BaseDirectory, writeTextFile } from "@tauri-apps/api/fs";

  // types
  import type { PageData } from "./$types";
  import type { Config, Project, ProjectCache } from "$lib/types";

  // variables init
  export let data: PageData;
  let config: Config = data.config;

  let projectCache: ProjectCache = data.projectCache;

  let projectDirs: Project[] = [];
  let filteredProjectDirs: Project[] = projectDirs;
  let selectedDir: string | null;
  let totalSize: number = 0;
  let filteredProjectsNumber: number = 0;
  let currentPage: number = 1;
  let currentPageClone: number = currentPage;
  let elementsPerPage: number = 10;

  let search: string = "";

  async function selectDirDialog() {
    selectedDir = await open({
      directory: true,
      multiple: false,
    }) as string;

    try {
      projectDirs = await invoke("find_projects", { projectDir: selectedDir })
      sortByType(config, projectDirs);
      projectCache.set(selectedDir as string, projectDirs!);
      await writeTextFile('projects.json', JSON.stringify(Object.fromEntries(projectCache)), { dir: BaseDirectory.AppConfig });
    } catch(error) {
      console.error(error)
    }
  }

  // function shorten_string(str: string, max_length: number): string {
  //   return str.substring(0, max_length) + "/.../" + str.substring(str.length - max_length, str.length);
  // }

  $: {
    if (projectDirs.length > 0) {
      // Set default
      totalSize = 0;
      filteredProjectsNumber = 0;
      filteredProjectDirs = [];

      for (const projectDir of projectDirs) {
        if (projectDir.path.toLowerCase().includes(search.toLowerCase())) {
          totalSize += projectDir.full_build_size;
          filteredProjectsNumber += 1;
          filteredProjectDirs.push(projectDir)
        }
      }
    }
  }

  $: totalPages = Math.ceil(filteredProjectsNumber / elementsPerPage);
  $: currentPage = currentPageClone;
</script>

<div>
  <ProjectChoose bind:selectedDir={selectedDir} bind:config={config} bind:projectDirs={projectDirs} bind:projectCache={data.projectCache} bind:currentPage={currentPageClone}/>
  <div class="m-auto mb-3 w-max">
    <div class="inline-block">
      <Input type="text" placeholder="Search name of the project..." class="max-w-xs" bind:value={search} />
    </div>
    <Button variant="default" on:click={selectDirDialog} class="inline-block">Select</Button>
  </div>

  <Table.Root class="w-max max-w-[60%] m-auto">
    <Table.Caption>
      <Separator class="mb-3"/>
      Number of projects: {filteredProjectsNumber} of total size: {formatBytes(totalSize)}
    </Table.Caption>
    <Table.Header>
      <Table.Row>
        <Table.Head class="w-[200px]">Name</Table.Head>
        <Table.Head class="w-[200px]">Total Size</Table.Head>
        <Table.Head class="w-[400px]">Build Dirs</Table.Head>
        <Table.Head class="text-right">Actions</Table.Head>
      </Table.Row>
    </Table.Header>
    <Table.Body>
      {#each filteredProjectDirs as projectDir, index}
        {#if projectDir.path.toLowerCase().includes(search.toLowerCase()) && index >= (currentPage - 1) * elementsPerPage && index < currentPage * elementsPerPage}
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
              <ManageDialog {projectDir}/>
            </Table.Cell>
          </Table.Row>
        {/if}
      {/each}
    </Table.Body>
  </Table.Root>
  <Pagination bind:currentPage={currentPageClone} {totalPages}/>
</div>
