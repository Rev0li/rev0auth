// sqlx::migrate!() scans crates/api/migrations/ at compile time.
// Pour ajouter une migration : créer le fichier SQL dans ce dossier,
// c'est tout — plus besoin de l'enregistrer manuellement.
pub async fn run_pending_migrations(pool: &sqlx::PgPool) -> anyhow::Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
