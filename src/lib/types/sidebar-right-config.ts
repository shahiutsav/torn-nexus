import type { Icon } from "./user";

export type BarConfig = {
  label: string;
  current: number;
  maximum: number;
  tick: number;
  barClass: string;
  overflowable?: boolean;
};

export type CooldownConfig = {
  icon?: Icon;
  countdown: number;
  noIconSrc: string;
  label: string;
};
