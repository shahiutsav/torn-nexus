<script lang="ts">
  import { userData } from "@/stores/user";

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
</script>

<ul>
  {#each $userData.icons.filter((icon) => icon !== drugIcon && icon !== medicalIcon && icon !== boosterIcon) as icon}
    <li
      class="mr-2.5 mb-2.5 inline-block h-4 w-4"
      style="
        background-image: url('https://www.torn.com/images/v2/svg_icons/sprites/user_status_icons_sprite.svg?v=1761056520');
        background-position: -{(icon.id - 1) * 18}px;
        background-repeat:none;
      "
    ></li>
  {/each}
</ul>
