import { z } from "zod";

export const formSchema = z.object({
  apiKey: z
    .string()
    .min(16, "The API Key must be exactly 16 characters")
    .max(16, "The API Key must be exactly 16 characters"),
});

export type FormSchema = typeof formSchema;
