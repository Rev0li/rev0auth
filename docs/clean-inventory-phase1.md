# Clean Inventory - Phase 1 (Safe)

Date: 2026-04-03
Branch: feature/saas-dashboard-clean-1

## Etat actuel (apres .gitignore)

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
- [ ] docs/FunFront.md
- [ ] docs/Next-Work.md
- [ ] docs/Outtime.md
- [ ] docs/architecture.md
- [ ] docs/audit-auth-complete.md
- [ ] docs/auth-005-report.md
- [ ] docs/auth-006-rbac.md
- [ ] docs/auth-006-tests.md
- [ ] docs/auth-008-flow-db.md
- [ ] docs/dev-book-auth.md
- [ ] docs/nest-001-audit-backend.md
- [ ] docs/perf-baseline.md
- [ ] docs/roadmap.md

Other untracked:
- [ ] README.md (root)

## Decision matrix (a cocher)

### Keep and track now
- [ ] Code in `crates/api/migrations/`
- [ ] Code in `crates/api/src/auth/`
- [ ] UI files in `crates/web/src/pages/portal.rs` and `crates/web/src/styles/`

### Archive docs (keep history, reduce root noise)
- [ ] Move old docs to `docs/archive/2026-04-clean-1/`
- [ ] Keep only active docs linked in indexes

### Drop local-only files
- [x] `.run/`, `backups/`, personal notes ignored via `.gitignore`

## Safe cleanup sequence

1. Commit `.gitignore` and this inventory.
2. Decide KEEP vs ARCHIVE for each untracked item.
3. Move ARCHIVE docs into `docs/archive/2026-04-clean-1/`.
4. Commit each lot by scope:
   - `clean(ignore)`
   - `clean(archive-docs)`
   - `feat(api-auth-foundation)` (if code kept)

## Goal

- Keep git status readable.
- Preserve useful history in archive.
- Prepare clean base for SaaS v1 work.
