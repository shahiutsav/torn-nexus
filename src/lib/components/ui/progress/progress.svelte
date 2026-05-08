<script lang="ts">
  import { Progress as ProgressPrimitive } from "bits-ui";
  import { cn, type WithoutChildrenOrChild } from "$lib/utils.js";

  let {
    ref = $bindable(null),
    class: className,
    barClass,
    max = 100,
    value,
    ...restProps
  }: WithoutChildrenOrChild<ProgressPrimitive.RootProps> & {
    barClass?: string;
  } = $props();
</script>

<ProgressPrimitive.Root
  bind:ref
  data-slot="progress"
  class={cn(
    "bg-muted relative flex h-1.5 w-full items-center overflow-x-hidden rounded-full",
    className,
  )}
  {value}
  {max}
  {...restProps}
>
  <div
    data-slot="progress-indicator"
    class={cn("bg-primary size-full flex-1 transition-all", barClass)}
    style="transform: translateX(-{100 - (100 * (value ?? 0)) / (max ?? 1)}%)"
  ></div>
</ProgressPrimitive.Root>
