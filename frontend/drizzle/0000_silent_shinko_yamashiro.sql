CREATE TABLE `audit_log` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`timestamp_epoch` integer NOT NULL,
	`action` text NOT NULL,
	`target` text,
	`detail` text
);
--> statement-breakpoint
CREATE TABLE `donations` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`pseudo` text NOT NULL,
	`method` text NOT NULL,
	`code` text NOT NULL,
	`reviewed` integer DEFAULT false NOT NULL,
	`approved` integer,
	`created_at` integer NOT NULL
);
--> statement-breakpoint
CREATE TABLE `messages` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`from_pseudo` text NOT NULL,
	`to_pseudo` text NOT NULL,
	`body` text NOT NULL,
	`is_read` integer DEFAULT false NOT NULL,
	`created_at` integer NOT NULL
);
--> statement-breakpoint
CREATE TABLE `sessions` (
	`token` text PRIMARY KEY NOT NULL,
	`pseudo` text NOT NULL,
	`kind` text NOT NULL,
	`expires_at` integer NOT NULL,
	`created_at` integer NOT NULL,
	FOREIGN KEY (`pseudo`) REFERENCES `users`(`pseudo`) ON UPDATE no action ON DELETE cascade
);
--> statement-breakpoint
CREATE TABLE `signup_requests` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`pseudo` text NOT NULL,
	`referral` text,
	`temp_password` text NOT NULL,
	`status` text DEFAULT 'pending' NOT NULL,
	`created_at` integer NOT NULL
);
--> statement-breakpoint
CREATE TABLE `test_runs` (
	`run_id` text PRIMARY KEY NOT NULL,
	`executed_at` integer NOT NULL,
	`passed` integer NOT NULL,
	`total` integer NOT NULL,
	`cases` text DEFAULT '[]' NOT NULL
);
--> statement-breakpoint
CREATE TABLE `users` (
	`pseudo` text PRIMARY KEY NOT NULL,
	`role` text DEFAULT 'guest' NOT NULL,
	`active` integer DEFAULT false NOT NULL,
	`password_hash` text NOT NULL,
	`must_change_password` integer DEFAULT true NOT NULL,
	`status` text DEFAULT 'inactif' NOT NULL,
	`bio` text,
	`commentary` text,
	`avatar_bytes` blob,
	`avatar_mime` text,
	`access_songsurf` integer DEFAULT false NOT NULL,
	`access_jellyfin` integer DEFAULT false NOT NULL,
	`request_songsurf` integer DEFAULT false NOT NULL,
	`request_jellyfin` integer DEFAULT false NOT NULL,
	`github_username` text,
	`linkedin_name` text,
	`created_at` integer NOT NULL
);
--> statement-breakpoint
CREATE TABLE `wall_posts` (
	`id` integer PRIMARY KEY AUTOINCREMENT NOT NULL,
	`pseudo` text NOT NULL,
	`body` text NOT NULL,
	`created_at` integer NOT NULL
);
