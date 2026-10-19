import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { requireAdminMember, verifyAuthentication, elevateSession } from '$lib/server/admin-passkey.js';
import { checkRateLimit, recordFailure, clearAttempts, getIp } from '$lib/server/ratelimit.js';
import { writeAudit } from '$lib/server/audit.js';

// Étape 2 : la clé a signé le défi → la session en cours ouvre la console
export const POST: RequestHandler = async ({ request, locals }) => {
    const admin = requireAdminMember(locals);
    const ip = getIp(request);
    if (checkRateLimit(ip).blocked) {
        return json({ ok: false, message: 'Trop de tentatives, réessaie dans 15 minutes.' }, { status: 429 });
    }

    const { response } = await request.json().catch(() => ({}));
    if (!(await verifyAuthentication(admin, response))) {
        recordFailure(ip);
        await writeAudit('admin_login_failed', admin.pseudo, '', 'passkey');
        return json({ ok: false, message: 'Clé non reconnue.' }, { status: 401 });
    }

    clearAttempts(ip);
    await elevateSession(admin.sessionId);
    await writeAudit('admin_login', admin.pseudo, '', 'passkey');
    return json({ ok: true });
};
