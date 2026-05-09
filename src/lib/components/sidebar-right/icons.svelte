<script lang="ts">
  import { userData } from "@/stores/user";
  import {
    COOLDOWN_CONFIG,
    MISC_ICON_TITLES,
  } from "@/types/sidebar-right-config";

  const EXCLUDED_TITLES = [
    ...COOLDOWN_CONFIG.map((c) => `${c.label} Cooldown`),
    ...MISC_ICON_TITLES,
  ];

  const remainingIcons = $derived(
    $userData.icons.filter(
      (icon) => !EXCLUDED_TITLES.some((title) => icon.title.includes(title)),
    ),
  );
</script>

<ul class="leading-none">
  {#each remainingIcons as icon}
    <li
      class="m-0 mr-2.5 inline-block h-4 w-4"
      style="
        background-image: url('https://www.torn.com/images/v2/svg_icons/sprites/user_status_icons_sprite.svg?v=1761056520');
        background-position: -{(icon.id - 1) * 18}px;
        background-repeat:none;
      "
    ></li>
  {/each}
</ul>
