<script lang="ts">
  import * as Item from "$lib/components/ui/item/index.js";
  import * as Avatar from "$lib/components/ui/avatar/index.js";
  import { Button } from "@/components/ui/button/index.js";
  import Settings from "@lucide/svelte/icons/settings";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";

  const { children } = $props();
</script>

<div>
  <Sidebar.Provider>
    <div>
      <Sidebar.Root>
        <Sidebar.Header />
        <Sidebar.Content>
          <Sidebar.Group />
          <Sidebar.Group />
        </Sidebar.Content>
        <Sidebar.Footer>
          <Item.Root variant={"outline"} class="px-2 py-2">
            <Item.Media>
              <Avatar.Root class="size-10">
                <Avatar.Image src="should be replaced by source" />
                <Avatar.Fallback>Profile Image</Avatar.Fallback>
              </Avatar.Root>
            </Item.Media>
            <Item.Content>
              <Item.Title>
                <p class="max-w-29 truncate">ingine</p>
              </Item.Title>
              <Item.Description>
                <div class="flex items-center gap-1">
                  <div
                    class="inline-block h-3 w-3 rounded-full bg-green-500"
                  ></div>
                  <span>Traveling</span>
                </div>
              </Item.Description>
            </Item.Content>
            <Item.Actions>
              <Button
                size="icon"
                variant="outline"
                // class="rounded-full"
                aria-label="Settings"
              >
                <Settings />
              </Button>
            </Item.Actions>
          </Item.Root>
        </Sidebar.Footer>
      </Sidebar.Root>
    </div>
    <Sidebar.Inset>
      {@render children()}
    </Sidebar.Inset>
  </Sidebar.Provider>
</div>
