<script lang="ts">
  import * as Card from "@/components/ui/card/index.js";
  import * as Table from "@/components/ui/table/index.js";
  import { userData } from "@/stores/user.js";
  import { cn } from "@/utils";
</script>

<Card.Root class="p-0">
  <Table.Root class="text-xs">
    <Table.Body>
      {#each $userData.skills as skill}
        <Table.Row>
          <Table.Cell>{skill.name}</Table.Cell>
          <Table.Cell
            class={cn("text-end", skill.level < 100 ? "text-destructive" : "")}
            >{skill.level}</Table.Cell
          >
        </Table.Row>
      {/each}
    </Table.Body>
  </Table.Root>
</Card.Root>
