import type { DataUpdate, UserData } from "@/types/user";
import { writable } from "svelte/store";

export const userData = writable<DataUpdate>();
