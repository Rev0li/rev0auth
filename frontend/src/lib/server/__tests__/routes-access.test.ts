import { describe, it, expect, vi, beforeAll } from 'vitest';
import { isHttpError, isRedirect } from '@sveltejs/kit';

// Matrice route × rôle (AUTH-05). Chaque handler serveur est appelé avec les
// `locals` que hooks.server.ts poserait pour chaque profil, sans base de
// données : tout accès à la base ou à BetterAuth lève PASSED, ce qui prouve
// que le handler a franchi son contrôle d'accès. Un refus = 401/403 (levé ou
// renvoyé) ou une redirection 302/303 vers une page de connexion.

const { PASSED, trap } = vi.hoisted(() => {
    const PASSED = new Error('GUARD_PASSED');
    // Proxy appelable : toute propriété renvoie le proxy, tout appel lève PASSED
    // (y compris `then`, donc un `await` sur la chaîne rejette avec PASSED).
    const trap: unknown = new Proxy(function () {}, {
        get: () => trap,
        apply: () => { throw PASSED; },
    });
    return { PASSED, trap };
});

vi.mock('$lib/server/db/index.js', () => ({ db: trap, migrateDb: async () => {} }));
vi.mock('$lib/server/auth-v2.js', () => ({ auth: trap, cookieDomain: undefined }));

type Level = 'public' | 'member' | 'adminMember' | 'admin' | 'secret' | 'own';
type Profile = 'anonyme' | 'guest' | 'member' | 'mod' | 'admin sans clé' | 'admin';

const PROFILES: Record<Profile, App.Locals> = {
    'anonyme':        { memberSession: null, adminSession: null },
    'guest':          { memberSession: member('guest'), adminSession: null },
    'member':         { memberSession: member('member'), adminSession: null },
    'mod':            { memberSession: member('mod'), adminSession: null },
    'admin sans clé': { memberSession: member('admin'), adminSession: null },
    'admin':          { memberSession: member('admin'), adminSession: { pseudo: 'Revo' } },
};

// Qui passe le contrôle, par niveau. `secret` : seul l'en-tête partagé du NAS
// ouvre la route, aucune session ne suffit.
const ALLOWED: Record<Level, Profile[]> = {
    public:      ['anonyme', 'guest', 'member', 'mod', 'admin sans clé', 'admin'],
    member:      ['guest', 'member', 'mod', 'admin sans clé', 'admin'],
    adminMember: ['admin sans clé', 'admin'],
    admin:       ['admin'],
    secret:      [],
    own:         [], // règle propre, testée à part
};

type Entry = { level: Level; body?: unknown };

// Clé : `<route> <export>`. Toute nouvelle route doit être ajoutée ici
// (le test « toutes les routes sont couvertes » échoue sinon).
const MATRIX: Record<string, Entry> = {
    // Public
    '/ load':                          { level: 'public' },
    '/portal GET':                     { level: 'public' },
    '/signup load':                    { level: 'public' },
    '/signup POST':                    { level: 'public' },
    '/auth/password-check POST':       { level: 'public' },
    '/auth/logout POST':               { level: 'public' },
    '/avatars/[seed] GET':             { level: 'public' },
    '/members/dashboard load':         { level: 'public' }, // redirection permanente
    // Porte des services (Caddy forward_auth) : cf. forward-auth.test.ts
    '/auth/verify GET':                { level: 'own' },
    // Espace membre
    '/home/friend load':               { level: 'member' },
    '/members/password PUT':           { level: 'member' },
    '/members/avatar POST':            { level: 'member' },
    '/members/avatar DELETE':          { level: 'member' },
    '/members/avatar/[pseudo] GET':    { level: 'member' },
    '/members/account DELETE':         { level: 'member' },
    // Rôle admin, clé pas encore validée (entrée dans la console)
    '/japprends/login load':                     { level: 'adminMember' },
    '/japprends/passkey/auth-options POST':      { level: 'adminMember' },
    '/japprends/passkey/auth-verify POST':       { level: 'adminMember' },
    '/japprends/passkey/register-options POST':  { level: 'adminMember', body: { code: '123456' } },
    '/japprends/passkey/register-verify POST':   { level: 'adminMember' },
    // Console admin (rôle admin + clé validée)
    '/status GET':                               { level: 'admin' },
    '/japprends/dashboard load':                 { level: 'admin' },
    '/japprends/audit load':                     { level: 'admin' },
    '/japprends/audit/data GET':                 { level: 'admin' },
    '/japprends/songsurf-activity load':         { level: 'admin' },
    '/japprends/songsurf-logs load':             { level: 'admin' },
    '/japprends/songsurf-logs GET':              { level: 'admin' },
    '/japprends/passkey GET':                    { level: 'admin' },
    '/japprends/passkey/[id] DELETE':            { level: 'admin' },
    '/japprends/users GET':                      { level: 'admin' },
    '/japprends/users POST':                     { level: 'admin' },
    '/japprends/users/[pseudo] PUT':             { level: 'admin' },
    '/japprends/users/[pseudo] DELETE':          { level: 'admin' },
    '/japprends/users/[pseudo]/password POST':   { level: 'admin' },
    '/japprends/users/[pseudo]/password DELETE': { level: 'admin' },
    '/japprends/invites GET':                    { level: 'admin' },
    '/japprends/invites POST':                   { level: 'admin' },
    '/japprends/invites/[id] DELETE':            { level: 'admin' },
    '/japprends/signup-requests GET':            { level: 'admin' },
    '/japprends/signup-requests POST':           { level: 'admin' },
    '/japprends/signup-requests/[id]/approve POST': { level: 'admin' },
    '/japprends/signup-requests/[id]/reject POST':  { level: 'admin' },
    // Machine à machine
    '/japprends/api/songsurf-events POST':       { level: 'secret' },
};

