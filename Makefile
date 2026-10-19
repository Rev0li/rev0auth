SHELL := /usr/bin/env bash

.PHONY: help devtools setup-vps caddy-install launch-all launch-frontend stop-all status test snapshot preflight admin-setup gen-secret frontend-dev frontend-build frontend-check

help:
	@echo "Targets:"
	@echo "  make gen-secret          — create/update .env interactively"
	@echo "  make admin-setup PSEUDO=x — rôle admin + code d'installation de la clé (15 min)"
	@echo "  make launch-all          — docker compose up -d --build"
	@echo "  make stop-all            — docker compose down"
	@echo "  make status              — show container status"
	@echo "  make test                — frontend check + vitest"
	@echo "  make preflight           — pre-deploy checks (env + docker build)"
	@echo "  make snapshot            — tarball of deployable files"
	@echo "  make frontend-dev        — vite dev server (port 5173)"
	@echo "  make setup-vps ARGS='--admin-user deploy --dry-run'"
	@echo "  make caddy-install ARGS='--dry-run'"

devtools:
	./scripts/devtools.sh

setup-vps:
	./scripts/devtools.sh setup-vps $(ARGS)

caddy-install:
	./scripts/devtools.sh caddy-install $(ARGS)

launch-all:
	./scripts/devtools.sh launch-all

stop-all:
	./scripts/devtools.sh stop all

status:
	./scripts/devtools.sh status

test:
	cd frontend && npm run check && npm test

snapshot:
	./scripts/project-snapshot.sh

preflight:
	./scripts/deploy-preflight.sh

# Console admin : rôle admin + code à usage unique pour enregistrer la clé de
# sécurité (première clé ou clé perdue). À lancer là où tourne la stack Docker.
admin-setup:
	@test -n "$(PSEUDO)" || { echo "Usage : make admin-setup PSEUDO=<pseudo>"; exit 1; }
	docker compose exec frontend node scripts/admin-setup.mjs "$(PSEUDO)"

gen-secret:
	./scripts/gen_secret.sh

frontend-dev:
	cd frontend && npm run dev

frontend-build:
	cd frontend && npm run build

frontend-check:
	cd frontend && npm run check

launch-frontend:
	cd frontend && npm run dev &
