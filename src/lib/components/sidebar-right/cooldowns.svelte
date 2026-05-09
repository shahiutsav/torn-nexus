<script lang="ts">
  import { userData } from "@/stores/user";
  import * as Collapsible from "$lib/components/ui/collapsible/index.js";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import { formatCountdownHHMMSS, startCountdown } from "@/utils";
  import type { CooldownConfig } from "@/types/sidebar-right-config";
  import * as Item from "../ui/item/index";

  // TODO: Derived in multiple places, make it so that there's only one source of truth
  const drugIcon = $derived(
    $userData.icons.find((icon) => icon.title.includes("Drug Cooldown")),
  );
  const medicalIcon = $derived(
    $userData.icons.find((icon) => icon.title.includes("Medical Cooldown")),
  );
  const boosterIcon = $derived(
    $userData.icons.find((icon) => icon.title.includes("Booster Cooldown")),
  );

  let drugCooldown = $state($userData.cooldowns.drug);
  let medicalCooldown = $state($userData.cooldowns.medical);
  let boosterCooldown = $state($userData.cooldowns.booster);

  $effect(() => {
    const offset = Math.floor(Date.now() / 1000) - $userData.timestamp;
    const cleanups = [
      startCountdown(
        $userData.cooldowns.drug - offset,
        (r) => (drugCooldown = r),
      ),
      startCountdown(
        $userData.cooldowns.medical - offset,
        (r) => (medicalCooldown = r),
      ),
      startCountdown(
        $userData.cooldowns.booster - offset,
        (r) => (boosterCooldown = r),
      ),
    ];

    return () => cleanups.forEach((cleanup) => cleanup());
  });

  const cooldowns: CooldownConfig[] = $derived([
    {
      icon: drugIcon,
      countdown: drugCooldown,
      noIconSrc: "/icons/cooldowns/no_drug_cd.svg",
      label: "Drug",
    },
    {
      icon: medicalIcon,
      countdown: medicalCooldown,
      noIconSrc: "/icons/cooldowns/no_med_cd.svg",
      label: "Medical",
    },
    {
      icon: boosterIcon,
      countdown: boosterCooldown,
      noIconSrc: "/icons/cooldowns/no_booster_cd.svg",
      label: "Booster",
    },
  ]);
</script>

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
    <Sidebar.GroupContent class="p-2">
      <Sidebar.Menu>
        {#each cooldowns as cooldown}
          <Sidebar.MenuItem>
            <Item.Root variant="muted" class="p-2.5">
              {#if cooldown.icon}
                <Item.Media variant="icon">
                  <div
                    style="
                      width: 16px;
                      height: 16px;
                      background-image: url('https://www.torn.com/images/v2/svg_icons/sprites/user_status_icons_sprite.svg?v=1761056520');
                      background-position: -{(cooldown.icon.id - 1) * 18}px;
                      background-repeat:none;
                "
                  ></div>
                </Item.Media>
                <Item.Content>
                  <Item.Title class="font-mono font-bold">
                    {formatCountdownHHMMSS(cooldown.countdown)}
                  </Item.Title>
                  <Item.Description class="font-sans text-sm">
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
        {/each}
      </Sidebar.Menu>
    </Sidebar.GroupContent>
  </Collapsible.Content>
</Collapsible.Root>
