<script lang="ts">
  import * as Sidebar from "@/components/ui/sidebar/index.js";
  import SidebarLeft from "@/components/sidebar-left/sidebar-left.svelte";
  import SidebarRight from "@/components/sidebar-right/sidebar-right.svelte";
  import { userData } from "@/stores/user";
  import LoaderCircle from "@lucide/svelte/icons/loader-circle";

  const { children } = $props();
</script>

<div>
  {#if $userData}
    <Sidebar.Provider>
      <SidebarLeft />
      <Sidebar.Inset>
        {@render children()}
      </Sidebar.Inset>
      <SidebarRight />
    </Sidebar.Provider>
  {:else}
    <div class="h-full w-full">
      <LoaderCircle
        class="absolute top-1/2 right-1/2 -translate-x-1/2 -translate-y-1/2 animate-spin"
      />
    </div>
  {/if}
</div>
