declare global {
    namespace App {
        interface Locals {
            // Session BetterAuth (ba_sessions) — pseudo = pseudo affiché (ba_users.name)
            memberSession: { pseudo: string; role: string; userId: string; sessionId: string } | null;
            // Console admin : rôle admin + clé de sécurité validée dans cette session
            adminSession: { pseudo: string } | null;
        }
    }
}

export {};
