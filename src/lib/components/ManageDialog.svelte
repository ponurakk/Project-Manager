<script lang="ts">
  import { Button } from "./ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import { formatBytes } from "$lib/utils";

  export let projectDir: import("$lib/types").Project
</script>

<Dialog.Root>
  <Dialog.Trigger>
    <Button variant="default">Manage</Button>
  </Dialog.Trigger>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>{projectDir.name}</Dialog.Title>
      <Dialog.Description>
        <div class="text-center">
          {#each projectDir.build_dirs as dir}
            <p>{dir.dir} - <strong>{formatBytes(dir.size)}</strong></p>
          {/each}
        </div>
      </Dialog.Description>
    </Dialog.Header>
  </Dialog.Content>
</Dialog.Root>
