import { describe, it, expect, vi, afterEach } from 'vitest';

// AUTH-07 : en prod, le cookie de session vaut pour le portail et tous ses
// sous-domaines ; en dev (http://localhost), il reste lié à l'hôte.

const ORIGINAL_ORIGIN = process.env.ORIGIN;

async function sessionCookie(origin: string) {
    vi.resetModules();
    process.env.ORIGIN = origin;
    const { auth } = await import('../auth-v2.js');
    return (await auth.$context).authCookies.sessionToken;
}

afterEach(() => {
    process.env.ORIGIN = ORIGINAL_ORIGIN;
    vi.resetModules();
});

describe('cookie de session BetterAuth', () => {
    it('prod : domaine parent, Secure, HttpOnly, SameSite=Lax', async () => {
        const cookie = await sessionCookie('https://rev0li.duckdns.org');
        expect(cookie.name).toBe('__Secure-better-auth.session_token');
        expect(cookie.attributes).toMatchObject({
            domain: 'rev0li.duckdns.org',
            secure: true,
            httpOnly: true,
            sameSite: 'lax',
            path: '/',
        });
    });

    it('dev : lié à l\'hôte, sans préfixe __Secure-', async () => {
        const cookie = await sessionCookie('http://localhost:5173');
        expect(cookie.name).toBe('better-auth.session_token');
        expect(cookie.attributes.domain).toBeUndefined();
        expect(cookie.attributes.httpOnly).toBe(true);
    });
});

describe('déconnexion', () => {
    async function logout(cookie: string) {
        vi.resetModules();
        vi.doMock('$lib/server/auth-v2.js', () => ({
            cookieDomain: 'rev0li.duckdns.org',
            auth: { api: { signOut: async () => ({
                headers: new Headers([['set-cookie',
                    '__Secure-better-auth.session_token=; Max-Age=0; Domain=rev0li.duckdns.org; Path=/; HttpOnly; Secure; SameSite=Lax']]),
            }) } },
        }));
        const { POST } = await import('../../../routes/auth/logout/+server.js');
        const res = await POST({ request: new Request('http://localhost/auth/logout', {
            method: 'POST', headers: { cookie },
        }) } as never) as Response;
        vi.doUnmock('$lib/server/auth-v2.js');
        return res.headers.getSetCookie();
    }

    it('efface le cookie du domaine parent et l\'ancien cookie lié à l\'hôte', async () => {
        const cookies = await logout('theme=dark; __Secure-better-auth.session_token=abc');
        expect(cookies).toHaveLength(2);
        expect(cookies[0]).toContain('Domain=rev0li.duckdns.org');
        expect(cookies[1]).toMatch(/^__Secure-better-auth\.session_token=; Path=\/; Max-Age=0; HttpOnly; SameSite=Lax; Secure$/);
    });

    it('n\'efface rien de plus sans cookie de session', async () => {
        expect(await logout('theme=dark')).toHaveLength(1);
    });
});
