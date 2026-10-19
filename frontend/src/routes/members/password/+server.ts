import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { auth } from '$lib/server/auth-v2.js';

// Changement de mot de passe (l'actuel est exigé). Les autres sessions sont
// fermées ; la session en cours est remplacée par une neuve (cookie renvoyé).
export const PUT: RequestHandler = async ({ request, locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');
    const { currentPassword, newPassword } = await request.json();
    if (!currentPassword || !newPassword) return json({ ok: false, message: 'Champs requis.' }, { status: 400 });
    if (newPassword.length < 8) return json({ ok: false, message: 'Minimum 8 caractères.' }, { status: 400 });

    let result;
    try {
        result = await auth.api.changePassword({
            body: { currentPassword, newPassword, revokeOtherSessions: true },
            headers: request.headers,
            returnHeaders: true,
        });
    } catch {
        return json({ ok: false, message: 'Mot de passe actuel incorrect.' }, { status: 401 });
    }

    const response = json({ ok: true, message: 'Mot de passe mis à jour.' });
    for (const cookie of result.headers.getSetCookie()) {
        response.headers.append('set-cookie', cookie);
    }
    return response;
};
