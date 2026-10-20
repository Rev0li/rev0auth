import { describe, it, expect, vi, beforeEach } from 'vitest';

// Complément de routes-access.test.ts : de la session BetterAuth aux `locals`
// que les routes contrôlent (AUTH-05).

const { getSession, isSessionElevated } = vi.hoisted(() => ({
    getSession: vi.fn(),
    isSessionElevated: vi.fn(),
}));

vi.mock('$lib/server/db/index.js', () => ({ migrateDb: async () => {} }));
vi.mock('$lib/server/auth-v2.js', () => ({ auth: { api: { getSession } } }));
vi.mock('$lib/server/admin-passkey.js', () => ({ isSessionElevated }));

const { handle } = await import('../../../hooks.server.js');

const COOKIE = '__Secure-better-auth.session_token=abc';

async function localsFor(cookie: string | null): Promise<App.Locals> {
    const headers = new Headers(cookie ? { cookie } : {});
    const event = { request: new Request('http://localhost/', { headers }), locals: {} } as never;
    await handle({ event, resolve: async () => new Response() });
    return (event as { locals: App.Locals }).locals;
}

function session(user: { role: string; active: boolean }) {
    return { user: { id: 'u1', name: 'Alice', accessSongsurf: true, accessJellyfin: false, ...user }, session: { id: 's1' } };
}

beforeEach(() => {
    getSession.mockReset();
    isSessionElevated.mockReset();
});

describe('hooks.server — sessions', () => {
    it('sans cookie BetterAuth : anonyme, sans requête', async () => {
        const locals = await localsFor('theme=dark');
        expect(locals).toEqual({ memberSession: null, adminSession: null });
        expect(getSession).not.toHaveBeenCalled();
    });

    it('cookie invalide ou expiré : anonyme', async () => {
        getSession.mockResolvedValue(null);
        expect(await localsFor(COOKIE)).toEqual({ memberSession: null, adminSession: null });
    });

    it('compte désactivé : anonyme', async () => {
        getSession.mockResolvedValue(session({ role: 'member', active: false }));
        expect(await localsFor(COOKIE)).toEqual({ memberSession: null, adminSession: null });
    });

    it('membre actif : session membre identifiée par id, pas de console', async () => {
        getSession.mockResolvedValue(session({ role: 'member', active: true }));
        isSessionElevated.mockResolvedValue(true);
        const locals = await localsFor(COOKIE);
        expect(locals.memberSession).toEqual({
            pseudo: 'Alice', role: 'member', userId: 'u1', sessionId: 's1',
            access: { songsurf: true, jellyfin: false },
        });
        expect(locals.adminSession).toBeNull();
        expect(isSessionElevated).not.toHaveBeenCalled();
    });

    it('admin sans clé validée : session membre seulement', async () => {
        getSession.mockResolvedValue(session({ role: 'admin', active: true }));
        isSessionElevated.mockResolvedValue(false);
        const locals = await localsFor(COOKIE);
        expect(locals.memberSession?.role).toBe('admin');
        expect(locals.adminSession).toBeNull();
        expect(isSessionElevated).toHaveBeenCalledWith('s1');
    });

    it('admin avec clé validée dans cette session : console ouverte', async () => {
        getSession.mockResolvedValue(session({ role: 'admin', active: true }));
        isSessionElevated.mockResolvedValue(true);
        expect((await localsFor(COOKIE)).adminSession).toEqual({ pseudo: 'Alice' });
    });

    it('admin désactivé, même avec clé validée : anonyme', async () => {
        getSession.mockResolvedValue(session({ role: 'admin', active: false }));
        isSessionElevated.mockResolvedValue(true);
        expect(await localsFor(COOKIE)).toEqual({ memberSession: null, adminSession: null });
    });
});
