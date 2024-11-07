include docker/.env
## start everything from scratch
.PHONY: init
init: build startdb dbup startmayday
	@:
############################################
## build containers only
.PHONY: build
build:
	@docker-compose -f docker/docker-compose.yaml create
############################################
## up db and run rust code
.PHONY: dev
dev: startdb dbup
	@cargo run
############################################
## start containers
.PHONY: up
up: build startdb dbup
	@docker-compose -f docker/docker-compose.yaml up

## start containers
.PHONY: backend
backend: build startdb dbup
	@docker-compose -f docker/docker-compose.yaml up mayday

## start containers
.PHONY: frontend
frontend: build startdb dbup backend
	@docker-compose -f docker/docker-compose.yaml up mayday-fe

## start containers daemonized
.PHONY: start
start: build
	@docker-compose -f docker/docker-compose.yaml up -d

############################################
## start database
.PHONY: startdb
startdb:
	@docker-compose -f docker/docker-compose.yaml up db -d
## start mayday
.PHONY: startmayday
startmayday:
	@docker-compose -f docker/docker-compose.yaml up mayday -d

############################################
## stop containers
.PHONY: down
down:
	@docker-compose -f docker/docker-compose.yaml down

## kill containers
.PHONY: kill
kill: down
	@:
############################################
.PHONY: k3sup
k3sup: build
	sudo docker save mayday:latest | sudo k3s ctr images import -
	sudo docker save mayday-db:latest | sudo k3s ctr images import -
	@sudo kubectl apply -f kube/namespace.yaml
	@sudo kubectl apply -f kube/db-deployment.yaml
	@sudo kubectl apply -f kube/db-service.yaml
	@sudo kubectl apply -f kube/mayday-deployment.yaml

.PHONY: k3sdown
k3sdown:
	@kubectl delete namespace mayday

############################################
## remove all everything
.PHONY: clean
clean: prune
	@:

.PHONY: prune
prune: down
	@yes|docker system prune -a
	@cargo clean
	@cd migration/ && cargo clean
	@rm -rf frontend/.next/
	@rm -rf frontend/node_modules/
############################################
## build rust
.PHONY: rust
rust:
	@cargo build

############################################
## frontend
.PHONY: fedev
fedev :
	@cd frontend/ && npm install --force && npm run dev

.PHONY: frontend-dev
frontend-dev: fedev
	@:

############################################
## database migrations - seaorm

## apply migrations
.PHONY: dbup
dbup:
	@cd migration/ && env DATABASE_URL=$(DATABASE_URL) cargo run -- up

## rollback migrations
.PHONY: dbdown
dbdown:
	@cd migration/ && env DATABASE_URL=$(DATABASE_URL) cargo run -- down

## check migration status
.PHONY: dbstatus
dbstatus:
	@cd migration/ && env DATABASE_URL=$(DATABASE_URL) cargo run -- status

## connect to database
.PHONY: dbconn
dbconn:
	@mariadb -u ${MARIADB_USER} -h ${MARIADB_HOST} -p${MARIADB_PASS}

## Dependencies
.PHONY: install-node
install-node:
	@./scripts/install-node.sh

.PHONY: install-deps
install-deps: install-node
	@:
