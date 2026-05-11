<script lang="ts">
  import * as Sidebar from "@/components/ui/sidebar/index.js";
  import * as Item from "@/components/ui/item/index.js";
  import * as Tooltip from "@/components/ui/tooltip/index.js";

  import { userData } from "@/stores/user";
  import { formatCountdownHHMMSS, spriteStyle, startCountdown } from "@/utils";

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
    COOLDOWN_CONFIG.map((config) => {
      const icon = $userData.icons.find((i) =>
        i.title.includes(`${config.label} Cooldown`),
      );
      return {
        ...config,
        hasIcon: !!icon,
        spriteStyle: icon ? spriteStyle(icon.id) : null,
        title: icon?.title ?? `No ${config.label} Cooldown`,
        description: icon
          ? new Date(icon.until! * 1000).toLocaleString()
          : null,
        tooltipDescription: icon
          ? config.showDescription
            ? icon.description
            : null
          : null,
        tooltipTitle: `${config.label} Cooldown`,
        countdown: countdowns[config.key],
      };
    }),
  );
</script>

<Sidebar.Menu>
  {#each cooldowns as cooldown}
    <Tooltip.Root>
      <Tooltip.Trigger>
        {#snippet child({ props })}
          <Sidebar.MenuItem {...props}>
            <Item.Root variant="muted" class="p-2.5">
              <Item.Media variant="icon">
                {#if cooldown.hasIcon}
                  <div style={cooldown.spriteStyle}></div>
                {:else}
                  <img
                    src={cooldown.noIconSrc}
                    alt="No {cooldown.label} Cooldown"
                  />
                {/if}
              </Item.Media>
              <Item.Content>
                <Item.Title>
                  {#if cooldown.countdown <= 0}
                    <span class="text-destructive"
                      >No {cooldown.label} Cooldown</span
                    >
                  {:else}
                    <span class="font-mono"
                      >{formatCountdownHHMMSS(cooldown.countdown)}</span
                    >
                  {/if}
                </Item.Title>
                {#if cooldown.description}
                  <Item.Description class="font-sans text-xs">
                    {cooldown.description}
                  </Item.Description>
                {/if}
              </Item.Content>
            </Item.Root>
          </Sidebar.MenuItem>
        {/snippet}
      </Tooltip.Trigger>
      <Tooltip.Content side="left" class="flex-col gap-0">
        <p class="font-bold">{cooldown.tooltipTitle}</p>
        {#if cooldown.tooltipDescription}
          <p>{cooldown.tooltipDescription}</p>
        {/if}
      </Tooltip.Content>
    </Tooltip.Root>
  {/each}
</Sidebar.Menu>
