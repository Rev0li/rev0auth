import { sqliteTable, text, integer, blob } from 'drizzle-orm/sqlite-core';

export const users = sqliteTable('users', {
    pseudo:            text('pseudo').primaryKey(),
    role:              text('role', { enum: ['guest', 'member', 'mod', 'admin'] }).notNull().default('guest'),
    active:            integer('active', { mode: 'boolean' }).notNull().default(false),
    passwordHash:      text('password_hash').notNull(),
    mustChangePassword: integer('must_change_password', { mode: 'boolean' }).notNull().default(true),
    status:            text('status', { enum: ['actif', 'occupe', 'inactif'] }).notNull().default('inactif'),
    bio:               text('bio'),
    commentary:        text('commentary'),
    avatarBytes:       blob('avatar_bytes'),
    avatarMime:        text('avatar_mime'),
    accessSongsurf:    integer('access_songsurf', { mode: 'boolean' }).notNull().default(false),
    accessJellyfin:    integer('access_jellyfin', { mode: 'boolean' }).notNull().default(false),
    requestSongsurf:   integer('request_songsurf', { mode: 'boolean' }).notNull().default(false),
    requestJellyfin:   integer('request_jellyfin', { mode: 'boolean' }).notNull().default(false),
    githubUsername:    text('github_username'),
    linkedinName:      text('linkedin_name'),
    createdAt:         integer('created_at', { mode: 'timestamp' }).notNull().$defaultFn(() => new Date()),
});

export const sessions = sqliteTable('sessions', {
    token:     text('token').primaryKey(),
    pseudo:    text('pseudo').notNull().references(() => users.pseudo, { onDelete: 'cascade' }),
    kind:      text('kind', { enum: ['admin', 'member'] }).notNull(),
    expiresAt: integer('expires_at', { mode: 'timestamp' }).notNull(),
    createdAt: integer('created_at', { mode: 'timestamp' }).notNull().$defaultFn(() => new Date()),
});

export const signupRequests = sqliteTable('signup_requests', {
    id:          integer('id').primaryKey({ autoIncrement: true }),
    pseudo:      text('pseudo').notNull(),
    referral:    text('referral'),
    tempPassword: text('temp_password').notNull(),
    status:      text('status', { enum: ['pending', 'approved', 'rejected'] }).notNull().default('pending'),
    createdAt:   integer('created_at', { mode: 'timestamp' }).notNull().$defaultFn(() => new Date()),
});

export const messages = sqliteTable('messages', {
    id:         integer('id').primaryKey({ autoIncrement: true }),
    fromPseudo: text('from_pseudo').notNull(),
    toPseudo:   text('to_pseudo').notNull(),
    body:       text('body').notNull(),
    isRead:     integer('is_read', { mode: 'boolean' }).notNull().default(false),
    createdAt:  integer('created_at', { mode: 'timestamp' }).notNull().$defaultFn(() => new Date()),
});

export const donations = sqliteTable('donations', {
    id:        integer('id').primaryKey({ autoIncrement: true }),
    pseudo:    text('pseudo').notNull(),
    method:    text('method', { enum: ['pcs', 'crypto'] }).notNull(),
    code:      text('code').notNull(),
    reviewed:  integer('reviewed', { mode: 'boolean' }).notNull().default(false),
    approved:  integer('approved', { mode: 'boolean' }),
    createdAt: integer('created_at', { mode: 'timestamp' }).notNull().$defaultFn(() => new Date()),
});

export const wallPosts = sqliteTable('wall_posts', {
    id:        integer('id').primaryKey({ autoIncrement: true }),
    pseudo:    text('pseudo').notNull(),
    body:      text('body').notNull(),
    createdAt: integer('created_at', { mode: 'timestamp' }).notNull().$defaultFn(() => new Date()),
});

export const testRuns = sqliteTable('test_runs', {
    runId:      text('run_id').primaryKey(),
    executedAt: integer('executed_at', { mode: 'timestamp' }).notNull(),
    passed:     integer('passed').notNull(),
    total:      integer('total').notNull(),
    cases:      text('cases').notNull().default('[]'), // JSON
});

export const auditLog = sqliteTable('audit_log', {
    id:             integer('id').primaryKey({ autoIncrement: true }),
    timestampEpoch: integer('timestamp_epoch').notNull(),
    action:         text('action').notNull(),
    target:         text('target'),
    detail:         text('detail'),
});

// ─── Inferred types ───────────────────────────────────────────────────────────

export type User           = typeof users.$inferSelect;
export type NewUser        = typeof users.$inferInsert;
export type Session        = typeof sessions.$inferSelect;
export type SignupRequest  = typeof signupRequests.$inferSelect;
export type Message        = typeof messages.$inferSelect;
export type Donation       = typeof donations.$inferSelect;
export type WallPost       = typeof wallPosts.$inferSelect;
export type TestRun        = typeof testRuns.$inferSelect;
export type AuditEntry     = typeof auditLog.$inferSelect;
