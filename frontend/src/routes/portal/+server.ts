import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';

// Signup is invite-only — no public submission endpoint.
export const POST: RequestHandler = async () => {
    return json(
        { ok: false, message: 'Inscription sur invitation uniquement.' },
        { status: 403 }
    );
};
