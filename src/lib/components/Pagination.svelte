<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";

  export let currentPage: number;
  export let totalPages: number;
</script>

<div class="m-auto w-max mt-3">
  {#if currentPage > 1}
    <Button on:click={() => currentPage -= 1}>&lt;</Button>
    <Button variant="secondary" on:click={() => currentPage = 1}>1</Button>
  {:else}
    <Button disabled>&lt;</Button>
    <Button variant="secondary" disabled>1</Button>
  {/if}


  {#if currentPage > 3}
    <Button variant="ghost" disabled>...</Button>
  {/if}

  {#each Array.from({ length: totalPages }) as _, index}
    {#if index >= currentPage - 3 && index <= currentPage + 1 && index + 1 != 1 && index + 1 != totalPages}
      {#if index + 1 === currentPage}
        <Button variant="secondary" disabled on:click={() => currentPage = index + 1}>{index + 1}</Button>
      {:else}
        <Button variant="secondary" on:click={() => currentPage = index + 1}>{index + 1}</Button>
      {/if} 
    {/if}
  {/each}

  {#if currentPage < totalPages - 1}
    <Button variant="ghost" disabled>...</Button>
  {/if}

  {#if currentPage < totalPages}
    <Button variant="secondary" on:click={() => currentPage = totalPages}>{totalPages}</Button>
    <Button on:click={() => currentPage += 1}>&gt;</Button>
  {:else if totalPages == 0 || totalPages == 1}
    <Button disabled>&gt;</Button>
  {:else}
    <Button variant="secondary" disabled>{totalPages}</Button>
    <Button disabled>&gt;</Button>
  {/if}

</div>
