<script lang="ts">
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import Bars from "./bars.svelte";
  import Cooldowns from "./cooldowns.svelte";
  import Icons from "./icons.svelte";
</script>

<Sidebar.Root
  collapsible="none"
  class="sticky top-8.75 flex h-[calc(100svh-35px)] border-s"
>
  <Sidebar.Header class="pb-0">
    <Icons />
  </Sidebar.Header>
  <Sidebar.Separator class="mx-0" />
  <Sidebar.Header class="gap-2.5">
    <Bars />
  </Sidebar.Header>
  <Sidebar.Separator class="mx-0" />
  <Sidebar.Content>
    <Sidebar.Group>
      <Cooldowns />
    </Sidebar.Group>
  </Sidebar.Content>
</Sidebar.Root>
