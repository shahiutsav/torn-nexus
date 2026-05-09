import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChild<T> = T extends { child?: any } ? Omit<T, "child"> : T;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChildren<T> = T extends { children?: any }
  ? Omit<T, "children">
  : T;
export type WithoutChildrenOrChild<T> = WithoutChildren<WithoutChild<T>>;
export type WithElementRef<T, U extends HTMLElement = HTMLElement> = T & {
  ref?: U | null;
};

export function formatDuration(seconds: number): string {
  const days = Math.floor(seconds / 86400);
  const hours = Math.floor((seconds % 86400) / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;

  const parts = [
    days > 0 ? `${days}d` : null,
    hours > 0 ? `${hours}h` : null,
    minutes > 0 ? `${minutes}m` : null,
    secs > 0 ? `${secs}s` : null,
  ].filter(Boolean);

  return parts.slice(0, 2).join(" ") || "0s";
}

export function formatCountdownMMSS(seconds: number): string {
  if (seconds <= 0) return "00:00";
  const minutes = Math.floor(seconds / 60);
  const secs = seconds % 60;
  return `${String(minutes).padStart(2, "0")}:${String(secs).padStart(2, "0")}`;
}

export function formatCountdownHHMMSS(seconds: number): string {
  const hours = Math.floor(seconds / (60 * 60));
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;
  return `${String(hours).padStart(2, "0")}:${String(minutes).padStart(2, "0")}:${String(secs).padStart(2, "0")}`;
}

export function startCountdown(
  seconds: number,
  onTick: (remaining: number) => void,
  resetTo?: number,
) {
  let remaining = seconds;
  let timerId: ReturnType<typeof setTimeout>;

  function tick() {
    if (remaining <= 0 && resetTo) remaining = resetTo;
    remaining -= 1;
    onTick(remaining);
    if (remaining > 0) {
      const ms = 1000 - (Date.now() % 1000);
      timerId = setTimeout(tick, ms);
    }
  }

  const ms = 1000 - (Date.now() % 1000);
  timerId = setTimeout(tick, ms);

  return () => clearTimeout(timerId);
}
