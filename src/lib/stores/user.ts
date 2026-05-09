import type { UserData } from "@/types/user";
import { writable } from "svelte/store";

export const userData = writable<UserData>();
