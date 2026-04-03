SHELL := /usr/bin/env bash

.PHONY: help devtools setup-vps caddy-install launch-all launch-api launch-web stop-all stop-api stop-web status test

help:
	@echo "Targets:"
	@echo "  make setup-vps ARGS='--admin-user deploy --dry-run'"
	@echo "  make caddy-install ARGS='--dry-run'"
	@echo "  make launch-all"
	@echo "  make launch-api"
	@echo "  make launch-web"
	@echo "  make stop-all"
	@echo "  make status"
	@echo "  make test"

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
