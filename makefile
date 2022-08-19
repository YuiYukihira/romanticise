.PHONY: build docker test doc clean run


romantacise.tar.gz:
	nix-build -o $@

docker: romantacise.tar.gz

load: docker
	docker load < romantacise.tar.gz

build test docs clean:
	cargo $@
	rm romantacise.tar.gz