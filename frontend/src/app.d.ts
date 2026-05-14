import type { Session } from '$lib/server/db/schema.js';

declare global {
    namespace App {
        interface Locals {
            adminSession:  Session | null;
            memberSession: Session | null;
        }
    }
}

export {};
