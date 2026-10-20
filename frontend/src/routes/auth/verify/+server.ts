import type { RequestHandler } from './$types.js';
import { isService, canAccess, requestedUrl, loginUrl } from '$lib/server/forward-auth.js';

// Appelé par Caddy (forward_auth) avant chaque requête vers un service.
// 2xx → Caddy laisse passer et copie les en-têtes X-User-* vers le service ;
// toute autre réponse est renvoyée telle quelle au navigateur.
export const GET: RequestHandler = async ({ url, request, locals }) => {
    const service = url.searchParams.get('service');
    if (!isService(service)) return text(400, 'Service inconnu.');

    const member = locals.memberSession;
    if (!member) {
        // Navigation → page de connexion, puis retour à l'adresse demandée.
        // Appels d'API (fetch, extension) → 401, qu'ils savent traiter.
        const accept = request.headers.get('accept') ?? '';
        const method = request.headers.get('x-forwarded-method') ?? 'GET';
        if (method === 'GET' && accept.includes('text/html')) {
            return new Response(null, {
                status: 302,
                headers: { location: loginUrl(requestedUrl(request.headers)), 'cache-control': 'no-store' },
            });
        }
        return text(401, 'Connexion requise.');
    }

    if (!canAccess(member, service)) {
        return new Response(forbiddenPage(service), {
            status: 403,
            headers: { 'content-type': 'text/html; charset=utf-8', 'cache-control': 'no-store' },
        });
    }

    return new Response(null, {
        status: 204,
        headers: {
            'x-user-id':     member.userId,
            'x-user-name':   member.pseudo,
            'x-user-role':   member.role,
            'cache-control': 'no-store',
        },
    });
};

function text(status: number, message: string): Response {
    return new Response(message, {
        status,
        headers: { 'content-type': 'text/plain; charset=utf-8', 'cache-control': 'no-store' },
    });
}

function forbiddenPage(service: string): string {
    return `<!doctype html><html lang="fr"><meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Accès refusé</title>
<body style="font-family:system-ui,sans-serif;max-width:32rem;margin:15vh auto;padding:0 1rem">
<h1>Pas d'accès à ${service}</h1>
<p>Ton compte n'a pas encore accès à ce service. Tu peux le demander depuis ton espace.</p>
<p><a href="${loginUrl(null)}home/friend">Aller à mon espace</a></p>
</body></html>`;
}
