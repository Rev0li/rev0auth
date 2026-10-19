import { pgTable, text, boolean, bigint, bigserial, customType, index } from 'drizzle-orm/pg-core';
import { sql } from 'drizzle-orm';

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

// ─── web_users (shared with Rust web crate) ───────────────────────────────────

export const users = pgTable('web_users', {
    pseudo:            text('pseudo').primaryKey(),
    role:              text('role').notNull().default('member'),
    active:            boolean('active').notNull().default(true),
    status:            text('status').notNull().default('offline'),
    bio:               text('bio').notNull().default(''),
    accessJellyfin:    boolean('access_jellyfin').notNull().default(false),
    accessSongsurf:    boolean('access_songsurf').notNull().default(false),
    requestJellyfin:   boolean('request_jellyfin').notNull().default(false),
    requestSongsurf:   boolean('request_songsurf').notNull().default(false),
    githubUsername:    text('github_username'),
    linkedinName:      text('linkedin_name'),
    avatarFilename:    text('avatar_filename'),
    avatarSizeBytes:   bigint('avatar_size_bytes', { mode: 'number' }),
    avatarMime:        text('avatar_mime_type'),
    avatarBytes:       bytea('avatar_bytes'),
    passwordHash:      text('password_hash').notNull().default(''),
    createdAt:         bigint('created_at_epoch', { mode: 'number' }).notNull().default(nowEpoch).$defaultFn(() => Math.floor(Date.now() / 1000)),
    approved:          boolean('approved').notNull().default(false),
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

// ─── web_sessions (SvelteKit-specific) ─────────────────

export const sessions = pgTable('web_sessions', {
    token:     text('token').primaryKey(),
    pseudo:    text('pseudo').notNull(),
    kind:      text('kind').notNull(),
    expiresAt: bigint('expires_at', { mode: 'number' }).notNull(),
    createdAt: bigint('created_at', { mode: 'number' }).notNull().default(nowEpoch).$defaultFn(() => Math.floor(Date.now() / 1000)),
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

// ─── Inferred types ───────────────────────────────────────────────────────────

export type User       = typeof users.$inferSelect;
export type NewUser    = typeof users.$inferInsert;
export type Session    = typeof sessions.$inferSelect;
export type Invite     = typeof invites.$inferSelect;
export type AuditEntry = typeof auditLog.$inferSelect;
export type SongsurfEvent    = typeof songsurfEvents.$inferSelect;
export type NewSongsurfEvent = typeof songsurfEvents.$inferInsert;
