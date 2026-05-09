<script lang="ts">
  import { Progress } from "$lib/components/ui/progress/index.js";
  import { cn, formatCountdownMMSS, startCountdown } from "@/utils";
  import { userData } from "@/stores/user";
  import type { BarConfig } from "@/types/sidebar-right-config";

  let energyTick = $state($userData.bars.energy.tick_time);
  let nerveTick = $state($userData.bars.nerve.tick_time);
  let lifeTick = $state($userData.bars.life.tick_time);
  let happyTick = $state($userData.bars.happy.tick_time);
  let chainTick = $state($userData.bars.chain.timeout);

  $effect(() => {
    const offset = Math.floor(Date.now() / 1000) - $userData.timestamp;
    const cleanups = [
      startCountdown(
        $userData.bars.energy.tick_time - offset,
        (r) => (energyTick = r),
        $userData.bars.energy.interval,
      ),
      startCountdown(
        $userData.bars.nerve.tick_time - offset,
        (r) => (nerveTick = r),
        $userData.bars.nerve.interval,
      ),
      startCountdown(
        $userData.bars.life.tick_time - offset,
        (r) => (lifeTick = r),
        $userData.bars.life.interval,
      ),
      startCountdown(
        $userData.bars.happy.tick_time - offset,
        (r) => (happyTick = r),
        $userData.bars.happy.interval,
      ),
      startCountdown(
        $userData.bars.chain.timeout - offset,
        (r) => (chainTick = r),
      ),
    ];

    return () => cleanups.forEach((cleanup) => cleanup());
  });

  const bars: BarConfig[] = $derived([
    {
      label: "Energy",
      current: $userData.bars.energy.current,
      maximum: $userData.bars.energy.maximum,
      tick: energyTick,
      barClass: "bg-linear-to-b from-[#6cad2b] to-[#4d7c1e]",
    },
    {
      label: "Nerve",
      current: $userData.bars.nerve.current,
      maximum: $userData.bars.nerve.maximum,
      tick: nerveTick,
      barClass: "bg-linear-to-b from-[#cc7032] to-[#b3382c]",
    },
    {
      label: "Happy",
      current: $userData.bars.happy.current,
      maximum: $userData.bars.happy.maximum,
      tick: happyTick,
      barClass: "bg-linear-to-b from-[#cccc32] to-[#b3992c]",
      overflowable: true,
    },
    {
      label: "Life",
      current: $userData.bars.life.current,
      maximum: $userData.bars.life.maximum,
      tick: lifeTick,
      barClass: "bg-linear-to-b from-[#708bdb] to-[#3f43cf]",
    },
    {
      label: "Chain",
      current: $userData.bars.chain.current,
      maximum: $userData.bars.chain.max,
      tick: chainTick,
      barClass: "bg-linear-to-b from-[#878787] to-[#6b6b6b]",
    },
  ]);

  function getBarDisplay(bar: BarConfig): { text: string; class?: string } {
    if (bar.current === bar.maximum) return { text: "FULL" };
    if (bar.current > bar.maximum) {
      return bar.overflowable
        ? { text: formatCountdownMMSS(bar.tick), class: "text-red-400" }
        : { text: "OVER" };
    }
    return { text: formatCountdownMMSS(bar.tick) };
  }
</script>

{#each bars as bar}
  {@const display = getBarDisplay(bar)}
  <div class="text-sm">
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
      value={(bar.current / bar.maximum) * 100}
      class="mt-0.5 h-2"
      barClass={bar.barClass}
    />
  </div>
{/each}
