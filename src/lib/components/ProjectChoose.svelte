<script lang="ts">
  import * as Select from "$lib/components/ui/select";
  import { SortType, type Config, SortDirection, type ProjectCache, type Project } from "$lib/types";
  import { selectDirCache, sortByType } from "$lib/utils";

  export let selectedDir: string | null;
  export let config: Config;
  export let projectCache: ProjectCache;
  export let projectDirs: Project[];
  export let currentPage: number;

  $: {
    sortByType(config, projectDirs);
    // this doesn't make any sense but it works so i won't touch it
    projectDirs = projectDirs;
  }
</script>

<div class="m-auto w-max">
  <div class="inline-block">
    <Select.Root bind:value={selectedDir} onValueChange={() => {
      projectDirs = selectDirCache(config, projectDirs, projectCache, selectedDir);
      currentPage = 1;
    }}>
      <Select.Trigger class="w-[300px]">
        <Select.Value placeholder="Choose cached dir..." />
      </Select.Trigger>
      <Select.Content>
        {#each [...projectCache] as [key, _]}
          <Select.Item value={key}>{key}</Select.Item>
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
        <Select.Item value={SortType.ByName}>By Name</Select.Item>
        <Select.Item value={SortType.BySize}>By Size</Select.Item>
      </Select.Content>
    </Select.Root>
  </div>

  <div class="inline-block mb-3">
    <Select.Root bind:value={config.sortDirection}>
      <Select.Trigger class="w-[150px]">
        <Select.Value placeholder="Select Sort Type" />
      </Select.Trigger>
      <Select.Content>
        <Select.Item value={SortDirection.ASC} aria-selected>Ascending</Select.Item>
        <Select.Item value={SortDirection.DESC}>Descending</Select.Item>
      </Select.Content>
    </Select.Root>
  </div>
</div>
