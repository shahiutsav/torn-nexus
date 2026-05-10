<script lang="ts">
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import Bars from "./bars.svelte";
  import Battlestat from "./battlestat.svelte";
  import Cooldowns from "./cooldowns.svelte";
  import Icons from "./icons.svelte";
  import Miscellaneous from "./miscellaneous.svelte";
</script>

<Sidebar.Root
  collapsible="none"
  class="sticky top-8.75 flex h-[calc(100svh-35px)] border-s"
>
  <Sidebar.Header class="">
    <Icons />
  </Sidebar.Header>
  <Sidebar.Separator class="mx-0" />
  <Sidebar.Header class="gap-2.5">
    <Bars />
  </Sidebar.Header>
  <Sidebar.Separator class="mx-0" />
  <Sidebar.Content>
    <Sidebar.Group class="pb-0">
      <Cooldowns />
    </Sidebar.Group>
    <Sidebar.Separator class="mx-0" />
    <Sidebar.Group class="py-0">
      <Miscellaneous />
    </Sidebar.Group>
    <Sidebar.Separator class="mx-0" />
    <Sidebar.Group class="py-0">
      <Battlestat />
    </Sidebar.Group>
  </Sidebar.Content>
</Sidebar.Root>
