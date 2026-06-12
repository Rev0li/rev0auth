import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';

// L'ancienne page /portal (Rust) n'affichait que "inscription sur invitation
// uniquement" — info désormais intégrée à la page de connexion. Redirect
// permanent pour les vieux liens.
export const GET: RequestHandler = async () => {
    throw redirect(301, '/');
};
