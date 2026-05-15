SHELL := /usr/bin/env bash

.PHONY: help devtools setup-vps caddy-install launch-all launch-api launch-web launch-frontend stop-all stop-api stop-web status test snapshot preflight admin-otp admin-2fa-init gen-secret enroll-yubikey frontend-dev frontend-build frontend-check

help:
	@echo "Targets:"
	@echo "  make gen-secret          — create/update .env interactively"
	@echo "  make enroll-yubikey      — persist YubiKey credential to .env"
	@echo "  make admin-2fa-init      — generate/display TOTP secret + QR"
	@echo "  make admin-otp           — print current OTP code"
	@echo "  make launch-all          — docker compose up -d --build"
	@echo "  make stop-all            — docker compose down"
	@echo "  make status              — show container status"
	@echo "  make test                — security audit + cargo test"
	@echo "  make preflight           — pre-deploy checks (env + docker build)"
	@echo "  make snapshot            — tarball of deployable files"
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

launch-api:
	./scripts/devtools.sh launch api --bg

launch-web:
	./scripts/devtools.sh launch web --bg

stop-all:
	./scripts/devtools.sh stop all

stop-api:
	./scripts/devtools.sh stop api

stop-web:
	./scripts/devtools.sh stop web

status:
	./scripts/devtools.sh status

test:
	./scripts/devtools.sh test

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

enroll-yubikey:
	./scripts/enroll-yubikey.sh

frontend-dev:
	cd frontend && npm run dev

frontend-build:
	cd frontend && npm run build

frontend-check:
	cd frontend && npm run check

launch-frontend:
	cd frontend && npm run dev &
