// Console admin servie sous son propre nom (AUTH-11, ADMIN_ORIGIN), que Caddy
// ne laisse joindre que depuis le réseau privé (Tailscale). La même appli sert
// les deux noms : ce module décide, selon l'hôte demandé, ce que chacun montre.
// Même domaine parent : la session (cookie partagé, AUTH-07) et les clés de
// sécurité (RP ID = domaine du portail) valent sur les deux noms.
//
// Sans ADMIN_ORIGIN (dev), pas de séparation : la console reste sous /japprends.

// Seule route de la console appelée depuis Internet : le NAS y pousse les
// évènements SongSurf, authentifiés par secret partagé (SONGSURF_EVENTS_SECRET).
export const SONGSURF_EVENTS_PATH = '/japprends/api/songsurf-events';

function portalOrigin(): string {
    return process.env.ORIGIN ?? 'http://localhost:5173';
}

function consoleOrigin(): string | null {
    return process.env.ADMIN_ORIGIN || null;
}

export function isConsolePath(pathname: string): boolean {
    return pathname === '/japprends' || pathname.startsWith('/japprends/');
}

// Adresse d'une page de la console, absolue dès qu'elle a son propre nom
export function consoleUrl(path: string): string {
    return `${consoleOrigin() ?? ''}${path}`;
}

// Adresse d'une page du portail public (liens d'invitation…)
export function portalUrl(path: string): string {
    return `${portalOrigin()}${path}`;
}

// Origine attendue par WebAuthn pour les clés de la console
export function consoleWebAuthnOrigin(): string {
    return consoleOrigin() ?? portalOrigin();
}

export type Gate =
    | { kind: 'pass' }
    | { kind: 'not-found' }
    | { kind: 'redirect'; location: string };

// `host` = en-tête Host (posé par Caddy d'après le nom demandé ; ORIGIN fige
// event.url, il ne dit donc pas quel nom a été demandé).
export function gateRequest(host: string | null, pathname: string, search: string): Gate {
    const console_ = consoleOrigin();
    if (!console_) return { kind: 'pass' };

    if (host !== new URL(console_).host) {
        // Portail public (ou tout autre nom) : la console n'existe pas
        return isConsolePath(pathname) && pathname !== SONGSURF_EVENTS_PATH
            ? { kind: 'not-found' }
            : { kind: 'pass' };
    }

    // Nom de la console : la console, ses fichiers et la déconnexion ; la
    // racine mène à l'entrée de la console, le reste au portail public.
    if (isConsolePath(pathname) || pathname.startsWith('/_app/') || pathname === '/auth/logout') {
        return { kind: 'pass' };
    }
    if (pathname === '/') return { kind: 'redirect', location: consoleUrl('/japprends/login') };
    return { kind: 'redirect', location: portalUrl(`${pathname}${search}`) };
}
