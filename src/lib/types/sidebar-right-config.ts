export const COOLDOWN_CONFIG = [
  {
    key: "drug",
    label: "Drug",
    noIconSrc: "/icons/cooldowns/no_drug_cd.svg",
  },
  {
    key: "medical",
    label: "Medical",
    noIconSrc: "/icons/cooldowns/no_med_cd.svg",
  },
  {
    key: "booster",
    label: "Booster",
    noIconSrc: "/icons/cooldowns/no_booster_cd.svg",
  },
] as const;

export type CooldownKey = (typeof COOLDOWN_CONFIG)[number]["key"];

export const MISC_ICON_TITLES = [
  "Bank Investment",
  "Reading Book",
  "Education",
  "Organized Crime",
  "Subscriber",
  "Donator",
] as const;

export type MiscIconTitle = (typeof MISC_ICON_TITLES)[number];
