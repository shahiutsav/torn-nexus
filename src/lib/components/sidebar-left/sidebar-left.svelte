<script lang="ts">
  import * as Sidebar from "@/components/ui/sidebar/index.js";

  import type { ComponentProps } from "svelte";
  import FooterItem from "./footer-item.svelte";

  let {
    ref = $bindable(null),
    ...restProps
  }: ComponentProps<typeof Sidebar.Root> = $props();
</script>

<Sidebar.Root bind:ref class="border-e-0" {...restProps}>
  <Sidebar.Header />
  <Sidebar.Content>
    <Sidebar.Group />
    <Sidebar.Group />
  </Sidebar.Content>
  <Sidebar.Footer>
    <FooterItem />
  </Sidebar.Footer>
</Sidebar.Root>
