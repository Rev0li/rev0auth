import type { RequestHandler } from './$types.js';
import { auth } from '$lib/server/auth-v2.js';

// Toutes les routes BetterAuth (sign-up, sign-in, session, …) sous /api/auth/*.
// Coexiste avec l'auth custom pendant la Phase 2 — aucun flow existant ne pointe ici encore.
export const GET: RequestHandler = ({ request }) => auth.handler(request);
export const POST: RequestHandler = ({ request }) => auth.handler(request);
