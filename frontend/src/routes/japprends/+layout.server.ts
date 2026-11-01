import type { LayoutServerLoad } from './$types.js';
import { portalUrl } from '$lib/server/console-host.js';

// Cadre de la console (barre + onglets), seulement une fois la clé validée :
// la page d'entrée (/japprends/login) reste nue. Chaque page garde son propre
// contrôle d'accès ; ce layout ne fait que l'habillage.
export const load: LayoutServerLoad = async ({ locals }) => ({
    shell: locals.adminSession
        ? {
            pseudo: locals.adminSession.pseudo,
            // Retour à l'espace membre : servi aussi sous le nom de la console
            // (AUTH-29), donc lien relatif = même site, sans rechargement
            memberUrl: '/home/friend',
            avatarUrl: portalUrl(`/members/avatar/${encodeURIComponent(locals.adminSession.pseudo)}`),
        }
        : null,
});
