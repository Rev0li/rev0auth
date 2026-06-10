import { SignJWT } from 'jose';

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
