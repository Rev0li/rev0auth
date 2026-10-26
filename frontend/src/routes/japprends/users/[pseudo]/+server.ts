import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { findUserByPseudo, updateUser, deleteUser, ROLES } from '$lib/server/users.js';
import { writeAudit } from '$lib/server/audit.js';

function requireAdmin({ locals }: { locals: App.Locals }) {
    if (!locals.adminSession) throw error(401, 'Non autorisé.');
}

const BOOLEAN_FIELDS = ['active', 'approved'] as const;
const TEXT_FIELDS = ['bio'] as const;

// Seuls les champs métier listés ici sont modifiables, avec leur type
export const PUT: RequestHandler = async ({ request, locals, params }) => {
    requireAdmin({ locals });
    const body = await request.json().catch(() => null);
    if (!body || typeof body !== 'object') return json({ ok: false, message: 'Corps invalide.' }, { status: 400 });

    const patch: Record<string, unknown> = {};
    if ('role' in body) {
        if (!ROLES.includes(body.role)) return json({ ok: false, message: 'Rôle inconnu.' }, { status: 400 });
        patch.role = body.role;
    }
    for (const field of BOOLEAN_FIELDS) {
        if (!(field in body)) continue;
        if (typeof body[field] !== 'boolean') return json({ ok: false, message: `${field} invalide.` }, { status: 400 });
        patch[field] = body[field];
    }
    for (const field of TEXT_FIELDS) {
        if (!(field in body)) continue;
        const value = body[field];
        if (value !== null && (typeof value !== 'string' || value.length > 500)) {
            return json({ ok: false, message: `${field} invalide.` }, { status: 400 });
        }
        patch[field] = value;
    }
    if (Object.keys(patch).length === 0) return json({ ok: false, message: 'Aucun champ modifiable.' }, { status: 400 });

    const user = await findUserByPseudo(params.pseudo);
    if (!user) return json({ ok: false, message: 'Utilisateur introuvable.' }, { status: 404 });

    await updateUser(user.id, patch);
    if ('role' in patch || 'active' in patch) {
        await writeAudit('update_user', locals.adminSession!.pseudo, user.name, JSON.stringify(patch));
    }
    return json({ ok: true });
};

export const DELETE: RequestHandler = async ({ locals, params }) => {
    requireAdmin({ locals });
    const user = await findUserByPseudo(params.pseudo);
    if (!user) return json({ ok: false, message: 'Utilisateur introuvable.' });

    await deleteUser(user.id); // sessions, mot de passe, clés admin, avatar en cascade
    await writeAudit('delete_user', locals.adminSession!.pseudo, user.name, 'user and all data deleted');
    return json({ ok: true });
};
