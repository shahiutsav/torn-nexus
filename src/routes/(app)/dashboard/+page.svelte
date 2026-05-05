<script lang="ts">
  import { goto } from "$app/navigation";
  import { Button } from "@/components/ui/button/index.js";
  import { invoke } from "@tauri-apps/api/core";

  async function handleLogOut() {
    try {
      await invoke("log_out");
      goto("/setup", { replaceState: true });
    } catch (e) {
      console.error(e);
    }
  }
</script>

<main>
  <Button onclick={handleLogOut}>Log Out</Button>
</main>
