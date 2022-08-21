.PHONY: build docker test doc clean run setup spoof

VERSION ?= $(shell tomlq -r .package.version Cargo.toml)
BUILD_DIR := ./build-artifacts
CACHIX_NAME ?= yuiyukihira

$(BUILD_DIR):
	@mkdir -p $@

Cargo.lock:
	cargo update

Cargo.nix: Cargo.lock
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
	rm -rf Cargo.nix sqlx-data.json

wipe: killdb clean
	rm -rf $(BUILD_DIR)

setup: | $(BUILD_DIR)
	docker run -d --rm -p 5432:5432 -e POSTGRES_PASSWORD=notalivepassword postgres > $(BUILD_DIR)/db
	sqlx database create

$(BUILD_DIR)/db: | $(BUILD_DIR)
	docker run -d --rm -p 5432:5432 -e POSTGRES_PASSWORD=notalivepassword postgres > $(BUILD_DIR)/db
	sqlx database create
	
killdb:
	docker stop "$(shell cat $(BUILD_DIR)/db)"
	rm $(BUILD_DIR)/db

sqlx-data.json: $(BUILD_DIR)/db 
	sqlx mig run
	cargo sqlx prepare
