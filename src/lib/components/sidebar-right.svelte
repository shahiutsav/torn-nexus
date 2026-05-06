<script lang="ts">
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
</script>

<Sidebar.Root
  collapsible="none"
  class="sticky top-8.75 flex h-[calc(100svh-35px)] border-s"
>
  <Sidebar.Header class="border-sidebar-border"></Sidebar.Header>
  <Sidebar.Content></Sidebar.Content>
  <Sidebar.Footer>
    <Sidebar.Menu>
      <Sidebar.MenuItem>
        <Sidebar.MenuButton>
          <span>New Calendar</span>
        </Sidebar.MenuButton>
      </Sidebar.MenuItem>
    </Sidebar.Menu>
  </Sidebar.Footer>
</Sidebar.Root>
