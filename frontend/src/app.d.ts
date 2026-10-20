declare global {
    namespace App {
        interface Locals {
            // Session BetterAuth (ba_sessions) — pseudo = pseudo affiché (ba_users.name),
            // access = services accordés par l'admin (lus par /auth/verify)
            memberSession: {
                pseudo: string; role: string; userId: string; sessionId: string;
                access: { songsurf: boolean; jellyfin: boolean };
            } | null;
            // Console admin : rôle admin + clé de sécurité validée dans cette session
            adminSession: { pseudo: string } | null;
        }
    }
}

export {};
