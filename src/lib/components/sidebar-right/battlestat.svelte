<script lang="ts">
  import * as Card from "@/components/ui/card/index.js";
  import * as Collapsible from "@/components/ui/collapsible/index.js";
  import * as Sidebar from "@/components/ui/sidebar/index.js";
  import * as Table from "@/components/ui/table/index.js";
  import * as Tooltip from "@/components/ui/tooltip/index.js";

  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";

  import { userData } from "@/stores/user";
  import type { Battlestat } from "@/types/user";
  import { cn } from "@/utils";
  import { usePersistedOpen } from "@/hooks/use-persisted-open.svelte";

  const UNITS = [
    { threshold: 1_000_000_000_000_000, suffix: "Qa" },
    { threshold: 1_000_000_000_000, suffix: "T" },
  ];

  const battlestatsCollapsible = usePersistedOpen("sidebar:battlestats:open");

  function formatStat(value: number): string {
    const unit = UNITS.find((u) => value >= u.threshold);
    if (unit) return (value / unit.threshold).toFixed(2) + " " + unit.suffix;
    return value.toLocaleString();
  }

  function calculateEffectiveStats(battlestat: Battlestat): number {
    return Math.round(
      battlestat.value + (battlestat.modifier / 100) * battlestat.value,
    );
  }

  const battlestats = $derived(
    (["strength", "defense", "speed", "dexterity"] as const).map((key) => {
      const stat = $userData.battlestats[key];
      return {
        label: key.charAt(0).toUpperCase() + key.slice(1),
        value: stat.value,
        modifier: stat.modifier,
        modifiers: stat.modifiers,
        effective: calculateEffectiveStats(stat),
      };
    }),
  );

  const effectiveTotal = $derived(
    battlestats.reduce((sum, stat) => sum + stat.effective, 0),
  );

  const exampleNumberInQuad = 123456789012345678;
</script>

<Collapsible.Root
  open={battlestatsCollapsible.isOpen}
  onOpenChange={battlestatsCollapsible.toggle}
  class="group/collapsible"
>
  <Sidebar.GroupLabel
    class="group/label text-sidebar-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground w-full text-sm"
  >
    {#snippet child({ props })}
      <Collapsible.Trigger {...props}>
        Battle Stats
        <ChevronRightIcon
          class="ml-auto transition-transform group-data-[state=open]/collapsible:rotate-90"
        />
      </Collapsible.Trigger>
    {/snippet}
  </Sidebar.GroupLabel>
  <Collapsible.Content>
    <Sidebar.GroupContent class="py-1">
      <Sidebar.Menu>
        <Sidebar.MenuItem>
          <Card.Root class="py-0">
            <Table.Root>
              <Table.Body class="text-xs">
                {#each battlestats as stat}
                  <Table.Row>
                    <Table.Cell class="max-w-19.5 font-medium">
                      {stat.label}
                    </Table.Cell>
                    <Table.Cell class="text-end">
                      {formatStat(stat.value)}
                    </Table.Cell>
                  </Table.Row>
                {/each}
              </Table.Body>
              <Table.Footer class="text-xs">
                <Table.Row>
                  <Table.Cell>Total</Table.Cell>
                  <Table.Cell class="text-end">
                    {formatStat($userData.battlestats.total)}
                  </Table.Cell>
                </Table.Row>
              </Table.Footer>
            </Table.Root>
          </Card.Root>
        </Sidebar.MenuItem>
      </Sidebar.Menu>
      <Sidebar.MenuItem>
        <p class="pt-3 pb-2 pl-2">Effective Battle Stats</p>
        <Card.Root class="py-0">
          <Table.Root>
            <Table.Body class="text-xs">
              {#each battlestats as stat}
                <Tooltip.Root>
                  <Tooltip.Trigger>
                    {#snippet child({ props })}
                      <Table.Row {...props}>
                        <Table.Cell class="max-w-19.5 font-medium">
                          {stat.label}
                        </Table.Cell>
                        <Table.Cell class="text-end">
                          {formatStat(stat.effective)}
                        </Table.Cell>
                        <Table.Cell
                          class={cn(
                            "text-end",
                            stat.modifier < 0 ? "text-destructive" : "",
                          )}
                        >
                          {stat.modifier}%
                        </Table.Cell>
                      </Table.Row>
                    {/snippet}
                  </Tooltip.Trigger>
                  <Tooltip.Content side="left" class="grid grid-cols-2 gap-0">
                    {#each stat.modifiers as modifier}
                      <p class="font-bold">{modifier.type}</p>
                      <p class="text-end">{modifier.value}%</p>
                    {/each}
                  </Tooltip.Content>
                </Tooltip.Root>
              {/each}
            </Table.Body>
            <Table.Footer class="text-xs">
              <Table.Row>
                <Table.Cell>Total</Table.Cell>
                <Table.Cell class="text-end">
                  {formatStat(effectiveTotal)}
                </Table.Cell>
                <Table.Cell></Table.Cell>
              </Table.Row>
            </Table.Footer>
          </Table.Root>
        </Card.Root>
      </Sidebar.MenuItem>
    </Sidebar.GroupContent>
  </Collapsible.Content>
</Collapsible.Root>
