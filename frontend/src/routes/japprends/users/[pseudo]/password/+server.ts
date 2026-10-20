import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { hashPassword } from '$lib/server/auth.js';
import { findUserByPseudo, setPassword, removePassword, revokeSessions } from '$lib/server/users.js';
import { writeAudit } from '$lib/server/audit.js';

// Mot de passe remplacé ou retiré par l'admin : les sessions du membre sont
// fermées, il devra se reconnecter (ou ne le pourra plus, sans mot de passe).
export const POST: RequestHandler = async ({ request, locals, params }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
    const { password } = await request.json();
    if (typeof password !== 'string' || !password) return json({ ok: false, message: 'Mot de passe requis.' }, { status: 400 });
    if (password.length < 8) return json({ ok: false, message: 'Minimum 8 caractères.' }, { status: 400 });

    const user = await findUserByPseudo(params.pseudo);
    if (!user) return json({ ok: false, message: 'Utilisateur introuvable.' });

    await setPassword(user.id, await hashPassword(password));
    await revokeSessions(user.id);
    await writeAudit('set_password', locals.adminSession.pseudo, user.name, 'password replaced');
    return json({ ok: true });
};

export const DELETE: RequestHandler = async ({ locals, params }) => {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');

    const user = await findUserByPseudo(params.pseudo);
    if (!user) return json({ ok: false, message: 'Utilisateur introuvable.' });
    if (!(await removePassword(user.id))) {
        return json({ ok: false, message: 'Pas de mot de passe pour cet utilisateur.' });
    }

    await revokeSessions(user.id);
    await writeAudit('remove_password', locals.adminSession.pseudo, user.name, 'password cleared');
    return json({ ok: true, message: 'Mot de passe supprime.' });
};
