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
  import { Skeleton } from "$lib/components/ui/skeleton";

  // utils
  import { formatBytes, sortByType } from "$lib/utils";

  // tauri functions
  import { invoke } from "@tauri-apps/api/tauri";
  import { emit, listen } from '@tauri-apps/api/event';
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
  let selectedDirClone: string | null;
  let totalSize: number = 0;
  let filteredProjectsNumber: number = 0;
  let currentPage: number = 1;
  let currentPageClone: number = currentPage;
  let elementsPerPage: number = 10;

  let loadingMessage: string = "";
  let isLoading: boolean = false;
  let duration: string = "0s";

  let search: string = "";

  listen('search-files', (event) => {
    let payload: { message: string } = event.payload as any;
    loadingMessage = payload.message;
  });

  listen('loading-status', (event) => {
    let payload: { status: boolean, duration: { nanos: number, secs: number } } = event.payload as any;
    isLoading = payload.status;
    duration = (payload.duration.secs + (payload.duration.nanos/1_000_000_000)).toFixed(2) + "s";
  });

  async function selectDirDialog() {
    selectedDir = await open({
      directory: true,
      multiple: false,
      // to consider
      // defaultPath: "/home/"
    }) as string;

    try {
      projectDirs = await invoke("find_projects", { projectDir: selectedDir })
      sortByType(config, projectDirs);
      projectCache.set(selectedDir as string, projectDirs!);
      await writeTextFile('projects.json', JSON.stringify(Object.fromEntries(projectCache)), { dir: BaseDirectory.AppConfig });
    } catch(error: any) {
      console.error(error)
      if (error.hasOwnProperty("Cancel")) {
        isLoading = false;
      }
    }
  }

  async function cancelFn() {
    await emit("cancel-fn");
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

  // variables are cloned because on update there is no
  // additional logic being executed when not needed
  $: currentPage = currentPageClone;
  $: selectedDir = selectedDirClone;
</script>

<div>
  <Button variant="default" on:click={cancelFn} class="inline-block">Cancel Function</Button>
  <ProjectChoose 
    bind:selectedDir={selectedDirClone}
    bind:config={config}
    bind:projectDirs={projectDirs}
    bind:projectCache={data.projectCache}
    bind:currentPage={currentPageClone}
    {isLoading}
  />
  <div class="m-auto mb-3 w-max">
    <div class="inline-block">
      <Input type="text" placeholder="Search name of the project..." class="max-w-xs" on:input={() => currentPageClone = 1} bind:value={search} />
    </div>
    {#if !isLoading}
      <Button variant="default" on:click={selectDirDialog} class="inline-block">Select Directory</Button>
    {:else}
      <Button variant="default" class="inline-block" disabled>Select Directory</Button>
    {/if}
  </div>

  {#if isLoading}
    <div class="w-max m-auto flex items-center justify-center">
      <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-2 h-4 w-4 animate-spin">
        <path d="M21 12a9 9 0 1 1-6.219-8.56"></path>
      </svg>
      <p>{loadingMessage}</p>
    </div>
  {/if}

  <Table.Root class="w-max max-w-[60%] m-auto">
    <Table.Caption>
      <Separator class="mb-3"/>
      Number of projects: {filteredProjectsNumber} of total size: {formatBytes(totalSize)} took {duration}
    </Table.Caption>
    <Table.Header>
      <Table.Row>
        <Table.Head class="w-[200px]">Name</Table.Head>
        <Table.Head class="w-[200px]">Total Size</Table.Head>
        <Table.Head class="w-[400px]">Build Dirs</Table.Head>
        <Table.Head class="w-[200px]">Language</Table.Head>
        <Table.Head class="text-right">Actions</Table.Head>
      </Table.Row>
    </Table.Header>
    <Table.Body>
      {#if !isLoading}
        {#each filteredProjectDirs as projectDir, index}
          {#if projectDir.path.toLowerCase().includes(search.toLowerCase()) && index >= (currentPage - 1) * elementsPerPage && index < currentPage * elementsPerPage}
            <Table.Row>
              <!-- Name -->
              <Table.Cell class="font-medium">

                <Tooltip.Root>
                  <Tooltip.Trigger>{projectDir.name}</Tooltip.Trigger>
                  <Tooltip.Content>
                    <p>{projectDir.path}</p>
                  </Tooltip.Content>
                </Tooltip.Root>

              </Table.Cell>
              <!-- Total Size -->
              <Table.Cell>{formatBytes(projectDir.full_build_size)}</Table.Cell>
              <!-- Build Dirs -->
              <Table.Cell>
                {#each projectDir.build_dirs as dir}
                  <p>{dir.dir} - <strong>{formatBytes(dir.size)}</strong></p>
                {/each}
              </Table.Cell>
              <!-- Language -->
              <Table.Cell>{projectDir.language}</Table.Cell>
              <!-- Actions -->
              <Table.Cell class="text-right">
                <ManageDialog {projectDir}/>
              </Table.Cell>
            </Table.Row>
          {/if}
        {/each}
      {:else}
        {#each Array.from({ length: 4 }) as _}
          <Table.Row>
            <!-- Name -->
            <Table.Cell class="font-medium"><Skeleton class="h-5 w-[81px]" /></Table.Cell>
            <!-- Total Size -->
            <Table.Cell><Skeleton class="h-5 w-16" /></Table.Cell>
            <!-- Build Dirs -->
            <Table.Cell>
              <Skeleton class="h-5 w-20" />
              <Skeleton class="h-5 w-28 mt-1" />
            </Table.Cell>
            <!-- Language -->
            <Table.Cell><Skeleton class="h-5 w-[81px]" /></Table.Cell>
            <!-- Actions -->
            <Table.Cell class="text-right">
              <Skeleton class="h-9 w-[81px]" />
            </Table.Cell>
          </Table.Row>
        {/each}
      {/if}
    </Table.Body>
  </Table.Root>
  {#if !isLoading}
    <Pagination bind:currentPage={currentPageClone} {totalPages}/>
  {:else}
    <Pagination currentPage={1} totalPages={1}/>
  {/if}
</div>
