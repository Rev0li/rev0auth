import { cookieDomain } from './auth-v2.js';

// Porte commune des services (AUTH-08) : Caddy appelle /auth/verify avant
// chaque requête vers un sous-domaine protégé (forward_auth). Le service est
// fixé dans la config Caddy (`?service=`), jamais déduit de la requête.

export const SERVICES = ['songsurf', 'jellyfin'] as const;
export type Service = (typeof SERVICES)[number];

export function isService(value: string | null): value is Service {
    return SERVICES.includes(value as Service);
}

// Adresse de retour après connexion : https, sur le domaine du portail ou un
// de ses sous-domaines. Tout le reste est refusé (pas de redirection ouverte).
export function safeReturnTo(raw: string | null): string | null {
    if (!raw || !cookieDomain) return null;
    let url: URL;
    try {
        url = new URL(raw);
    } catch {
        return null;
    }
    // Sans port ni identifiants : l'adresse commence alors par https://<hôte>/
    if (url.protocol !== 'https:' || url.port || !url.href.startsWith(`https://${url.host}/`)) return null;
    const host = url.hostname;
    if (host !== cookieDomain && !host.endsWith(`.${cookieDomain}`)) return null;
    return url.href;
}

// Adresse demandée par le navigateur, reconstruite depuis les en-têtes posés
// par Caddy (il remplace ceux envoyés par le client).
export function requestedUrl(headers: Headers): string | null {
    const host = headers.get('x-forwarded-host');
    const uri  = headers.get('x-forwarded-uri') ?? '/';
    if (!host || !uri.startsWith('/')) return null;
    return safeReturnTo(`https://${host}${uri}`);
}

export function loginUrl(returnTo: string | null): string {
    const origin = process.env.ORIGIN ?? 'http://localhost:5173';
    return returnTo ? `${origin}/?redirect=${encodeURIComponent(returnTo)}` : `${origin}/`;
}