type Handler = (event: unknown) => unknown;
const modules = import.meta.glob<Record<string, unknown>>('/src/routes/**/+{server,page.server}.ts');
const HANDLER_EXPORTS = ['GET', 'POST', 'PUT', 'PATCH', 'DELETE', 'load'];

function routeOf(file: string): string {
    const dir = file.replace(/^\/src\/routes/, '').replace(/\/\+(server|page\.server)\.ts$/, '');
    return dir || '/';
}

function member(role: string): NonNullable<App.Locals['memberSession']> {
    return { pseudo: 'Alice', role, userId: 'u1', sessionId: 's1' };
}

function buildEvent(route: string, method: string, locals: App.Locals, body: unknown) {
    const url = new URL(`http://localhost${route.replace('[seed]', 'abc').replace(/\[\w+\]/g, '1')}`);
    const httpMethod = method === 'load' ? 'GET' : method;
    const request = new Request(url, {
        method: httpMethod,
        headers: { 'content-type': 'application/json', 'x-forwarded-for': '10.0.0.1' },
        body: httpMethod === 'GET' ? undefined : JSON.stringify(body ?? {}),
    });
    return {
        locals, request, url,
        params: { id: '1', pseudo: 'alice', seed: 'abc' },
        cookies: { get: () => undefined, getAll: () => [], set: () => {}, delete: () => {} },
        getClientAddress: () => '10.0.0.1',
        setHeaders: () => {},
        fetch: () => { throw PASSED; },
    };
}

const LOGIN_PAGES = ['/', '/japprends/login'];

async function outcome(handler: Handler, event: unknown): Promise<'allowed' | 'refused'> {
    try {
        const res = await handler(event);
        if (res instanceof Response && (res.status === 401 || res.status === 403)) return 'refused';
        return 'allowed';
    } catch (e) {
        if (e === PASSED) return 'allowed';
        if (isHttpError(e)) return e.status === 401 || e.status === 403 ? 'refused' : 'allowed';
        if (isRedirect(e)) {
            // Absolue quand la connexion est sur le portail public (AUTH-11)
            return (e.status === 302 || e.status === 303) && LOGIN_PAGES.includes(new URL(e.location, 'http://localhost').pathname)
                ? 'refused' : 'allowed';
        }
        throw e;
    }
}

const handlers = new Map<string, Handler>();

beforeAll(async () => {
    // Avec le secret configuré, la route NAS répond 401 (et non 503) sans en-tête
    process.env.SONGSURF_EVENTS_SECRET = 'secret-de-test';
    vi.stubGlobal('fetch', () => { throw PASSED; });
    for (const [file, load] of Object.entries(modules)) {
        const mod = await load();
        for (const name of HANDLER_EXPORTS) {
            if (typeof mod[name] === 'function') handlers.set(`${routeOf(file)} ${name}`, mod[name] as Handler);
        }
    }
});

describe('droits d\'accès par route', () => {
    it('toutes les routes serveur sont dans la matrice', () => {
        const missing = [...handlers.keys()].filter((k) => !(k in MATRIX));
        const stale   = Object.keys(MATRIX).filter((k) => !handlers.has(k));
        expect({ missing, stale }).toEqual({ missing: [], stale: [] });
    });

    for (const [key, { level, body }] of Object.entries(MATRIX)) {
        if (level === 'own') continue;
        const [route, method] = key.split(' ');
        for (const profile of Object.keys(PROFILES) as Profile[]) {
            const expected = ALLOWED[level].includes(profile) ? 'allowed' : 'refused';
            it(`${key} — ${profile} → ${expected === 'allowed' ? 'passe' : 'refusé'}`, async () => {
                const handler = handlers.get(key);
                expect(handler, `${key} introuvable`).toBeDefined();
                const event = buildEvent(route, method, structuredClone(PROFILES[profile]), body);
                expect(await outcome(handler!, event)).toBe(expected);
            });
        }
    }
});
