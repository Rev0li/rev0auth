declare global {
    namespace App {
        interface Locals {
            // Session BetterAuth (ba_sessions) — pseudo = web_users.pseudo exact
            memberSession: { pseudo: string; role: string; userId: string; sessionId: string } | null;
            // Console admin : rôle admin + clé de sécurité validée dans cette session
            adminSession: { pseudo: string } | null;
        }
    }
}

export {};
