import { pgTable, text, boolean, bigint, customType } from 'drizzle-orm/pg-core';

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
    commentary:        text('commentary').notNull().default(''),
    accessGithub:      boolean('access_github').notNull().default(false),
    accessJellyfin:    boolean('access_jellyfin').notNull().default(false),
    accessSongsurf:    boolean('access_songsurf').notNull().default(false),
    requestGithub:     boolean('request_github').notNull().default(false),
    requestJellyfin:   boolean('request_jellyfin').notNull().default(false),
    requestSongsurf:   boolean('request_songsurf').notNull().default(false),
    githubStarClaimed: boolean('github_star_claimed').notNull().default(false),
    githubUsername:    text('github_username'),
    linkedinName:      text('linkedin_name'),
    avatarFilename:    text('avatar_filename'),
    avatarSizeBytes:   bigint('avatar_size_bytes', { mode: 'number' }),
    avatarMime:        text('avatar_mime_type'),
    avatarBytes:       bytea('avatar_bytes'),
    passwordHash:      text('password_hash').notNull().default(''),
    createdAt:         bigint('created_at_epoch', { mode: 'number' }).notNull().$defaultFn(() => Math.floor(Date.now() / 1000)),
    approved:          boolean('approved').notNull().default(false),
});

// ─── web_messages (shared with Rust web crate) ───────────────────────────────

export const messages = pgTable('web_messages', {
    id:         bigint('id', { mode: 'number' }).primaryKey().generatedAlwaysAsIdentity(),
    fromPseudo: text('from_pseudo').notNull(),
    toPseudo:   text('to_pseudo').notNull(),
    body:       text('body').notNull(),
    isRead:     boolean('is_read').notNull().default(false),
    createdAt:  bigint('created_at_epoch', { mode: 'number' }).notNull().$defaultFn(() => Math.floor(Date.now() / 1000)),
});

// ─── web_donations (shared with Rust web crate) ──────────────────────────────

export const donations = pgTable('web_donations', {
    id:        bigint('id', { mode: 'number' }).primaryKey().generatedAlwaysAsIdentity(),
    pseudo:    text('pseudo').notNull(),
    method:    text('method').notNull(),
    code:      text('code').notNull().default(''),
    reviewed:  boolean('reviewed').notNull().default(false),
    approved:  boolean('approved').notNull().default(false),
    createdAt: bigint('created_at_epoch', { mode: 'number' }).notNull().$defaultFn(() => Math.floor(Date.now() / 1000)),
});

// ─── web_wall_posts (shared with Rust web crate) ─────────────────────────────

export const wallPosts = pgTable('web_wall_posts', {
    id:        bigint('id', { mode: 'number' }).primaryKey().generatedAlwaysAsIdentity(),
    pseudo:    text('pseudo').notNull(),
    body:      text('body').notNull(),
    createdAt: bigint('created_at_epoch', { mode: 'number' }).notNull().$defaultFn(() => Math.floor(Date.now() / 1000)),
});

// ─── web_invites (shared with Rust web crate) ────────────────────────────────

export const invites = pgTable('web_invites', {
    id:            bigint('id', { mode: 'number' }).primaryKey().generatedAlwaysAsIdentity(),
    code:          text('code').notNull().unique(),
    note:          text('note').notNull().default(''),
    createdAt:     bigint('created_at_epoch', { mode: 'number' }).notNull(),
    expiresAt:     bigint('expires_at_epoch', { mode: 'number' }).notNull(),
    usedBy:        text('used_by'),
    usedAt:        bigint('used_at_epoch', { mode: 'number' }),
});

// ─── web_sessions (SvelteKit-specific, created via init SQL) ─────────────────

export const sessions = pgTable('web_sessions', {
    token:     text('token').primaryKey(),
    pseudo:    text('pseudo').notNull(),
    kind:      text('kind').notNull(),
    expiresAt: bigint('expires_at', { mode: 'number' }).notNull(),
    createdAt: bigint('created_at', { mode: 'number' }).notNull().$defaultFn(() => Math.floor(Date.now() / 1000)),
});

// ─── web_audit_log (SvelteKit-specific, created via init SQL) ────────────────

export const auditLog = pgTable('web_audit_log', {
    id:          bigint('id', { mode: 'number' }).primaryKey().generatedAlwaysAsIdentity(),
    action:      text('action').notNull(),
    actorPseudo: text('actor_pseudo').notNull(),
    target:      text('target').notNull().default(''),
    detail:      text('detail').notNull().default(''),
    createdAt:   bigint('created_at_epoch', { mode: 'number' }).notNull().$defaultFn(() => Math.floor(Date.now() / 1000)),
});

// ─── songsurf_events (SvelteKit-specific, created via init SQL) ──────────────
// Événements poussés par le NAS (watcher + songsurf) via /japprends/api/songsurf-events.
// detail = JSON string (convention cases TEXT). Tri/affichage sur receivedAt (horloge VPS).

export const songsurfEvents = pgTable('songsurf_events', {
    id:         bigint('id', { mode: 'number' }).primaryKey().generatedAlwaysAsIdentity(),
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
    receivedAt: bigint('received_at_epoch', { mode: 'number' }).notNull().$defaultFn(() => Math.floor(Date.now() / 1000)),
});

// ─── Inferred types ───────────────────────────────────────────────────────────

export type User       = typeof users.$inferSelect;
export type NewUser    = typeof users.$inferInsert;
export type Session    = typeof sessions.$inferSelect;
export type Message    = typeof messages.$inferSelect;
export type Donation   = typeof donations.$inferSelect;
export type WallPost   = typeof wallPosts.$inferSelect;
export type Invite     = typeof invites.$inferSelect;
export type AuditEntry = typeof auditLog.$inferSelect;
export type SongsurfEvent    = typeof songsurfEvents.$inferSelect;
export type NewSongsurfEvent = typeof songsurfEvents.$inferInsert;
