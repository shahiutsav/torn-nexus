<script lang="ts">
  import { goto } from "$app/navigation";
  import { Button } from "@/components/ui/button/index.js";
  import { invoke } from "@tauri-apps/api/core";

  import { listen } from "@tauri-apps/api/event";

  let data = $state<number | null>(null);

  $effect(() => {
    let unlisten: (() => void) | undefined;

    (async () => {
      unlisten = await listen<number>("data-updated", (event) => {
        data = event.payload;
      });
    })();

    return () => unlisten?.();
  });

  async function handleLogOut() {
    await invoke("log_out");
    await goto("/setup", { replaceState: true });
  }
</script>

<main>
  <p>{data}</p>
  <Button onclick={handleLogOut}>Log Out</Button>
</main>
