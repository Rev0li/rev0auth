import { db } from './db/index.js';
import { auditLog } from './db/schema.js';

export async function writeAudit(
    action: string,
    actorPseudo: string,
    target = '',
    detail = '',
): Promise<void> {
    try {
        await db.insert(auditLog).values({ action, actorPseudo, target, detail });
    } catch (e) {
        console.error('[audit] insert failed', e);
    }
}
