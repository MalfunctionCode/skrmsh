import { writable, derived } from 'svelte/store';
import type { components } from '$lib/api/schema';

type Player = components['schemas']['PlayerResponse'];

export const player = writable<Player | null>(null);

export const isLoggedIn = derived(player, $player => $player !== null)
export const isOnTeam = derived(player, $player => $player !== null && $player?.team_id !== null)
