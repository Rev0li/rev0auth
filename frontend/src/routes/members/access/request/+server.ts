import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types.js';
import { db } from '$lib/server/db/index.js';
import { users } from '$lib/server/db/schema.js';
import { eq } from 'drizzle-orm';

// Demande d'accès à un service. Chaque service a sa condition :
//   songsurf → star sur github.com/Rev0li/SongSurf + pseudo GitHub
//   jellyfin → recommandation LinkedIn + nom de profil LinkedIn
// Le pseudo soumis est stocké sur web_users pour vérification par l'admin.

export const POST: RequestHandler = async ({ request, locals }) => {
    if (!locals.memberSession) throw error(401, 'Non autorisé.');
    const { service, github_username, linkedin_name } = await request.json();
    const pseudo = locals.memberSession.pseudo;

    if (service === 'songsurf') {
        const gh = github_username?.trim();
        if (!gh || !/^[a-zA-Z0-9-]{1,39}$/.test(gh)) {
            return json({ ok: false, message: 'Pseudo GitHub requis.' }, { status: 400 });
        }
        await db.update(users)
            .set({ requestSongsurf: true, githubUsername: gh })
            .where(eq(users.pseudo, pseudo));
        return json({ ok: true });
    }

    if (service === 'jellyfin') {
        const li = linkedin_name?.trim();
        if (!li || li.length > 100) {
            return json({ ok: false, message: 'Nom LinkedIn requis.' }, { status: 400 });
        }
        await db.update(users)
            .set({ requestJellyfin: true, linkedinName: li })
            .where(eq(users.pseudo, pseudo));
        return json({ ok: true });
    }

    return json({ ok: false, message: 'Service inconnu.' }, { status: 400 });
};
