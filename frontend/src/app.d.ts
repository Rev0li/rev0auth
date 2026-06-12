import type { Session } from '$lib/server/db/schema.js';

declare global {
    namespace App {
        interface Locals {
            adminSession:  Session | null;
            // Session BetterAuth (ba_sessions) — pseudo = web_users.pseudo exact
            memberSession: { pseudo: string; role: string } | null;
        }
    }
}

export {};
