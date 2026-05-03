<script lang="ts">
  import * as Form from "$lib/components/ui/form/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Button } from "@/components/ui/button/index.js";
  import { formSchema, type FormSchema } from "./schema";
  import {
    type SuperValidated,
    type Infer,
    superForm,
  } from "sveltekit-superforms";
  import { zod4Client } from "sveltekit-superforms/adapters";

  let { data }: { data: { form: SuperValidated<Infer<FormSchema>> } } =
    $props();

  const form = superForm(data.form, {
    validators: zod4Client(formSchema),
  });

  const { form: formData, enhance } = form;
</script>

<form method="POST" use:enhance>
  <Form.Field {form} name="apiKey">
    <Form.Control>
      {#snippet children({ props })}
        <div class="flex items-center">
          <Form.Label>API Key</Form.Label>
          <Button
            href="https://www.torn.com/preferences.php#tab=api?step=addNewKey&title=torn-nexus&type=3"
            variant="link"
            target="_blank"
            class="ms-auto inline-block h-min text-sm underline-offset-4"
          >
            Generate me an API Key
          </Button>
        </div>
        <Input
          {...props}
          placeholder="Enter your API Key here"
          bind:value={$formData.apiKey}
          class="font-mono"
        />
        <Form.Description>
          Your API key must have at least
          <span class="text-[#b28500] dark:text-[#fcc419]">
            Limited Access
          </span>
          permissions for TornNexus to work properly.
        </Form.Description>
      {/snippet}
    </Form.Control>
    <Form.FieldErrors class="mb-3" />
  </Form.Field>
  <Form.Button class="w-full">Submit</Form.Button>
</form>
