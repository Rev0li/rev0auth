const API_UPSTREAM = (process.env.REV0AUTH_API_UPSTREAM ?? '').trim() || '127.0.0.1:8080';

export async function checkApiUp(timeoutMs = 500): Promise<boolean> {
    try {
        const ctrl = new AbortController();
        const timer = setTimeout(() => ctrl.abort(), timeoutMs);
        const res = await fetch(`http://${API_UPSTREAM}/health`, { signal: ctrl.signal });
        clearTimeout(timer);
        return res.ok;
    } catch {
        return false;
    }
}
