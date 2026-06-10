import { relations } from "drizzle-orm";
import { pgTable, text, timestamp, boolean, index } from "drizzle-orm/pg-core";

export const ba_users = pgTable("ba_users", {
  id: text("id").primaryKey(),
  name: text("name").notNull(),
  email: text("email").notNull().unique(),
  emailVerified: boolean("email_verified").default(false).notNull(),
  image: text("image"),
  createdAt: timestamp("created_at").defaultNow().notNull(),
  updatedAt: timestamp("updated_at")
    .defaultNow()
    .$onUpdate(() => /* @__PURE__ */ new Date())
    .notNull(),
  username: text("username").unique(),
  displayUsername: text("display_username"),
  role: text("role").default("member"),
});

export const ba_sessions = pgTable(
  "ba_sessions",
  {
    id: text("id").primaryKey(),
    expiresAt: timestamp("expires_at").notNull(),
    token: text("token").notNull().unique(),
    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at")
      .$onUpdate(() => /* @__PURE__ */ new Date())
      .notNull(),
    ipAddress: text("ip_address"),
    userAgent: text("user_agent"),
    userId: text("user_id")
      .notNull()
      .references(() => ba_users.id, { onDelete: "cascade" }),
  },
  (table) => [index("ba_sessions_userId_idx").on(table.userId)],
);

export const ba_accounts = pgTable(
  "ba_accounts",
  {
    id: text("id").primaryKey(),
    accountId: text("account_id").notNull(),
    providerId: text("provider_id").notNull(),
    userId: text("user_id")
      .notNull()
      .references(() => ba_users.id, { onDelete: "cascade" }),
    accessToken: text("access_token"),
    refreshToken: text("refresh_token"),
    idToken: text("id_token"),
    accessTokenExpiresAt: timestamp("access_token_expires_at"),
    refreshTokenExpiresAt: timestamp("refresh_token_expires_at"),
    scope: text("scope"),
    password: text("password"),
    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at")
      .$onUpdate(() => /* @__PURE__ */ new Date())
      .notNull(),
  },
  (table) => [index("ba_accounts_userId_idx").on(table.userId)],
);

export const ba_verifications = pgTable(
  "ba_verifications",
  {
    id: text("id").primaryKey(),
    identifier: text("identifier").notNull(),
    value: text("value").notNull(),
    expiresAt: timestamp("expires_at").notNull(),
    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at")
      .defaultNow()
      .$onUpdate(() => /* @__PURE__ */ new Date())
      .notNull(),
  },
  (table) => [index("ba_verifications_identifier_idx").on(table.identifier)],
);

export const ba_usersRelations = relations(ba_users, ({ many }) => ({
  ba_sessionss: many(ba_sessions),
  ba_accountss: many(ba_accounts),
}));

export const ba_sessionsRelations = relations(ba_sessions, ({ one }) => ({
  ba_users: one(ba_users, {
    fields: [ba_sessions.userId],
    references: [ba_users.id],
  }),
}));

export const ba_accountsRelations = relations(ba_accounts, ({ one }) => ({
  ba_users: one(ba_users, {
    fields: [ba_accounts.userId],
    references: [ba_users.id],
  }),
}));
