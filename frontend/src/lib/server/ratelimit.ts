// In-memory rate limiter (per IP). Resets on server restart — acceptable for admin login.
const MAX_ATTEMPTS  = 5;
const LOCKOUT_MS    = 15 * 60 * 1000; // 15 min

interface Entry { count: number; lockedUntil: number; }
const store = new Map<string, Entry>();

export function checkRateLimit(ip: string): { blocked: boolean } {
    const now  = Date.now();
    const entry = store.get(ip);
    if (entry && now < entry.lockedUntil) return { blocked: true };
    return { blocked: false };
}

export function recordFailure(ip: string): void {
    const now   = Date.now();
    const entry = store.get(ip) ?? { count: 0, lockedUntil: 0 };
    entry.count += 1;
    if (entry.count >= MAX_ATTEMPTS) entry.lockedUntil = now + LOCKOUT_MS;
    store.set(ip, entry);
}

export function clearAttempts(ip: string): void {
    store.delete(ip);
}

export function getIp(request: Request): string {
    return (
        request.headers.get('x-forwarded-for')?.split(',')[0]?.trim() ??
        request.headers.get('x-real-ip') ??
        'unknown'
    );
}
