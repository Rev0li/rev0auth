CREATE TABLE "user_avatars" (
	"user_id" text PRIMARY KEY NOT NULL,
	"bytes" "bytea" NOT NULL,
	"mime_type" text NOT NULL,
	"filename" text,
	"size_bytes" bigint,
	"updated_at" timestamp DEFAULT now() NOT NULL
);
--> statement-breakpoint
UPDATE "ba_users" SET "role" = 'member' WHERE "role" IS NULL;--> statement-breakpoint
ALTER TABLE "ba_users" ALTER COLUMN "role" SET NOT NULL;--> statement-breakpoint
ALTER TABLE "ba_users" ADD COLUMN "active" boolean DEFAULT true NOT NULL;--> statement-breakpoint
ALTER TABLE "ba_users" ADD COLUMN "approved" boolean DEFAULT false NOT NULL;--> statement-breakpoint
ALTER TABLE "ba_users" ADD COLUMN "bio" text DEFAULT '' NOT NULL;--> statement-breakpoint
ALTER TABLE "ba_users" ADD COLUMN "access_jellyfin" boolean DEFAULT false NOT NULL;--> statement-breakpoint
ALTER TABLE "ba_users" ADD COLUMN "access_songsurf" boolean DEFAULT false NOT NULL;--> statement-breakpoint
ALTER TABLE "ba_users" ADD COLUMN "request_jellyfin" boolean DEFAULT false NOT NULL;--> statement-breakpoint
ALTER TABLE "ba_users" ADD COLUMN "request_songsurf" boolean DEFAULT false NOT NULL;--> statement-breakpoint
ALTER TABLE "ba_users" ADD COLUMN "github_username" text;--> statement-breakpoint
ALTER TABLE "ba_users" ADD COLUMN "linkedin_name" text;--> statement-breakpoint
ALTER TABLE "user_avatars" ADD CONSTRAINT "user_avatars_user_id_ba_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."ba_users"("id") ON DELETE cascade ON UPDATE no action;