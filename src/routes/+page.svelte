<script lang="ts">
  import SearchIcon from "@lucide/svelte/icons/search";
  import { invoke } from "@tauri-apps/api/core";
  import * as InputGroup from "@/components/ui/input-group/index.js";
  import { Button } from "$lib/components/ui/button/index.js";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<main class="container">
  <h1>Welcome to Tauri + Svelte</h1>

  <div class="row">
    <form class="row" onsubmit={greet}>
      <InputGroup.Root
        ><InputGroup.Input placeholder="Search..." />
        <InputGroup.Addon>
          <SearchIcon />
        </InputGroup.Addon>
        <InputGroup.Addon align="inline-end">
          <InputGroup.Button>Search</InputGroup.Button>
        </InputGroup.Addon>
      </InputGroup.Root>
      <Button type="submit">Greet</Button>
    </form>
    <p>{greetMsg}</p>
  </div>
</main>
