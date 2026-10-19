import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { requireAdminMember, verifyRegistration, elevateSession } from '$lib/server/admin-passkey.js';
import { writeAudit } from '$lib/server/audit.js';

// La clé enregistrée prouve sa présence : la session ouvre aussi la console
export const POST: RequestHandler = async ({ request, locals }) => {
    const admin = requireAdminMember(locals);
    const { response, name } = await request.json().catch(() => ({}));

    const result = await verifyRegistration(admin, response, typeof name === 'string' ? name : '');
    if (!result) return json({ ok: false, message: "Enregistrement de la clé refusé." }, { status: 400 });

    await elevateSession(admin.sessionId);
    await writeAudit('admin_passkey_registered', admin.pseudo, typeof name === 'string' ? name.slice(0, 60) : '',
        result.viaCode ? 'setup code' : 'validated session');
    return json({ ok: true });
};
