<script lang="ts">
  import * as Card from "@/components/ui/card/index.js";
  import SettingsForm from "./settings-form.svelte";
  import { Button } from "@/components/ui/button/index.js";
  import ModeToggle from "@/components/mode-toggle.svelte";
</script>

<main>
  <Card.Root class="absolute top-1/2 left-1/2 w-90 -translate-1/2">
    <Card.Header>
      <Card.Title class="flex items-center">
        <img
          src="./icon.png"
          alt="TornNexus Logo"
          class="inline-block h-9 w-9"
        />
        <span class="ml-0.5 text-xl font-bold tracking-wider uppercase">
          ornNexus
        </span>
      </Card.Title>
      <Card.Action>
        <ModeToggle />
      </Card.Action>
    </Card.Header>
    <Card.Content>
      <SettingsForm />
    </Card.Content>
    <Card.Footer>
      <Button
        class="w-full"
        variant="outline"
        href="https://www.torn.com/3255609"
        target="_blank"
      >
        New to Torn? Start Playing!
      </Button>
    </Card.Footer>
  </Card.Root>
</main>
