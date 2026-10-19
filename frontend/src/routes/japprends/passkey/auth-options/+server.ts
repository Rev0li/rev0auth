import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { requireAdminMember, authenticationOptions } from '$lib/server/admin-passkey.js';
import { checkRateLimit, getIp } from '$lib/server/ratelimit.js';

// Étape 1 de l'entrée dans la console : défi WebAuthn pour les clés du compte
export const POST: RequestHandler = async ({ request, locals }) => {
    const admin = requireAdminMember(locals);
    if (checkRateLimit(getIp(request)).blocked) {
        return json({ ok: false, message: 'Trop de tentatives, réessaie dans 15 minutes.' }, { status: 429 });
    }
    const options = await authenticationOptions(admin);
    if (!options) return json({ ok: false, message: 'Aucune clé enregistrée.' }, { status: 409 });
    return json({ ok: true, options });
};
