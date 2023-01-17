import { writable, type Writable } from "svelte/store";
import type Scene from "../lib/scene";

export const sceneStore: Writable<Scene | undefined> = writable(undefined);