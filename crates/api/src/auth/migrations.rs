use sqlx::{PgPool, Postgres, Transaction};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
pub struct Migration {
    pub version: i64,
    pub name: &'static str,
    pub sql: &'static str,
}

const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        name: "0001_auth_schema",
        sql: include_str!("../../migrations/0001_auth_schema.sql"),
    },
    Migration {
        version: 2,
        name: "0002_audit_logs_table",
        sql: include_str!("../../migrations/0002_audit_logs_table.sql"),
    },
    Migration {
        version: 3,
        name: "0003_indexes_optimization",
        sql: include_str!("../../migrations/0003_indexes_optimization.sql"),
    },
    Migration {
        version: 4,
        name: "0004_web_state",
        sql: include_str!("../../migrations/0004_web_state.sql"),
    },
    Migration {
        version: 5,
        name: "0005_member_approved",
        sql: include_str!("../../migrations/0005_member_approved.sql"),
    },
];

// Dev note: runs all pending SQL migrations in deterministic version order.
// Attached to: Postgres startup path in AppState::from_env.
pub async fn run_pending_migrations(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS _migrations (
            version BIGINT PRIMARY KEY,
            name TEXT NOT NULL,
            applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await?;

    let applied_rows = sqlx::query_as::<_, (i64,)>("SELECT version FROM _migrations")
        .fetch_all(pool)
        .await?;
    let applied: HashSet<i64> = applied_rows.into_iter().map(|(v,)| v).collect();

    for migration in MIGRATIONS {
        if applied.contains(&migration.version) {
            continue;
        }

        let mut tx = pool.begin().await?;
        execute_sql_batch(&mut tx, migration.sql).await?;
        sqlx::query("INSERT INTO _migrations (version, name) VALUES ($1, $2)")
            .bind(migration.version)
            .bind(migration.name)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
    }

    Ok(())
}

async fn execute_sql_batch(
    tx: &mut Transaction<'_, Postgres>,
    sql_batch: &str,
) -> anyhow::Result<()> {
    for statement in sql_batch.split(';') {
        let stmt = statement.trim();
        if stmt.is_empty() {
            continue;
        }
        sqlx::query(stmt).execute(&mut **tx).await?;
    }

    Ok(())
}
