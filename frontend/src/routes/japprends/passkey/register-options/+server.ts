import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { requireAdminMember, registrationOptions, setupCodeValid } from '$lib/server/admin-passkey.js';
import { checkRateLimit, recordFailure, getIp } from '$lib/server/ratelimit.js';

// Enregistrer une clé : depuis une session déjà validée (clé supplémentaire),
// ou avec le code d'installation généré sur le serveur (première clé, clé perdue).
export const POST: RequestHandler = async ({ request, locals }) => {
    const admin = requireAdminMember(locals);
    const ip = getIp(request);
    if (checkRateLimit(ip).blocked) {
        return json({ ok: false, message: 'Trop de tentatives, réessaie dans 15 minutes.' }, { status: 429 });
    }

    if (locals.adminSession) {
        return json({ ok: true, options: await registrationOptions(admin, false) });
    }

    const { code } = await request.json().catch(() => ({}));
    if (typeof code !== 'string' || !(await setupCodeValid(admin.pseudo.toLowerCase(), code))) {
        recordFailure(ip);
        return json({ ok: false, message: 'Code invalide ou expiré.' }, { status: 403 });
    }
    return json({ ok: true, options: await registrationOptions(admin, true) });
};
