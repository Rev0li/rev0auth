-- Les avatars de l'ancienne galerie (chat, aigle, dragon, renard, loup…) sont
-- effacés : ces membres retombent sur l'avatar DiceBear « initial-face » par
-- défaut (seedé par le pseudo) et peuvent en composer un nouveau depuis leur profil.
-- Un avatar DiceBear contient toujours la mention de licence « dicebear ».
UPDATE "web_users"
SET "avatar_bytes" = NULL,
    "avatar_mime_type" = NULL,
    "avatar_filename" = NULL,
    "avatar_size_bytes" = NULL
WHERE "avatar_bytes" IS NOT NULL
  AND position('dicebear' in lower(convert_from("avatar_bytes", 'UTF8'))) = 0;
