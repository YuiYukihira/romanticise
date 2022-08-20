.PHONY: build docker test doc clean run setup


Cargo.nix:
	crate2nix generate

build/romanticise.tar.gz: Cargo.nix sqlx-data.json
	cargo sqlx prepare --check
	nix-build -o $@

load: build/romanticise.tar.gz
	docker load < build/romanticise.tar.gz

build test docs: sqlx-data.json
	cargo sqlx prepare --check
	cargo $@

clean:
	cargo clean
	rm build/romanticise.tar.gz Cargo.nix sqlx-data.json

wipe: killdb clean

build/db:
	@mkdir -p build
	docker run -d --rm -p 5432:5432 -e POSTGRES_PASSWORD=notalivepassword postgres > build/db
	
killdb:
	docker stop "$(shell cat build/db)"
	rm -r build

setup: build/db
	sqlx database create

build/migrated: build/db
	@mkdir -p build
	sqlx mig run
	touch build/migrated

sqlx-data.json: build/migrated
	cargo sqlx prepare
