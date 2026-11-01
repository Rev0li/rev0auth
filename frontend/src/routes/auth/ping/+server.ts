import type { RequestHandler } from './$types.js';

// Battement de présence des pages connectées (AppNav, une fois par minute) :
// hooks.server.ts a déjà noté l'activité du membre ; rien d'autre à faire.
export const POST: RequestHandler = async ({ locals }) =>
    new Response(null, { status: locals.memberSession ? 204 : 401, headers: { 'cache-control': 'no-store' } });
