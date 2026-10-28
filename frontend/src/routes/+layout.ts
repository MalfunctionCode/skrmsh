import type { PlayerResponse } from '$lib/api/schema';
import type { LayoutLoad } from './$types';

export const ssr = false;

export const load: LayoutLoad = async ({ fetch }) => {
	let me: PlayerResponse | null = null;

	const res = await fetch('/api/players/me');
	if (res.ok) {
		me = await res.json();
	}

	return { me };
};
