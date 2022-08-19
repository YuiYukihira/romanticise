.PHONY: build docker test doc clean run


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
