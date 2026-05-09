<script lang="ts">
  import { userData } from "@/stores/user";
  import * as Collapsible from "$lib/components/ui/collapsible/index.js";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import * as Item from "../ui/item/index";
  import type { Icon } from "@/types/user";
  import {
    MISC_ICON_TITLES,
    type MiscIconTitle,
  } from "@/types/sidebar-right-config";

  const miscellaneous = $derived(
    MISC_ICON_TITLES.map((title) =>
      $userData.icons.find((icon) => icon.title.includes(title)),
    ).filter(Boolean),
  );

  const descriptionFormatters: Partial<
    Record<MiscIconTitle, (icon: Icon) => string>
  > = {
    "Bank Investment": (icon) =>
      icon.description?.replace("Current bank investment worth ", "") ?? "",
    "Reading Book": (icon) => icon.description?.split("<br>")[0] ?? "",
    Education: (icon) =>
      icon.description?.replace("Currently completing the ", "") ?? "",
    Subscriber: (icon) =>
      icon.description?.split("<br>")[1]?.split(":")[1] ?? "",
    "Organized Crime": (icon) =>
      icon.description?.split("<br>")[0]?.split(" in ")[1] ?? "",
  };

  const formatDescription = (icon: Icon): string =>
    descriptionFormatters[icon.title as MiscIconTitle]?.(icon) ??
    icon.description ??
    "";
</script>

<Collapsible.Root open={true} class="group/collapsible">
  <Sidebar.GroupLabel
    class="group/label text-sidebar-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground w-full text-sm"
  >
    {#snippet child({ props })}
      <Collapsible.Trigger {...props}>
        Miscellaneous
        <ChevronRightIcon
          class="ml-auto transition-transform group-data-[state=open]/collapsible:rotate-90"
        />
      </Collapsible.Trigger>
    {/snippet}
  </Sidebar.GroupLabel>
  <Collapsible.Content>
    <Sidebar.GroupContent class="py-1">
      <Sidebar.Menu>
        {#each miscellaneous as misc}
          <Sidebar.MenuItem>
            <Item.Root variant="muted" class="gap-2 p-2.5">
              {#if misc}
                {@const isOC = misc.title === "Organized Crime"}
                {@const isSubscriber =
                  misc.title === "Subscriber" || misc.title === "Donator"}
                {@const ocIncomplete = isOC && misc.until == null}
                <Item.Media variant="icon">
                  <div
                    style="
                      width: 16px;
                      height: 16px;
                      background-image: url('https://www.torn.com/images/v2/svg_icons/sprites/user_status_icons_sprite.svg?v=1761056520');
                      background-position: -{(misc.id - 1) * 18}px;
                      background-repeat:none;
                    "
                  ></div>
                </Item.Media>
                <Item.Content>
                  <Item.Title>
                    {formatDescription(misc)}
                  </Item.Title>
                  {#if !isSubscriber}
                    <Item.Description class="font-sans text-xs">
                      {#if isOC}
                        <p>
                          {misc.description?.split("<br>")[0].split(" in ")[0]}
                          · {misc.description?.split("<br>")[1]}
                        </p>
                      {/if}

                      {#if ocIncomplete}
                        <p class="text-red-300/70">Incomplete</p>
                      {:else}
                        <p>{new Date(misc.until! * 1000).toLocaleString()}</p>
                      {/if}
                    </Item.Description>
                  {/if}
                </Item.Content>
              {/if}
            </Item.Root>
          </Sidebar.MenuItem>
        {/each}
      </Sidebar.Menu>
    </Sidebar.GroupContent>
  </Collapsible.Content>
</Collapsible.Root>
