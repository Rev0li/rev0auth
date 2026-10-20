import { describe, it, expect, vi } from 'vitest';

// AUTH-09 : les appels du portail vers SongSurf passent la porte avec la
// session de l'admin.

vi.mock('$lib/server/auth-v2.js', () => ({ cookieDomain: 'rev0li.duckdns.org', auth: {} }));

const { portalSessionCookies } = await import('../songsurf.js');

const TARGET = 'https://songsurf.rev0li.duckdns.org/api/admin/dl-logs';
const SENT = 'theme=dark; __Secure-better-auth.session_token=abc.sig; better-auth.session_data=xyz';

describe('portalSessionCookies', () => {
    it('ne reprend que le cookie de session', () => {
        expect(portalSessionCookies(SENT, TARGET)).toEqual(['__Secure-better-auth.session_token=abc.sig']);
    });

    it('reprend aussi l\'ancien cookie sans préfixe', () => {
        expect(portalSessionCookies('better-auth.session_token=old', TARGET)).toEqual(['better-auth.session_token=old']);
    });

    it.each([
        'https://evil.com/api',
        'https://rev0li.duckdns.org.evil.com/',
        'http://songsurf.rev0li.duckdns.org/',
        'http://100.64.0.1:9000/',
        'pas une adresse',
    ])('n\'envoie rien vers %s', (target) => {
        expect(portalSessionCookies(SENT, target)).toEqual([]);
    });

    it('rien sans cookie ou avec une valeur vide', () => {
        expect(portalSessionCookies(null, TARGET)).toEqual([]);
        expect(portalSessionCookies('__Secure-better-auth.session_token=', TARGET)).toEqual([]);
    });
});

describe('GET /japprends/songsurf-logs', () => {
    it('transmet la session de l\'admin, sans jeton', async () => {
        process.env.SONGSURF_URL = 'https://songsurf.rev0li.duckdns.org';
        const fetchMock = vi.fn(async () => new Response('[]', { status: 200 }));
        vi.stubGlobal('fetch', fetchMock);
        const { GET } = await import('../../../routes/japprends/songsurf-logs/+server.js');

        const url = new URL('http://localhost/japprends/songsurf-logs?pseudo=Alice');
        const res = await GET({
            url,
            request: new Request(url, { headers: { cookie: SENT } }),
            locals: { adminSession: { pseudo: 'Revo' } },
        } as never) as Response;

        expect(res.status).toBe(200);
        const [target, init] = fetchMock.mock.calls[0] as unknown as [string, RequestInit];
        expect(target).toBe(`${TARGET}?pseudo=Alice&limit=100`);
        const cookie = (init.headers as Record<string, string>).Cookie;
        expect(cookie).toBe('__Secure-better-auth.session_token=abc.sig');
        vi.unstubAllGlobals();
    });
});
