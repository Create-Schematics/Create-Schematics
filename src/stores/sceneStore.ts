import { writable } from "svelte/store";
import * as THREE from 'three';

export const scene = writable(new THREE.Scene());