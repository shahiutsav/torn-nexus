<script lang="ts">
  import * as Item from "$lib/components/ui/item/index.js";
  import * as Avatar from "$lib/components/ui/avatar/index.js";
  import { Button } from "@/components/ui/button/index.js";
  import Settings from "@lucide/svelte/icons/settings";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";

  import type { ComponentProps } from "svelte";

  let {
    ref = $bindable(null),
    ...restProps
  }: ComponentProps<typeof Sidebar.Root> = $props();
</script>

<Sidebar.Root bind:ref class="border-e-0" {...restProps}>
  <Sidebar.Header />
  <Sidebar.Content>
    <Sidebar.Group />
    <Sidebar.Group />
  </Sidebar.Content>
  <Sidebar.Footer>
    <Item.Root variant={"muted"} class="px-2 py-2">
      <Item.Media>
        <div class="relative">
          <svg
            width="40"
            height="40"
            viewBox="0 0 40 40"
            xmlns="http://www.w3.org/2000/svg"
          >
            <defs>
              <mask id="svg-mask-avatar">
                <rect width="40" height="40" rx="50%" fill="white" />
                <!-- cut the notch where the dot will go -->
                <circle cx="34" cy="34" r="9" fill="black" />
              </mask>
            </defs>
            <image
              href="https://avatars.torn.com/48X48_a9fd4110-7859-4a12-866c-d4190c2fe257-3255609.png"
              mask="url(#svg-mask-avatar)"
              width="40"
              height="40"
              preserveAspectRatio="xMidYMid slice"
            />
          </svg>
          <svg
            class="absolute right-0 bottom-0"
            width="12"
            height="12"
            viewBox="16 16 16 16"
          >
            <defs>
              <mask id="svg-mask-status-idle">
                <!-- White = visible, Black = cut out -->
                <!-- Full square visible first -->
                <rect
                  width="16"
                  height="16"
                  fill="white"
                  x="16"
                  y="16"
                  rx="8"
                />
                <!-- Cut a chunk out to make the crescent -->
                <circle cx="20" cy="20" r="6" fill="black" />
              </mask>
            </defs>
            <rect
              width="16"
              height="16"
              x="16"
              y="16"
              fill="#ffc04e"
              mask="url(#svg-mask-status-idle)"
            />
          </svg>
        </div>
      </Item.Media>
      <Item.Content>
        <Item.Title>
          <p class="max-w-29 truncate">ingine</p>
        </Item.Title>
        <Item.Description class="text-xs">
          <div class="flex items-center gap-1">
            <div class="inline-block h-3 w-3 rounded bg-green-500"></div>
            <span>Traveling</span>
          </div>
        </Item.Description>
      </Item.Content>
      <Item.Actions>
        <Button
          size="icon"
          variant="ghost"
          // class="rounded-full"
          aria-label="Settings"
        >
          <Settings />
        </Button>
      </Item.Actions>
    </Item.Root>
  </Sidebar.Footer>
</Sidebar.Root>
