<script lang="ts">
  import { userData } from "@/stores/user";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import * as Collapsible from "$lib/components/ui/collapsible/index.js";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import { Progress } from "$lib/components/ui/progress/index.js";
  import { formatCountdown, formatDuration, startCountdown } from "@/utils";

  console.log($userData);

  let energyTick = $state($userData.data.bars.energy.tick_time);
  let nerveTick = $state($userData.data.bars.nerve.tick_time);
  let lifeTick = $state($userData.data.bars.life.tick_time);
  let happyTick = $state($userData.data.bars.happy.tick_time);
  let chainTick = $state($userData.data.bars.chain.timeout);

  $effect(() => {
    const offset = Math.floor(Date.now() / 1000) - $userData.data.timestamp;
    const cleanups = [
      startCountdown(
        $userData.data.bars.energy.tick_time - offset,
        (r) => (energyTick = r),
        $userData.data.bars.energy.interval,
      ),
      startCountdown(
        $userData.data.bars.nerve.tick_time - offset,
        (r) => (nerveTick = r),
        $userData.data.bars.nerve.interval,
      ),
      startCountdown(
        $userData.data.bars.life.tick_time - offset,
        (r) => (lifeTick = r),
        $userData.data.bars.life.interval,
      ),
      startCountdown(
        $userData.data.bars.happy.tick_time - offset,
        (r) => (happyTick = r),
        $userData.data.bars.happy.interval,
      ),
      startCountdown(
        $userData.data.bars.chain.timeout - offset,
        (r) => (chainTick = r),
      ),
    ];

    return () => cleanups.forEach((cleanup) => cleanup());
  });
</script>

<Sidebar.Root
  collapsible="none"
  class="sticky top-8.75 flex h-[calc(100svh-35px)] border-s"
>
  <Sidebar.Header class="gap-2.5 text-xs">
    <div>
      <p class="flex justify-between">
        <span>
          <span class="font-bold">Energy</span>: {$userData.data.bars.energy
            .current}/{$userData.data.bars.energy.maximum}
        </span>
        <span class="text-muted-foreground font-mono">
          {formatCountdown(energyTick)}
        </span>
      </p>
      <Progress
        value={($userData.data.bars.energy.current /
          $userData.data.bars.energy.maximum) *
          100}
        class="mt-0 h-2"
        barClass="bg-linear-to-b from-[#6cad2b] to-[#4d7c1e]"
      />
    </div>
    <div>
      <p class="flex justify-between">
        <span>
          <span class="font-bold">Nerve</span>: {$userData.data.bars.nerve
            .current}/{$userData.data.bars.nerve.maximum}
        </span>
        <span class="text-muted-foreground font-mono">
          {formatCountdown(nerveTick)}
        </span>
      </p>
      <Progress
        value={($userData.data.bars.nerve.current /
          $userData.data.bars.nerve.maximum) *
          100}
        class="mt-0 h-2"
        barClass="bg-linear-to-b from-[#cc7032] to-[#b3382c]"
      />
    </div>
    <div>
      <p class="flex justify-between">
        <span>
          <span class="font-bold">Happy</span>: {$userData.data.bars.happy
            .current}/{$userData.data.bars.happy.maximum}
        </span>
        <span class="text-muted-foreground font-mono">
          {formatCountdown(happyTick)}
        </span>
      </p>
      <Progress
        value={($userData.data.bars.happy.current /
          $userData.data.bars.happy.maximum) *
          100}
        class="mt-0 h-2"
        barClass="bg-linear-to-b from-[#cccc32] to-[#b3992c]"
      />
    </div>
    <div>
      <p class="flex justify-between">
        <span>
          <span class="font-bold">Life</span>: {$userData.data.bars.life
            .current}/{$userData.data.bars.life.maximum}
        </span>
        <span class="text-muted-foreground font-mono"
          >{formatCountdown(lifeTick)}</span
        >
      </p>
      <Progress
        value={($userData.data.bars.life.current /
          $userData.data.bars.life.maximum) *
          100}
        class="mt-0 h-2"
        barClass="bg-linear-to-b from-[#708bdb] to-[#3f43cf]"
      />
    </div>
    <div>
      <p class="flex justify-between">
        <span>
          <span class="font-bold">Chain</span>: {$userData.data.bars.chain
            .current}/{$userData.data.bars.chain.max}
        </span>
        <span class="text-muted-foreground font-mono"
          >{formatCountdown(chainTick)}</span
        >
      </p>
      <Progress
        value={($userData.data.bars.chain.current /
          $userData.data.bars.chain.max) *
          100}
        class="mt-0 h-2"
        barClass="bg-linear-to-b from-[#878787] to-[#6b6b6b]"
      />
    </div>
  </Sidebar.Header>
  <Sidebar.Separator class="mx-0" />
  <Sidebar.Content>
    <Sidebar.Group class="">
      <Collapsible.Root open={true} class="group/collapsible">
        <Sidebar.GroupLabel
          class="group/label text-sidebar-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground w-full text-sm"
        >
          {#snippet child({ props })}
            <Collapsible.Trigger {...props}>
              Cooldowns
              <ChevronRightIcon
                class="ml-auto transition-transform group-data-[state=open]/collapsible:rotate-90"
              />
            </Collapsible.Trigger>
          {/snippet}
        </Sidebar.GroupLabel>
        <Collapsible.Content>
          <Sidebar.GroupContent class="p-2 text-sm">
            <Sidebar.Menu>
              <Sidebar.MenuItem>
                {formatDuration($userData.data.cooldowns.drug)}
              </Sidebar.MenuItem>
              <Sidebar.MenuItem>
                {formatDuration($userData.data.cooldowns.medical)}
              </Sidebar.MenuItem>
              <Sidebar.MenuItem>
                {formatDuration($userData.data.cooldowns.booster)}
              </Sidebar.MenuItem>
            </Sidebar.Menu>
          </Sidebar.GroupContent>
        </Collapsible.Content>
      </Collapsible.Root>
    </Sidebar.Group>
  </Sidebar.Content>
</Sidebar.Root>
