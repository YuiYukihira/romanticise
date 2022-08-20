.PHONY: build docker test doc clean run setup


Cargo.nix:
	crate2nix generate

romanticise.tar.gz: Cargo.nix
	nix-build -o $@

docker: romanticise.tar.gz

load: docker
	docker load < romanticise.tar.gz

build test docs:
	cargo $@

clean:
	cargo clean
	rm romanticise.tar.gz Cargo.nix

wipe: killdb clean

db:
	docker run -d --rm -p 5432:5432 -e POSTGRES_PASSWORD=notalivepassword postgres > db
	
killdb:
	docker stop "$(shell cat db)"
	rm db

setup: db
	sqlx database create
	./scripts/migrations/run

migrate: db
	sqlx mig run
