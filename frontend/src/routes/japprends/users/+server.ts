import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { hashPassword } from '$lib/server/auth.js';
import { listUsers, adminView, createUser, findUserByPseudo, PSEUDO_RE, ROLES } from '$lib/server/users.js';

function requireAdmin({ locals }: { locals: App.Locals }) {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
}

export const GET: RequestHandler = async ({ locals }) => {
    requireAdmin({ locals });
    return json((await listUsers()).map(adminView));
};

export const POST: RequestHandler = async ({ request, locals }) => {
    requireAdmin({ locals });
    const { pseudo, role, password } = await request.json();
    const key = typeof pseudo === 'string' ? pseudo.trim().toLowerCase() : '';
    if (!key || typeof password !== 'string' || !password) {
        return json({ ok: false, message: 'Champs requis.' }, { status: 400 });
    }
    if (!PSEUDO_RE.test(key)) {
        return json({ ok: false, message: 'Pseudo invalide (3-20 caractères, lettres, chiffres, _ ou -).' }, { status: 400 });
    }
    if (role !== undefined && !ROLES.includes(role)) {
        return json({ ok: false, message: 'Rôle inconnu.' }, { status: 400 });
    }
    if (await findUserByPseudo(key)) {
        return json({ ok: false, message: 'Ce pseudo est déjà utilisé.' }, { status: 409 });
    }

    await createUser({ pseudo: key, passwordHash: await hashPassword(password), role: role ?? 'member' });
    return json({ ok: true, pseudo: key });
};
