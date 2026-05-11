<script lang="ts">
  import { Progress } from "@/components/ui/progress/index.js";
  import * as Tooltip from "@/components/ui/tooltip/index.js";
  import {
    cn,
    formatCountdownHHMMSS,
    formatCountdownMMSS,
    startCountdown,
  } from "@/utils";
  import { userData } from "@/stores/user";
  import { BAR_CONFIG } from "@/constants/sidebar-right-constants";

  type RegularBar = {
    key: string;
    label: string;
    current: number;
    maximum: number;
    increment: number;
    interval: number;
    full_time: number;
    tick: number;
    barClass: string;
    overflowable?: boolean;
    isChain: false;
  };

  type ChainBar = {
    label: "Chain";
    current: number;
    maximum: number;
    tick: number;
    barClass: string;
    isChain: true;
  };

  type Bar = RegularBar | ChainBar;

  type TickKey = (typeof BAR_CONFIG)[number]["key"] | "chain";

  let ticks = $state<Record<TickKey, number>>({
    ...(Object.fromEntries(
      BAR_CONFIG.map(({ key }) => [key, $userData.bars[key].tick_time]),
    ) as Record<TickKey, number>),
    chain: $userData.bars.chain.timeout,
  });

  $effect(() => {
    const data = $userData;
    const offset = Math.floor(Date.now() / 1000) - data.timestamp;

    const cleanups = [
      ...BAR_CONFIG.map(({ key }) =>
        startCountdown(
          data.bars[key].tick_time - offset,
          (r) => (ticks[key] = r),
          data.bars[key].interval,
        ),
      ),
      startCountdown(
        data.bars.chain.timeout - offset,
        (r) => (ticks.chain = r),
      ),
    ];

    return () => cleanups.forEach((c) => c());
  });

  const bars: Bar[] = $derived([
    ...BAR_CONFIG.map((config) => {
      const bar = $userData.bars[config.key];
      return {
        ...config,
        current: bar.current,
        maximum: bar.maximum,
        increment: bar.increment,
        interval: bar.interval,
        full_time:
          bar.full_time - (Math.floor(Date.now() / 1000) - $userData.timestamp),
        tick: ticks[config.key],
        isChain: false as const,
      };
    }),
    {
      label: "Chain",
      current: $userData.bars.chain.current,
      maximum: $userData.bars.chain.max,
      tick: ticks.chain,
      barClass: "bg-linear-to-b from-[#878787] to-[#6b6b6b]",
      isChain: true as const,
    },
  ]);

  function getBarDisplay(bar: Bar): { text: string; class?: string } {
    if (bar.current === bar.maximum) return { text: "FULL" };

    if (bar.current > bar.maximum) {
      // Only Happy can legitimately overflow; everything else shouldn't tick.
      if (!bar.isChain && bar.overflowable) {
        return { text: formatCountdownMMSS(bar.tick), class: "text-red-400" };
      }
      return { text: "OVER" };
    }

    return { text: formatCountdownMMSS(bar.tick) };
  }
</script>

{#each bars as bar}
  {@const display = getBarDisplay(bar)}
  <Tooltip.Root>
    <Tooltip.Trigger>
      {#snippet child({ props })}
        <div class="text-xs last:min-h-9" {...props}>
          <div>
            <p class="float-left w-13 font-bold">{bar.label}</p>
            <p class="float-left font-mono">: {bar.current}/{bar.maximum}</p>
            <p
              class={cn(
                "text-muted-foreground float-right font-mono",
                display.class ?? "",
              )}
            >
              {display.text}
            </p>
          </div>
          <Progress
            value={Math.min((bar.current / bar.maximum) * 100, 100)}
            class="mt-0.5 h-2"
            barClass={bar.barClass}
          />
        </div>
      {/snippet}
    </Tooltip.Trigger>

    <!--
      Chain has a completely different tooltip (timeout countdown only).
      Regular bars show regen rate + full-time details.
      Keeping these in separate #if branches avoids non-null assertions and
      makes the intent clear.
    -->
    <Tooltip.Content side="left" class="flex-col items-start gap-0">
      {#if bar.isChain}
        {#if bar.current > 0 && bar.current < 10}
          <p class="font-bold">Chain warm-up</p>
          <p>
            Make {10 - bar.current} more hits within {formatCountdownMMSS(
              bar.tick,
            )} to start a chain
          </p>
        {:else if bar.current >= 10}
          <p>TBD</p>
        {:else}
          <p class="font-bold">Chain inactive</p>
          <p>Make 10 hits within 5 minutes to start a chain</p>
        {/if}
      {:else}
        <p class="font-bold">
          {bar.label} increases by {bar.increment} every {bar.interval / 60} minutes
        </p>
        {#if bar.current === bar.maximum}
          <p>You have full <span class="lowercase">{bar.label}</span></p>
        {:else if bar.current > bar.maximum}
          <p>
            You have over full
            {bar.overflowable ? "happiness" : bar.key}
          </p>
        {:else}
          <p>
            Full <span class="lowercase">{bar.label}</span> in {formatCountdownHHMMSS(
              bar.full_time,
            )}
          </p>
          <p>{new Date(Date.now() + bar.full_time * 1000).toLocaleString()}</p>
        {/if}
      {/if}
    </Tooltip.Content>
  </Tooltip.Root>
{/each}
