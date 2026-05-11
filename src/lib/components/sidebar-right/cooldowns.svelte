<script lang="ts">
  import * as Sidebar from "@/components/ui/sidebar/index.js";
  import * as Item from "@/components/ui/item/index.js";
  import * as Tooltip from "@/components/ui/tooltip/index.js";

  import { userData } from "@/stores/user";
  import { formatCountdownHHMMSS, startCountdown } from "@/utils";

  import {
    COOLDOWN_CONFIG,
    type CooldownKey,
  } from "@/types/sidebar-right-config";

  let countdowns = $state<Record<CooldownKey, number>>({
    drug: $userData.cooldowns.drug,
    medical: $userData.cooldowns.medical,
    booster: $userData.cooldowns.booster,
  });

  $effect(() => {
    const offset = Math.floor(Date.now() / 1000) - $userData.timestamp;
    const cleanups = COOLDOWN_CONFIG.map(({ key }) =>
      startCountdown(
        $userData.cooldowns[key] - offset,
        (r) => (countdowns[key] = r),
      ),
    );
    return () => cleanups.forEach((c) => c());
  });

  const cooldowns = $derived(
    COOLDOWN_CONFIG.map((config) => ({
      ...config,
      icon: $userData.icons.find((icon) =>
        icon.title.includes(`${config.label} Cooldown`),
      ),
      countdown: countdowns[config.key],
    })),
  );
</script>

<Sidebar.Menu>
  {#each cooldowns as cooldown}
    <Tooltip.Root>
      <Tooltip.Trigger>
        {#snippet child({ props })}
          <Sidebar.MenuItem {...props}>
            <Item.Root variant="muted" class="p-2.5">
              {#if cooldown.icon}
                <Item.Media variant="icon">
                  <div
                    style="
                            width: 16px;
                            height: 16px;
                            background-image: url('https://www.torn.com/images/v2/svg_icons/sprites/user_status_icons_sprite.svg?v=1761056520');
                            background-position: -{(cooldown.icon.id - 1) *
                      18}px;
                            background-repeat:none;
                          "
                  ></div>
                </Item.Media>
                <Item.Content>
                  <Item.Title class="font-mono font-bold">
                    {formatCountdownHHMMSS(cooldown.countdown)}
                  </Item.Title>
                  <Item.Description class="font-sans text-xs">
                    {new Date(cooldown.icon.until! * 1000).toLocaleString()}
                  </Item.Description>
                </Item.Content>
              {:else}
                <Item.Media>
                  <img
                    src={cooldown.noIconSrc}
                    alt="No {cooldown.label} Cooldown"
                  />
                </Item.Media>
                <Item.Content>
                  <Item.Title class="text-red-400">
                    No {cooldown.label} Cooldown
                  </Item.Title>
                </Item.Content>
              {/if}
            </Item.Root>
          </Sidebar.MenuItem>
        {/snippet}
      </Tooltip.Trigger>
      <Tooltip.Content side="left" class="flex-col gap-0">
        <p class="font-bold">{cooldown.icon?.title}</p>
        <p>
          {#if cooldown.label == "Drug"}
            {cooldown.icon?.description}
          {:else}
            <span class="font-mono"
              >{formatCountdownHHMMSS(cooldown.countdown)}/{cooldown.icon
                ?.description}
            </span>
          {/if}
        </p>
      </Tooltip.Content>
    </Tooltip.Root>
  {/each}
</Sidebar.Menu>
