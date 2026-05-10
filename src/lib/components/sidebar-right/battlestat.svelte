<script>
  import * as Card from "@/components/ui/card/index.js";
  import * as Collapsible from "@/components/ui/collapsible/index.js";
  import * as Sidebar from "@/components/ui/sidebar/index.js";
  import * as Table from "$lib/components/ui/table/index.js";

  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";

  import { userData } from "@/stores/user";

  const exampleNumberInQuad = 123456789012345678;
</script>

<Collapsible.Root open={true} class="group/collapsible">
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
                <Table.Row>
                  <Table.Cell class="max-w-19.5 font-medium"
                    >Strength</Table.Cell
                  >
                  <Table.Cell class="text-end">
                    {$userData.battlestats.strength.value.toLocaleString()}
                  </Table.Cell>
                </Table.Row>
                <Table.Row>
                  <Table.Cell class="font-medium">Defense</Table.Cell>
                  <Table.Cell class="text-end"
                    >{$userData.battlestats.defense.value.toLocaleString()}</Table.Cell
                  >
                </Table.Row>
                <Table.Row>
                  <Table.Cell class="font-medium">Speed</Table.Cell>
                  <Table.Cell class="text-end"
                    >{$userData.battlestats.speed.value.toLocaleString()}</Table.Cell
                  >
                </Table.Row>
                <Table.Row>
                  <Table.Cell class="font-medium">Dexterity</Table.Cell>
                  <Table.Cell class="text-end"
                    >{$userData.battlestats.dexterity.value.toLocaleString()}</Table.Cell
                  >
                </Table.Row>
              </Table.Body>
              <Table.Footer class="text-xs">
                <Table.Row>
                  <Table.Cell>Total</Table.Cell>
                  <Table.Cell class="text-end"
                    >{$userData.battlestats.total.toLocaleString()}</Table.Cell
                  >
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
              <Table.Row>
                <Table.Cell class="max-w-19.5 font-medium">Strength</Table.Cell>
                <Table.Cell class="text-end">
                  {Math.round(
                    $userData.battlestats.strength.value +
                      ($userData.battlestats.strength.modifier / 100) *
                        $userData.battlestats.strength.value,
                  ).toLocaleString()}
                </Table.Cell>
                <Table.Cell class="text-end">
                  {$userData.battlestats.strength.modifier}%
                </Table.Cell>
              </Table.Row>
              <Table.Row>
                <Table.Cell class="max-w-19.5 font-medium">Defense</Table.Cell>
                <Table.Cell class="text-end">
                  {Math.round(
                    $userData.battlestats.defense.value +
                      ($userData.battlestats.defense.modifier / 100) *
                        $userData.battlestats.defense.value,
                  ).toLocaleString()}
                </Table.Cell>
                <Table.Cell class="text-end">
                  {$userData.battlestats.defense.modifier}%
                </Table.Cell>
              </Table.Row>
              <Table.Row>
                <Table.Cell class="max-w-19.5 font-medium">Speed</Table.Cell>
                <Table.Cell class="text-end">
                  {Math.round(
                    $userData.battlestats.speed.value +
                      ($userData.battlestats.speed.modifier / 100) *
                        $userData.battlestats.speed.value,
                  ).toLocaleString()}
                </Table.Cell>
                <Table.Cell class="text-end">
                  {$userData.battlestats.speed.modifier}%
                </Table.Cell>
              </Table.Row>
              <Table.Row>
                <Table.Cell class="max-w-19.5 font-medium">Dexterity</Table.Cell
                >
                <Table.Cell class="text-end">
                  {Math.round(
                    $userData.battlestats.dexterity.value +
                      ($userData.battlestats.dexterity.modifier / 100) *
                        $userData.battlestats.dexterity.value,
                  ).toLocaleString()}
                </Table.Cell>
                <Table.Cell class="text-end">
                  {$userData.battlestats.dexterity.modifier}%
                </Table.Cell>
              </Table.Row>
            </Table.Body>
            <Table.Footer class="text-xs">
              <Table.Row>
                <Table.Cell>Total</Table.Cell>
                <Table.Cell class="text-end"
                  >{$userData.battlestats.total.toLocaleString()}</Table.Cell
                >
                <Table.Cell></Table.Cell>
              </Table.Row>
            </Table.Footer>
          </Table.Root>
        </Card.Root>
      </Sidebar.MenuItem>
    </Sidebar.GroupContent>
  </Collapsible.Content>
</Collapsible.Root>
