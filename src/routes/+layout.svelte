<script module>
  import { getCurrentWindow } from "@tauri-apps/api/window";
  const appWindow = getCurrentWindow();
</script>

<script lang="ts">
  import "./layout.css";
  import { ModeWatcher } from "mode-watcher";
  import { Button } from "@/components/ui/button/index.js";
  import * as Tooltip from "@/components/ui/tooltip/index.js";
  const { children } = $props();
</script>

<ModeWatcher />

<div
  class="titlebar border-accent bg-sidebar fixed top-0 right-0 left-0 z-99 grid h-8.75 grid-cols-[auto_max-content] border-b select-none"
>
  <p
    data-tauri-drag-region
    class="absolute top-1/2 left-0 my-auto ml-3 -translate-y-1/2 text-center text-xs font-bold"
  >
    TornNexus
  </p>
  <div data-tauri-drag-region></div>
  <div class="controls flex">
    <Button
      size={"icon"}
      id="titlebar-minimize"
      variant={"ghost"}
      title="Minimize"
      class="inline-flex h-8.75 w-8.75 rounded-none border-none"
      onclick={() => {
        appWindow.minimize();
      }}
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
      >
        <path fill="currentColor" d="M19 13H5v-2h14z" />
      </svg>
    </Button>
    <Button
      size={"icon"}
      id="titlebar-maximize"
      variant={"ghost"}
      title="Maximize"
      class="inline-flex h-8.75 w-8.75 rounded-none border-none"
      onclick={() => appWindow.toggleMaximize()}
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
      >
        <path fill="currentColor" d="M4 4h16v16H4zm2 4v10h12V8z" />
      </svg>
    </Button>
    <Button
      size={"icon"}
      id="titlebar-close"
      variant={"ghost"}
      title="Close"
      class="hover:bg-destructive dark:hover:bg-destructive inline-flex h-8.75 w-8.75 rounded-none border-none"
      onclick={() => appWindow.close()}
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
      >
        <path
          fill="currentColor"
          d="M13.46 12L19 17.54V19h-1.46L12 13.46L6.46 19H5v-1.46L10.54 12L5 6.46V5h1.46L12 10.54L17.54 5H19v1.46z"
        />
      </svg>
    </Button>
  </div>
</div>

<div class="relative top-8.75 h-[calc(100svh-35px)]">
  <Tooltip.Provider delayDuration={0}>
    {@render children()}
  </Tooltip.Provider>
</div>
