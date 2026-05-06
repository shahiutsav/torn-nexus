<script lang="ts">
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import SidebarLeft from "@/components/sidebar-left.svelte";
  import SidebarRight from "@/components/sidebar-right.svelte";
  const { children } = $props();
</script>

<div>
  <Sidebar.Provider>
    <SidebarLeft />
    <Sidebar.Inset>
      {@render children()}
    </Sidebar.Inset>
    <SidebarRight />
  </Sidebar.Provider>
</div>
