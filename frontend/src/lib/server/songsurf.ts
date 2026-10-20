import { cookieDomain } from './auth-v2.js';

// SongSurf est derrière la porte du portail (AUTH-09) : on y entre avec la
// session du portail, sans jeton.
export function songsurfBaseUrl(): string {
    return (process.env.SONGSURF_URL ?? '').trim().replace(/\/+$/, '');
}

export function songsurfConfigured(): boolean {
    return !!songsurfBaseUrl();
}

// Les appels du portail vers SongSurf passent par la porte (forward_auth) :
// ils portent la session de l'admin, comme le ferait son navigateur. Seul le
// cookie de session est repris, et seulement vers le domaine du portail.
const SESSION_COOKIE = /^(__Secure-)?better-auth\.session_token$/;

export function portalSessionCookies(cookieHeader: string | null, target: string): string[] {
    if (!cookieHeader || !cookieDomain) return [];
    let host: string;
    try {
        const url = new URL(target);
        if (url.protocol !== 'https:') return [];
        host = url.hostname;
    } catch {
        return [];
    }
    if (host !== cookieDomain && !host.endsWith(`.${cookieDomain}`)) return [];
    return cookieHeader.split(';').map((c) => c.trim())
        .filter((c) => SESSION_COOKIE.test(c.split('=')[0]) && c.split('=')[1]);
}

// Journal des téléchargements SongSurf, demandé avec la session de l'admin.
export async function fetchSongsurfDlLogs(
    cookieHeader: string | null, pseudo: string, limit: number, timeoutMs: number,
): Promise<Response> {
    const target = `${songsurfBaseUrl()}/api/admin/dl-logs?pseudo=${encodeURIComponent(pseudo)}&limit=${limit}`;
    const ctrl = new AbortController();
    const timer = setTimeout(() => ctrl.abort(), timeoutMs);
    try {
        return await fetch(target, {
            headers: { Cookie: portalSessionCookies(cookieHeader, target).join('; ') },
            signal: ctrl.signal,
        });
    } finally {
        clearTimeout(timer);
    }
}
