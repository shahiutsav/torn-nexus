<script lang="ts" module>
  import { string, z } from "zod";

  const formSchema = z.object({
    apiKey: z
      .string()
      .min(16, "The API Key must be exactly 16 characters")
      .max(16, "The API Key must be exactly 16 characters"),
  });
</script>

<script lang="ts">
  import { defaults, setError, superForm } from "sveltekit-superforms";
  import { zod4 } from "sveltekit-superforms/adapters";
  import * as Form from "$lib/components/ui/form/index.js";
  import * as InputGroup from "$lib/components/ui/input-group/index.js";
  import EyeIcon from "@lucide/svelte/icons/eye";
  import EyeCloseIcon from "@lucide/svelte/icons/eye-closed";
  import { Button } from "@/components/ui/button/index.js";
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";

  let showPassword = $state(false);

  const form = superForm(defaults(zod4(formSchema)), {
    validators: zod4(formSchema),
    SPA: true,
    resetForm: false,
    onUpdate: async ({ form }) => {
      if (form.valid) {
        try {
          await invoke("validate_and_save_key", {
            apiKey: form.data.apiKey,
          }).then(() => {
            console.log("api key verified, trying to navigate");
            window.location.replace("/");
          });
        } catch (e) {
          console.error(e);
          setError(form, "apiKey", e as string);
        }
      }
    },
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

        <InputGroup.Root>
          <InputGroup.Input
            {...props}
            bind:value={$formData.apiKey}
            placeholder="Enter your API Key here..."
            class="font-mono"
            type={showPassword ? "text" : "password"}
          />
          <InputGroup.Addon align="inline-end">
            <InputGroup.Button onclick={() => (showPassword = !showPassword)}>
              {#if showPassword}
                <EyeCloseIcon />
              {:else}
                <EyeIcon />
              {/if}
            </InputGroup.Button>
          </InputGroup.Addon>
        </InputGroup.Root>
      {/snippet}
    </Form.Control>
    <Form.FieldErrors />
    <Form.Description>
      At least
      <span class="text-[#b28500] dark:text-[#fcc419]"> Limited Access </span>
      permissions needed.
    </Form.Description>
  </Form.Field>

  <Form.Button class="mt-2 w-full">Submit</Form.Button>
</form>
