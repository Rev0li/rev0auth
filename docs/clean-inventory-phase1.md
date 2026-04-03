# Clean Inventory - Phase 1 (Safe)

Date: 2026-04-03
Branch: feature/saas-dashboard-clean-1

## Etat actuel (apres .gitignore)

Definition rapide:
- "untracked" = fichier present localement mais pas encore ajoute dans Git.
- "untracked code" = nouveau code non versionne (pas encore commit), donc potentiellement important mais pas protege par l'historique Git.

Tracked modifies:
- [ ] Cargo.lock
- [ ] docs/auth-012-media-install.md
- [ ] docs/install-to-launch.md

Untracked code candidates:
- [ ] crates/api/migrations/
- [ ] crates/api/src/auth/
- [ ] crates/web/src/pages/portal.rs
- [ ] crates/web/src/styles/

Untracked docs candidates:
- [x] docs/FunFront.md -> docs/archive/2026-04-clean-2/FunFront.md
- [x] docs/Next-Work.md -> docs/archive/2026-04-clean-2/Next-Work.md
- [x] docs/Outtime.md -> docs/archive/2026-04-clean-2/Outtime.md
- [x] docs/architecture.md -> docs/archive/2026-04-clean-2/architecture.md
- [x] docs/audit-auth-complete.md -> docs/archive/2026-04-clean-2/audit-auth-complete.md
- [x] docs/auth-005-report.md -> docs/archive/2026-04-clean-2/auth-005-report.md
- [x] docs/auth-006-rbac.md -> docs/archive/2026-04-clean-2/auth-006-rbac.md
- [x] docs/auth-006-tests.md -> docs/archive/2026-04-clean-2/auth-006-tests.md
- [x] docs/auth-008-flow-db.md -> docs/archive/2026-04-clean-2/auth-008-flow-db.md
- [x] docs/dev-book-auth.md -> docs/archive/2026-04-clean-2/dev-book-auth.md
- [x] docs/nest-001-audit-backend.md -> docs/archive/2026-04-clean-2/nest-001-audit-backend.md
- [x] docs/perf-baseline.md -> docs/archive/2026-04-clean-2/perf-baseline.md
- [x] docs/roadmap.md -> docs/archive/2026-04-clean-2/roadmap.md

Other untracked:
- [ ] README.md (root)

## Decision matrix (a cocher)

### Keep and track now
- [x] Code in `crates/api/migrations/`
- [x] Code in `crates/api/src/auth/`
- [x] UI files in `crates/web/src/pages/portal.rs` and `crates/web/src/styles/`

### Archive docs (keep history, reduce root noise)
- [x] Move old docs to `docs/archive/2026-04-clean-2/`
- [ ] Keep only active docs linked in indexes

### Drop local-only files
- [x] `.run/`, `backups/`, personal notes ignored via `.gitignore`

## Safe cleanup sequence

1. Commit `.gitignore` and this inventory.
2. Decide KEEP vs ARCHIVE for each untracked item.
3. Move ARCHIVE docs into `docs/archive/2026-04-clean-2/`.
4. Commit each lot by scope:
   - `clean(ignore)`
   - `clean(archive-docs)`
   - `feat(api-auth-foundation)` (if code kept)

## Goal

- Keep git status readable.
- Preserve useful history in archive.
- Prepare clean base for SaaS v1 work.
