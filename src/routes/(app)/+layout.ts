import { userData } from "@/stores/user";
import type { DataUpdate, UserData } from "@/types/user";
import { redirect } from "@sveltejs/kit";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export async function load() {
  let isAuthenticated = await invoke("verify_session");
  if (!isAuthenticated) {
    redirect(303, "/setup");
  }

  const data = await invoke<DataUpdate>("fetch_user_data");
  userData.set(data);

  listen<DataUpdate>("data-updated", (event) => {
    userData.set(event.payload);
  });
}
