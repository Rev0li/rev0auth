use axum::response::Html;

use super::{frontend_theme, portal_page_styles};

pub async fn portal() -> Html<String> {
    Html(
        r##"<!doctype html>
<html lang="fr">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>rev0auth - Inscription</title>
    %%FRONTEND_THEME_BOOT%%
    <style>%%PORTAL_PAGE_STYLES%%</style>
</head>
<body>
    <main class="page">
        <div class="header">
            <h1>Inscription</h1>
            <a class="link" href="/">← Connexion</a>
        </div>
        <article class="card" style="text-align:center;padding:2rem 1.5rem;">
            <p style="font-size:1.5rem;margin-bottom:0.75rem">🔒</p>
            <p style="font-weight:600;margin-bottom:0.5rem">Inscription sur invitation uniquement</p>
            <p style="font-size:0.875rem;color:var(--muted-foreground)">
                Tu as reçu un lien d'invitation ? Utilise-le directement.<br>
                Sinon, contacte un admin pour en obtenir un.
            </p>
        </article>
    </main>
</body>
</html>"##
        .replace("%%FRONTEND_THEME_BOOT%%", frontend_theme::FRONTEND_THEME_BOOT)
        .replace("%%PORTAL_PAGE_STYLES%%", portal_page_styles::PORTAL_PAGE_STYLES),
    )
}
