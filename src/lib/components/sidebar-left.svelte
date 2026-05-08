<script lang="ts">
  import * as Item from "@/components/ui/item/index.js";
  import * as Avatar from "@/components/ui/avatar/index.js";
  import { Button } from "@/components/ui/button/index.js";
  import * as Sidebar from "@/components/ui/sidebar/index.js";
  import Settings from "@lucide/svelte/icons/settings";

  import type { ComponentProps } from "svelte";
  import { userData } from "@/stores/user";
  import { cn } from "@/utils";

  let {
    ref = $bindable(null),
    ...restProps
  }: ComponentProps<typeof Sidebar.Root> = $props();

  const avatarUrl = $derived(
    $userData.data.profile.image.replace(
      "https://profileimages.torn.com/",
      "https://avatars.torn.com/48X48_",
    ),
  );

  const statusColors = {
    Okay: "bg-gradient-to-r from-lime-800/10  to-lime-800/40",
    Traveling: "bg-gradient-to-r from-blue-800/10  to-blue-800/40",
  };

  const statusIcons = {
    Online: "/icons/active-status/online.svg",
    Offline: "/icons/active-status/offline.svg",
    Idle: "/icons/active-status/idle.svg",
  };

  const color = $derived(
    statusColors[
      $userData.data.profile.status.state as keyof typeof statusColors
    ] ?? "bg-gradient-to-r from-red-800/10 to-red-800/40",
  );

  const activeStatusIcon = $derived(
    statusIcons[
      $userData.data.profile.last_action.status as keyof typeof statusIcons
    ],
  );
</script>

<Sidebar.Root bind:ref class="border-e-0" {...restProps}>
  <Sidebar.Header />
  <Sidebar.Content>
    <Sidebar.Group />
    <Sidebar.Group />
  </Sidebar.Content>
  <Sidebar.Footer>
    <Item.Root
      variant={"muted"}
      class={cn("border-border bg-clip-padding px-1.5 py-1.5", color)}
    >
      <Item.Media>
        <div class="relative">
          <Avatar.Root class="size-10">
            <Avatar.Image src={avatarUrl} alt={$userData.data.profile.name} />
            <Avatar.Fallback>{$userData.data.profile.name[0]}</Avatar.Fallback>
          </Avatar.Root>
          <div class="bg-muted absolute right-0 bottom-0 size-4.5 rounded-full">
            <div
              class="absolute top-1/2 left-1/2 size-3 -translate-x-1/2 -translate-y-1/2"
            >
              <img src={activeStatusIcon} alt="online indicator" />
            </div>
          </div>
        </div>
      </Item.Media>
      <Item.Content>
        <Item.Title>
          <p class="max-w-29 truncate">ingine</p>
        </Item.Title>
        <Item.Description class="text-xs">
          <div class="flex items-center gap-1">
            <span>{$userData.data.profile.status.state}</span>
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
