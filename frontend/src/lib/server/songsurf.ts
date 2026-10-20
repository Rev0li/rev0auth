import { SignJWT } from 'jose';
import { cookieDomain } from './auth-v2.js';

// Secret partagé avec SongSurf : AUTH_JWT_SECRET — la même valeur que dans
// SongSurf/.secrets et côté Rust. SONGSURF_JWT_SECRET reste accepté en fallback
// legacy ; à retirer une fois les .env nettoyés.
export function songsurfSecret(): string {
    return (process.env.AUTH_JWT_SECRET ?? process.env.SONGSURF_JWT_SECRET ?? '').trim();
}

export function songsurfBaseUrl(): string {
    return (process.env.SONGSURF_URL ?? '').trim().replace(/\/+$/, '');
}

export function songsurfConfigured(): boolean {
    return !!songsurfSecret() && !!songsurfBaseUrl();
}

export async function signSongsurfJwt(sub: string, role: string, ttlSeconds: number): Promise<string> {
    const now = Math.floor(Date.now() / 1000);
    return new SignJWT({ sub, role, email: '', token_type: 'access' })
        .setProtectedHeader({ alg: 'HS256' })
        .setIssuedAt(now)
        .setExpirationTime(now + ttlSeconds)
        .sign(new TextEncoder().encode(songsurfSecret()));
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
