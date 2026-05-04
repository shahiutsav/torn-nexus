import { redirect } from "@sveltejs/kit";

export function load() {
  // TODO: Authenticate the user and redirect accordingly
  redirect(307, "/setup");
}
