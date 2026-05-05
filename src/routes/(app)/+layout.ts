import { isAuthenticated } from "@/auth";
import { redirect } from "@sveltejs/kit";
import { invoke } from "@tauri-apps/api/core";
import { get } from "svelte/store";

export async function load() {
  if (get(isAuthenticated) !== null) return;
  try {
    await invoke("authenticate_from_keyring");
    isAuthenticated.set(true);
  } catch {
    isAuthenticated.set(false);
    redirect(303, "/setup");
  }
}
