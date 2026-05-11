import type {
  COOLDOWN_CONFIG,
  MISC_ICON_TITLES,
} from "@/constants/sidebar-right-constants";

export type CooldownKey = (typeof COOLDOWN_CONFIG)[number]["key"];

export type MiscIconTitle = (typeof MISC_ICON_TITLES)[number];
