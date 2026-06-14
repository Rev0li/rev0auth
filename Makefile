SHELL := /usr/bin/env bash

.PHONY: help devtools setup-vps caddy-install launch-all launch-frontend stop-all status test snapshot preflight admin-otp admin-2fa-init gen-secret frontend-dev frontend-build frontend-check

help:
	@echo "Targets:"
	@echo "  make gen-secret          — create/update .env interactively"
	@echo "  make admin-2fa-init      — generate/display TOTP secret + QR"
	@echo "  make admin-otp           — print current OTP code"
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

admin-otp:
	./scripts/admin-otp.sh

admin-2fa-init:
	./scripts/admin-2fa-init.sh

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
