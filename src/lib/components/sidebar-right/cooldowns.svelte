<script lang="ts">
  import { userData } from "@/stores/user";
  import * as Collapsible from "$lib/components/ui/collapsible/index.js";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import { formatDuration } from "@/utils";
</script>

<Collapsible.Root open={true} class="group/collapsible">
  <Sidebar.GroupLabel
    class="group/label text-sidebar-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground w-full text-sm"
  >
    {#snippet child({ props })}
      <Collapsible.Trigger {...props}>
        Cooldowns
        <ChevronRightIcon
          class="ml-auto transition-transform group-data-[state=open]/collapsible:rotate-90"
        />
      </Collapsible.Trigger>
    {/snippet}
  </Sidebar.GroupLabel>
  <Collapsible.Content>
    <Sidebar.GroupContent class="p-2 text-sm">
      <Sidebar.Menu>
        <Sidebar.MenuItem>
          {formatDuration($userData.cooldowns.drug)}
        </Sidebar.MenuItem>
        <Sidebar.MenuItem>
          {formatDuration($userData.cooldowns.medical)}
        </Sidebar.MenuItem>
        <Sidebar.MenuItem>
          {formatDuration($userData.cooldowns.booster)}
        </Sidebar.MenuItem>
      </Sidebar.Menu>
    </Sidebar.GroupContent>
  </Collapsible.Content>
</Collapsible.Root>
