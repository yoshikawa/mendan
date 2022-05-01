# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :

set shell := ["/bin/bash", "-c"]

alias b := build
alias u := up
alias dr := docker-rust

# Docker compose build
build:
	docker compose build

# Docker compose up
up:
	docker compose up -d

# Docker Actix-web container
docker-rust:
	docker compose exec rust /bin/bash