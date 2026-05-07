<script lang="ts">
  import { goto } from "$app/navigation";
  import { Button } from "@/components/ui/button/index.js";
  import { invoke } from "@tauri-apps/api/core";

  import { listen } from "@tauri-apps/api/event";

  async function handleLogOut() {
    await invoke("log_out");
    await goto("/setup", { replaceState: true });
  }
</script>

<main>
  <Button onclick={handleLogOut}>Log Out</Button>
</main>
