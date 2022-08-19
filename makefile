.PHONY: build docker test doc clean run


romanticise.tar.gz:
	nix-build -o $@

docker: romanticise.tar.gz

load: docker
	docker load < romanticise.tar.gz

build test docs clean:
	cargo $@
	rm romanticise.tar.gz