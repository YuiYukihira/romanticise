.PHONY: build docker test doc clean run setup

VERSION ?= $(shell tomlq -r .package.version Cargo.toml)
BUILD_DIR := ./target
CACHIX_NAME ?= yuiyukihira

Cargo.nix:
	crate2nix generate

$(BUILD_DIR)/romanticise-$(VERSION).tar.gz: Cargo.nix sqlx-data.json
	cachix watch-exec yuiyukihira nix-build -- -o $@ --arg imageTag '"$(VERSION)"'

load: $(BUILD_DIR)/romanticise-$(VERSION).tar.gz
	docker load < $<

build test docs: sqlx-data.json
	cargo sqlx prepare --check
	cargo $@ --locked

clean:
	cargo clean
	rm Cargo.nix sqlx-data.json

wipe: killdb clean

$(BUILD_DIR)/db:
	@mkdir -p $(BUILD_DIR)
	docker run -d --rm -p 5432:5432 -e POSTGRES_PASSWORD=notalivepassword postgres > $(BUILD_DIR)/db
	
killdb:
	docker stop "$(shell cat $(BUILD_DIR)/db)"
	rm -r build

setup: $(BUILD_DIR)/db
	sqlx database create

$(BUILD_DIR)/migrated: $(BUILD_DIR)/db
	@mkdir -p $(BUILD_DIR)
	sqlx mig run
	touch $(BUILD_DIR)/migrated

sqlx-data.json: $(BUILD_DIR)/migrated
	cargo sqlx prepare
