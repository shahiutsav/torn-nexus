import { redirect } from "@sveltejs/kit";
import { invoke } from "@tauri-apps/api/core";

export async function load() {
  let isAuthenticated = await invoke("verify_session");
  if (!isAuthenticated) {
    redirect(303, "/setup");
  }
}
