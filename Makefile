include docker/.env

## start everything from scratch
.PHONY: init
init: build startdb dbup startmayday
############################################
## build containers only
.PHONY: build
build:
	docker-compose -f docker/docker-compose.yaml create
############################################
## up db and run rust code
.PHONY: dev
dev: startdb
	cargo run
############################################
## start containers
.PHONY: up
up: build
	docker-compose -f docker/docker-compose.yaml up

## start containers daemonized
.PHONY: start
start: build
	docker-compose -f docker/docker-compose.yaml up -d

############################################
## start database
.PHONY: startdb
startdb:
	docker-compose -f docker/docker-compose.yaml up db -d
## start mayday
.PHONY: startmayday
startmayday:
	docker-compose -f docker/docker-compose.yaml up mayday -d

############################################
## stop containers
.PHONY: down
down:
	docker-compose -f docker/docker-compose.yaml down

## kill containers
.PHONY: kill
kill: down
############################################
## remove all everything
.PHONY: prune
prune: down
	yes|docker system prune -a
	cargo clean
	cd migration/ && cargo clean

############################################
## build rust
.PHONY: rust
rust:
	cargo build

############################################
## database migrations - seaorm

## apply migrations
.PHONY: dbup
dbup:
	cd migration/ && env DATABASE_URL=$(DATABASE_URL) cargo run -- up

## rollback migrations
.PHONY: dbdown
dbdown:
	cd migration/ && env DATABASE_URL=$(DATABASE_URL) cargo run -- down

## check migration status
.PHONY: dbstatus
dbstatus:
	cd migration/ && env DATABASE_URL=$(DATABASE_URL) cargo run -- status

## connect to database
.PHONY: dbconn
dbconn:
	mariadb -u ${MARIADB_USER} -h ${MARIADB_HOST} -p${MARIADB_PASS}
