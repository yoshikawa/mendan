# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :

set shell := ["/bin/bash", "-c"]

alias b := build
alias u := up
alias d := down
alias rust := docker-rust
alias db := docker-db
alias redis := docker-redis

# Docker compose build
build:
	docker compose build

# Docker compose up
up:
	docker compose up -d

# Docker compose down
down:
	docker compose down -v

# Docker Actix-web container
docker-rust:
	docker compose exec rust /bin/bash

# Docker PostgreSQL container
docker-db:
	docker compose exec db /bin/bash

# Docker Redis container
docker-redis:
	docker compose exec redis /bin/bash
