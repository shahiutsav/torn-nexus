<script lang="ts">
  import * as Card from "@/components/ui/card/index.js";
  import * as Sidebar from "@/components/ui/sidebar/index.js";
  import * as Table from "@/components/ui/table/index.js";
  import { Separator } from "@/components/ui/separator/index.js";

  import { userData } from "@/stores/user";

  const workstats = $derived(
    Object.entries($userData.workstats)
      .filter(([key]) => key !== "total")
      .map(([key, value]) => ({
        label: key.replaceAll("_", " "),
        value: value.toLocaleString(),
      })),
  );

  const jobPoints = $derived({
    jobs: Object.entries($userData.jobpoints.jobs).map(([key, value]) => ({
      label: key,
      value: value.toLocaleString(),
    })),
    companies: $userData.jobpoints.companies.map((c) => ({
      label: c.company.name,
      value: c.points.toLocaleString(),
    })),
  });
</script>

<Sidebar.Menu>
  <Card.Root class="gap-0 p-0">
    <Table.Root class="text-xs">
      <Table.Header>
        <Table.Row>
          <Table.Head colspan={2} class="text-center">Work Stats</Table.Head>
        </Table.Row>
      </Table.Header>
      <Table.Body>
        {#each workstats as workstat}
          <Table.Row>
            <Table.Cell class="capitalize">{workstat.label}</Table.Cell>
            <Table.Cell class="text-end">{workstat.value}</Table.Cell>
          </Table.Row>
        {/each}
      </Table.Body>
      <Table.Footer>
        <Table.Row>
          <Table.Cell>Total</Table.Cell>
          <Table.Cell class="text-end">
            {$userData.workstats.total.toLocaleString()}
          </Table.Cell>
        </Table.Row>
      </Table.Footer>
    </Table.Root>
  </Card.Root>
</Sidebar.Menu>
<Sidebar.Menu class="mt-2">
  <Card.Root class="gap-0 p-0">
    <Card.Header class="flex h-10 items-center justify-center text-xs">
      Job Points
    </Card.Header>
    <Separator />

    <Table.Root class="text-xs">
      <Table.Header class="bg-muted/50">
        <Table.Row>
          <Table.Head colspan={2} class="text-center">Jobs</Table.Head>
        </Table.Row>
      </Table.Header>
      <Table.Body>
        {#each jobPoints.jobs as job}
          <Table.Row>
            <Table.Cell class="capitalize">{job.label}</Table.Cell>
            <Table.Cell class="text-end">{job.value}</Table.Cell>
          </Table.Row>
        {/each}
      </Table.Body>
    </Table.Root>
    <Separator />
    <Table.Root class="text-xs">
      <Table.Header class="bg-muted/50">
        <Table.Row>
          <Table.Head colspan={2} class="text-center">Companies</Table.Head>
        </Table.Row>
      </Table.Header>
      <Table.Body>
        {#each jobPoints.companies as company}
          <Table.Row>
            <Table.Cell>{company.label}</Table.Cell>
            <Table.Cell class="text-end">{company.value}</Table.Cell>
          </Table.Row>
        {/each}
      </Table.Body>
    </Table.Root>
  </Card.Root>
</Sidebar.Menu>
