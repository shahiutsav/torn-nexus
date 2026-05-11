<script lang="ts">
  import * as Collapsible from "@/components/ui/collapsible/index.js";
  import * as Sidebar from "@/components/ui/sidebar/index.js";
  import { usePersistedOpen } from "@/hooks/use-persisted-open.svelte";

  import Bars from "./bars.svelte";
  import Battlestats from "./battlestats.svelte";
  import Cooldowns from "./cooldowns.svelte";
  import Icons from "./icons.svelte";
  import Miscellaneous from "./miscellaneous.svelte";

  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import Job from "./job.svelte";

  const componentMap = {
    cooldowns: Cooldowns,
    miscellaneous: Miscellaneous,
    battlestats: Battlestats,
    job: Job,
  } as const;

  const sections = (
    ["cooldowns", "miscellaneous", "battlestats", "job"] as const
  ).map((key) => {
    return {
      label: key.charAt(0).toUpperCase() + key.slice(1),
      collapsibleState: usePersistedOpen("sidebar:right:" + key + ":open"),
      component: componentMap[key],
    };
  });
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
  <Sidebar.Content class="pb-2">
    {#each sections as section}
      <Sidebar.Group class="not-first:py-0 first:pb-0">
        <Collapsible.Root
          open={section.collapsibleState.isOpen}
          onOpenChange={section.collapsibleState.toggle}
          class="group/collapsible"
        >
          <Sidebar.GroupLabel
            class="group/label text-sidebar-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground w-full text-sm"
          >
            {#snippet child({ props })}
              <Collapsible.Trigger {...props}>
                {section.label}
                <ChevronRightIcon
                  class="ml-auto transition-transform group-data-[state=open]/collapsible:rotate-90"
                />
              </Collapsible.Trigger>
            {/snippet}
          </Sidebar.GroupLabel>
          <Collapsible.Content>
            <Sidebar.GroupContent class="pt-2 pb-0">
              <section.component />
            </Sidebar.GroupContent>
          </Collapsible.Content>
        </Collapsible.Root>
      </Sidebar.Group>
      <Sidebar.Separator class="mx-0 last:hidden" />
    {/each}
  </Sidebar.Content>
</Sidebar.Root>
