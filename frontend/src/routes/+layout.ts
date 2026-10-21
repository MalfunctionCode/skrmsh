import type { LayoutLoad } from './$types';
import { player } from '$lib/stores/auth';

export const ssr = false;

export const load: LayoutLoad = async ({ fetch }) => {
  try {
    const res = await fetch('/api/players/me');
    player.set(res.ok ? await res.json() : null)
  } catch {
    player.set(null)
  }
}
