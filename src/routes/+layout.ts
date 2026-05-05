// Tauri doesn't have a Node.js server to do proper SSR
// so we use adapter-static with a fallback to index.html to put the site in SPA mode
// See: https://svelte.dev/docs/kit/single-page-apps

import { isAuthenticated } from "@/auth";
import { redirect } from "@sveltejs/kit";
import { invoke } from "@tauri-apps/api/core";
import { get } from "svelte/store";

// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const ssr = false;

export async function load() {
  if (get(isAuthenticated) !== null) return;
  try {
    await invoke("authenticate_from_keyring");
    isAuthenticated.set(true);
  } catch {
    isAuthenticated.set(false);
    redirect(307, "/setup");
  }
}
