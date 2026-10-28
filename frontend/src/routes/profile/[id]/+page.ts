import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import type { PlayerResponse } from "$lib/api/schema";

export const load: PageLoad = async ({ fetch, params }) => {
  const res = await fetch(`/api/players/${params.id}`)
  if (res.status === 404) error(404, 'Player not found')
  if (!res.ok) error(500, 'Failed to load player')

  const player: PlayerResponse = await res.json();
  return{player}
}
