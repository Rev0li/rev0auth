import type { LayoutServerLoad } from './$types.js';
import { portalUrl } from '$lib/server/console-host.js';

// Cadre de la console (barre + onglets), seulement une fois la clé validée :
// la page d'entrée (/japprends/login) reste nue. Chaque page garde son propre
// contrôle d'accès ; ce layout ne fait que l'habillage.
export const load: LayoutServerLoad = async ({ locals }) => ({
    shell: locals.adminSession
        ? {
            pseudo: locals.adminSession.pseudo,
            // Retour à l'espace membre : sur le portail public (même session)
            memberUrl: portalUrl('/home/friend'),
            avatarUrl: portalUrl(`/members/avatar/${encodeURIComponent(locals.adminSession.pseudo)}`),
        }
        : null,
});
