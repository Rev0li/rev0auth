import { describe, it, expect, vi, beforeAll } from 'vitest';

// AUTH-08 : porte commune des services (Caddy forward_auth → /auth/verify).

vi.mock('$lib/server/auth-v2.js', () => ({ cookieDomain: 'rev0li.duckdns.org', auth: {} }));

process.env.ORIGIN = 'https://rev0li.duckdns.org';
const { safeReturnTo, requestedUrl } = await import('../forward-auth.js');
const { GET } = await import('../../../routes/auth/verify/+server.js');

type Access = { songsurf: boolean; jellyfin: boolean };

function member(role: string, access: Partial<Access> = {}): App.Locals['memberSession'] {
    return {
        pseudo: 'Alice', role, userId: 'u1', sessionId: 's1',
        access: { songsurf: false, jellyfin: false, ...access },
    };
}

// Requête telle que Caddy la transmet : en-têtes du navigateur + X-Forwarded-*
function verify(service: string, memberSession: App.Locals['memberSession'], headers: Record<string, string> = {}) {
    const url = new URL(`http://localhost/auth/verify?service=${service}`);
    const request = new Request(url, {
        headers: {
            accept: 'text/html,application/xhtml+xml',
            'x-forwarded-method': 'GET',
            'x-forwarded-host': 'songsurf.rev0li.duckdns.org',
            'x-forwarded-uri': '/library?album=1',
            ...headers,
        },
    });
    return GET({ url, request, locals: { memberSession, adminSession: null } } as never) as Promise<Response>;
}

describe('safeReturnTo', () => {
    it.each([
        'https://songsurf.rev0li.duckdns.org/library?album=1',
        'https://rev0li.duckdns.org/home/friend',
        'https://a.b.rev0li.duckdns.org/',
    ])('accepte %s', (u) => expect(safeReturnTo(u)).toBe(new URL(u).href));

    it.each([
        null, '', '/home/friend', '//evil.com/', 'javascript:alert(1)',
        'http://songsurf.rev0li.duckdns.org/',
        'https://evil.com/',
        'https://rev0li.duckdns.org.evil.com/',
        'https://evilrev0li.duckdns.org/',
        'https://evil.com\\@songsurf.rev0li.duckdns.org/',
        'https://user:pw@songsurf.rev0li.duckdns.org/',
        'https://songsurf.rev0li.duckdns.org:8443/',
    ])('refuse %s', (u) => expect(safeReturnTo(u)).toBeNull());
});

describe('requestedUrl', () => {
    it('reconstruit l\'adresse demandée', () => {
        const h = new Headers({ 'x-forwarded-host': 'songsurf.rev0li.duckdns.org', 'x-forwarded-uri': '/a?b=1' });
        expect(requestedUrl(h)).toBe('https://songsurf.rev0li.duckdns.org/a?b=1');
    });

    it('ignore un hôte hors du domaine', () => {
        const h = new Headers({ 'x-forwarded-host': 'evil.com', 'x-forwarded-uri': '/' });
        expect(requestedUrl(h)).toBeNull();
    });
});

describe('GET /auth/verify', () => {
    beforeAll(() => { process.env.ORIGIN = 'https://rev0li.duckdns.org'; });

    it('service inconnu → 400', async () => {
        expect((await verify('admin', member('admin'))).status).toBe(400);
        expect((await verify('', member('admin'))).status).toBe(400);
    });

    it('anonyme qui navigue → connexion puis retour à l\'adresse demandée', async () => {
        const res = await verify('songsurf', null);
        expect(res.status).toBe(302);
        expect(res.headers.get('location')).toBe(
            'https://rev0li.duckdns.org/?redirect=' +
            encodeURIComponent('https://songsurf.rev0li.duckdns.org/library?album=1'),
        );
    });

    it('anonyme, hôte transmis hors domaine → connexion sans retour', async () => {
        const res = await verify('songsurf', null, { 'x-forwarded-host': 'evil.com' });
        expect(res.headers.get('location')).toBe('https://rev0li.duckdns.org/');
    });

    it('anonyme, appel d\'API → 401', async () => {
        expect((await verify('songsurf', null, { accept: 'application/json' })).status).toBe(401);
        expect((await verify('songsurf', null, { 'x-forwarded-method': 'POST' })).status).toBe(401);
    });

    it.each(['guest', 'member', 'mod'])('%s sans accès → 403, sans en-tête d\'identité', async (role) => {
        const res = await verify('songsurf', member(role, { jellyfin: true }));
        expect(res.status).toBe(403);
        expect(res.headers.get('x-user-id')).toBeNull();
    });

    it('membre avec accès → 204 + identité pour le service', async () => {
        const res = await verify('songsurf', member('member', { songsurf: true }));
        expect(res.status).toBe(204);
        expect(res.headers.get('x-user-id')).toBe('u1');
        expect(res.headers.get('x-user-name')).toBe('Alice');
        expect(res.headers.get('x-user-role')).toBe('member');
        expect(res.headers.get('cache-control')).toBe('no-store');
    });

    it('l\'accès est propre à chaque service', async () => {
        expect((await verify('jellyfin', member('member', { songsurf: true }))).status).toBe(403);
        expect((await verify('jellyfin', member('member', { jellyfin: true }))).status).toBe(204);
    });

    it('admin → accès à tous les services', async () => {
        expect((await verify('songsurf', member('admin'))).status).toBe(204);
        expect((await verify('jellyfin', member('admin'))).status).toBe(204);
    });
});
