import { pgTable, text, bigint, bigserial, timestamp, customType, index } from 'drizzle-orm/pg-core';
import { sql } from 'drizzle-orm';
import { ba_users } from './auth-schema.js';

// Défaut SQL des colonnes *_epoch (secondes Unix), en plus du $defaultFn côté JS
const nowEpoch = sql`EXTRACT(EPOCH FROM NOW())::BIGINT`;

// Postgres BYTEA mapped to Node Buffer
const bytea = customType<{ data: Buffer; driverData: unknown }>({
    dataType() { return 'bytea'; },
    toDriver(val: Buffer) { return val; },
    fromDriver(val: unknown): Buffer {
        if (Buffer.isBuffer(val)) return val;
        if (typeof val === 'string') return Buffer.from(val.replace(/^\\x/, ''), 'hex');
        return Buffer.from(val as ArrayBuffer);
    },
});

// ─── web_invites (shared with Rust web crate) ────────────────────────────────

export const invites = pgTable('web_invites', {
    id:            bigserial('id', { mode: 'number' }).primaryKey(),
    code:          text('code').notNull().unique(),
    note:          text('note').notNull().default(''),
    createdAt:     bigint('created_at_epoch', { mode: 'number' }).notNull(),
    expiresAt:     bigint('expires_at_epoch', { mode: 'number' }).notNull(),
    usedBy:        text('used_by'),
    usedAt:        bigint('used_at_epoch', { mode: 'number' }),
});

// ─── web_audit_log (SvelteKit-specific) ────────────────

export const auditLog = pgTable('web_audit_log', {
    id:          bigserial('id', { mode: 'number' }).primaryKey(),
    action:      text('action').notNull(),
    actorPseudo: text('actor_pseudo').notNull(),
    target:      text('target').notNull().default(''),
    detail:      text('detail').notNull().default(''),
    createdAt:   bigint('created_at_epoch', { mode: 'number' }).notNull().default(nowEpoch).$defaultFn(() => Math.floor(Date.now() / 1000)),
}, (t) => [index('web_audit_log_created_idx').on(t.createdAt.desc())]);

// ─── songsurf_events (SvelteKit-specific) ──────────────
// Événements poussés par le NAS (watcher + songsurf) via /japprends/api/songsurf-events.
// detail = JSON string (convention cases TEXT). Tri/affichage sur receivedAt (horloge VPS).

export const songsurfEvents = pgTable('songsurf_events', {
    id:         bigserial('id', { mode: 'number' }).primaryKey(),
    source:     text('source').notNull().default(''),
    eventType:  text('event_type').notNull(),
    eventTs:    bigint('event_ts_epoch', { mode: 'number' }).notNull(),
    pseudo:     text('pseudo').notNull().default(''),
    role:       text('role').notNull().default(''),
    artist:     text('artist').notNull().default(''),
    album:      text('album').notNull().default(''),
    title:      text('title').notNull().default(''),
    detail:     text('detail').notNull().default('{}'),
    ip:         text('ip').notNull().default(''),
    receivedAt: bigint('received_at_epoch', { mode: 'number' }).notNull().default(nowEpoch).$defaultFn(() => Math.floor(Date.now() / 1000)),
}, (t) => [
    index('songsurf_events_received_idx').on(t.receivedAt.desc()),
    index('songsurf_events_pseudo_idx').on(t.pseudo, t.receivedAt.desc()),
    index('songsurf_events_type_idx').on(t.eventType, t.receivedAt.desc()),
]);

// ─── user_avatars ────────────────────────────────────────────────────────────
// Avatar SVG (DiceBear) généré côté serveur, à part de ba_users pour ne pas
// charger l'image à chaque lecture de session. filename = choix du composeur.

export const userAvatars = pgTable('user_avatars', {
    userId:    text('user_id').primaryKey().references(() => ba_users.id, { onDelete: 'cascade' }),
    bytes:     bytea('bytes').notNull(),
    mimeType:  text('mime_type').notNull(),
    filename:  text('filename'),
    sizeBytes: bigint('size_bytes', { mode: 'number' }),
    updatedAt: timestamp('updated_at').notNull().defaultNow(),
});

// ─── admin_passkeys ──────────────────────────────────────────────────────────
// Clés de sécurité (WebAuthn) exigées pour entrer dans la console admin,
// rattachées à l'identité BetterAuth. publicKey = clé COSE en base64url.

export const adminPasskeys = pgTable('admin_passkeys', {
    id:           bigserial('id', { mode: 'number' }).primaryKey(),
    userId:       text('user_id').notNull().references(() => ba_users.id, { onDelete: 'cascade' }),
    credentialId: text('credential_id').notNull().unique(),
    publicKey:    text('public_key').notNull(),
    counter:      bigint('counter', { mode: 'number' }).notNull().default(0),
    transports:   text('transports').notNull().default(''),
    name:         text('name').notNull().default(''),
    createdAt:    timestamp('created_at').notNull().defaultNow(),
    lastUsedAt:   timestamp('last_used_at'),
}, (t) => [index('admin_passkeys_user_idx').on(t.userId)]);

// ─── Inferred types ───────────────────────────────────────────────────────────

export type Invite     = typeof invites.$inferSelect;
export type AuditEntry = typeof auditLog.$inferSelect;
export type AdminPasskey = typeof adminPasskeys.$inferSelect;
export type SongsurfEvent    = typeof songsurfEvents.$inferSelect;
export type NewSongsurfEvent = typeof songsurfEvents.$inferInsert;
