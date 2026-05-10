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

  type RegularBar = {
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

  let ticks = $state({
    energy: $userData.bars.energy.tick_time,
    nerve: $userData.bars.nerve.tick_time,
    life: $userData.bars.life.tick_time,
    happy: $userData.bars.happy.tick_time,
    chain: $userData.bars.chain.timeout,
  });

  $effect(() => {
    const data = $userData;
    const offset = Math.floor(Date.now() / 1000) - data.timestamp;

    const cleanups = [
      startCountdown(
        data.bars.energy.tick_time - offset,
        (r) => (ticks.energy = r),
        data.bars.energy.interval,
      ),
      startCountdown(
        data.bars.nerve.tick_time - offset,
        (r) => (ticks.nerve = r),
        data.bars.nerve.interval,
      ),
      startCountdown(
        data.bars.life.tick_time - offset,
        (r) => (ticks.life = r),
        data.bars.life.interval,
      ),
      startCountdown(
        data.bars.happy.tick_time - offset,
        (r) => (ticks.happy = r),
        data.bars.happy.interval,
      ),
      // Chain uses a timeout countdown, not a regen interval.
      startCountdown(
        data.bars.chain.timeout - offset,
        (r) => (ticks.chain = r),
      ),
    ];

    return () => cleanups.forEach((cleanup) => cleanup());
  });

  const bars: Bar[] = $derived([
    {
      label: "Energy",
      current: $userData.bars.energy.current,
      maximum: $userData.bars.energy.maximum,
      increment: $userData.bars.energy.increment,
      interval: $userData.bars.energy.interval,
      full_time:
        $userData.bars.energy.full_time -
        (Math.floor(Date.now() / 1000) - $userData.timestamp),
      tick: ticks.energy,
      barClass: "bg-linear-to-b from-[#6cad2b] to-[#4d7c1e]",
      isChain: false,
    },
    {
      label: "Nerve",
      current: $userData.bars.nerve.current,
      maximum: $userData.bars.nerve.maximum,
      increment: $userData.bars.nerve.increment,
      interval: $userData.bars.nerve.interval,
      full_time:
        $userData.bars.nerve.full_time -
        (Math.floor(Date.now() / 1000) - $userData.timestamp),
      tick: ticks.nerve,
      barClass: "bg-linear-to-b from-[#cc7032] to-[#b3382c]",
      isChain: false,
    },
    {
      label: "Happy",
      current: $userData.bars.happy.current,
      maximum: $userData.bars.happy.maximum,
      increment: $userData.bars.happy.increment,
      interval: $userData.bars.happy.interval,
      full_time:
        $userData.bars.happy.full_time -
        (Math.floor(Date.now() / 1000) - $userData.timestamp),
      tick: ticks.happy,
      barClass: "bg-linear-to-b from-[#cccc32] to-[#b3992c]",
      overflowable: true,
      isChain: false,
    },
    {
      label: "Life",
      current: $userData.bars.life.current,
      maximum: $userData.bars.life.maximum,
      increment: $userData.bars.life.increment,
      interval: $userData.bars.life.interval,
      full_time:
        $userData.bars.life.full_time -
        (Math.floor(Date.now() / 1000) - $userData.timestamp),
      tick: ticks.life,
      barClass: "bg-linear-to-b from-[#708bdb] to-[#3f43cf]",
      isChain: false,
    },
    {
      label: "Chain",
      current: $userData.bars.chain.current,
      maximum: $userData.bars.chain.max,
      tick: ticks.chain,
      barClass: "bg-linear-to-b from-[#878787] to-[#6b6b6b]",
      isChain: true,
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
        <div class="text-sm last:min-h-9" {...props}>
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
        {:else if bar.current > 10}
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
            {bar.label === "Happy"
              ? "You have over full happiness"
              : `You have over full ${bar.label.toLowerCase()}`}
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
