export const BAR_CONFIG = [
  {
    key: "energy" as const,
    label: "Energy",
    barClass: "bg-linear-to-b from-[#6cad2b] to-[#4d7c1e]",
  },
  {
    key: "nerve" as const,
    label: "Nerve",
    barClass: "bg-linear-to-b from-[#cc7032] to-[#b3382c]",
  },
  {
    key: "happy" as const,
    label: "Happy",
    barClass: "bg-linear-to-b from-[#cccc32] to-[#b3992c]",
    overflowable: true,
  },
  {
    key: "life" as const,
    label: "Life",
    barClass: "bg-linear-to-b from-[#708bdb] to-[#3f43cf]",
  },
] as const;

export const COOLDOWN_CONFIG = [
  {
    key: "drug",
    label: "Drug",
    noIconSrc: "/icons/cooldowns/no_drug_cd.svg",
    showDescription: true,
  },
  {
    key: "medical",
    label: "Medical",
    noIconSrc: "/icons/cooldowns/no_med_cd.svg",
    showDescription: false,
  },
  {
    key: "booster",
    label: "Booster",
    noIconSrc: "/icons/cooldowns/no_booster_cd.svg",
    showDescription: false,
  },
] as const;

export const MISC_ICON_TITLES = [
  "Bank Investment",
  "Reading Book",
  "Education",
  "Organized Crime",
  "Subscriber",
  "Donator",
] as const;
