include docker/.env

## start everything from scratch
.PHONY: init
init: build startdb dbup startmayday
	@:

############################################
## build containers only
.PHONY: build
build:
	@docker-compose --env-file docker/.env -f docker/docker-compose.yaml create

.PHONY: builddb
builddb:
	@docker-compose --env-file docker/.env -f docker/docker-compose.yaml create mayday-db

.PHONY: importk3s
importk3s: build
	@./scripts/importk3s.sh

.PHONY: kubeimport
kubeimport: importk3s
	@:
# function k3simport () { IMAGES=$(docker-compose -f docker/docker-compose.yaml images | tail -n+2 | tr -s ' ' | cut -d ' ' -f 2); for IMAGE in ${IMAGES[@]}; do docker save $IMAGE | k3s ctr images import -; done; }


.PHONY: kubeup
kubeup:
	@sudo kubectl apply -f kube/namespace.yaml
	@sudo kubectl apply -f kube/mayday-db-deployment.yaml
	@sudo kubectl apply -f kube/mayday-db-service.yaml
	@sleep 3s
	#@sudo kubectl apply -f kube/db-migration-pod.yaml
	@sleep 3s
	@sudo kubectl apply -f kube/
#	@make dbuponly

.PHONY: kuberestart
kuberestart:
	@kubectl delete -f kube/mayday-backend-deployment.yaml
	@kubectl delete -f kube/mayday_backend-service.yaml
	@kubectl delete -f kube/mayday-frontend-deployment.yaml
	@kubectl delete -f kube/mayday_frontend-service.yaml
	@kubectl delete -f kube/mayday-db-deployment.yaml
	@kubectl delete -f kube/mayday-db-service.yaml
	@kubectl apply -f kube/mayday-backend-deployment.yaml
	@kubectl apply -f kube/mayday_backend-service.yaml
	@kubectl apply -f kube/mayday-frontend-deployment.yaml
	@kubectl apply -f kube/mayday_frontend-service.yaml
	@kubectl apply -f kube/mayday-db-deployment.yaml
	@kubectl apply -f kube/mayday-db-service.yaml

.PHONY: kubedown
kubedown:
	@kubectl delete -f kube/mayday-backend-deployment.yaml
	@kubectl delete -f kube/mayday_backend-service.yaml
	@kubectl delete -f kube/mayday-frontend-deployment.yaml
	@kubectl delete -f kube/mayday_frontend-service.yaml
	@kubectl delete -f kube/mayday-db-deployment.yaml
	@kubectl delete -f kube/mayday-db-service.yaml


############################################
## up db and run rust code
.PHONY: dev
dev: startdb dbup
	@cargo run

############################################
## start containers
.PHONY: up
up: build startdb dbup
	@docker-compose --env-file docker/.env -f docker/docker-compose.yaml up

## start containers
.PHONY: backend
backend: build startdb dbup
	@docker-compose --env-file docker/.env -f docker/docker-compose.yaml up mayday-backend

## start containers
.PHONY: frontend
frontend: build startdb dbup backend
	@docker-compose --env-file docker/.env -f docker/docker-compose.yaml up mayday-frontend

## start containers daemonized
.PHONY: start
start: build startdb dbup
	@docker-compose --env-file docker/.env -f docker/docker-compose.yaml up -d

############################################
## start database
.PHONY: startdb
startdb:
	@docker-compose --env-file docker/.env -f docker/docker-compose.yaml up mayday-db -d

## start mayday
.PHONY: startmayday
startmayday:
	@docker-compose --env-file docker/.env -f docker/docker-compose.yaml up mayday-backend -d

############################################
## stop containers
.PHONY: down
down:
	@docker-compose --env-file docker/.env -f docker/docker-compose.yaml down

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
clean: down prune cleanfe cleankubenamespace
	@:

## remove kube namespace
.PHONY: cleankubenamespace
cleankubenamespace: prune cleanfe
	@sleep 6s
	@timeout 30s kubectl delete namespace mayday || true
	@#kubectl delete namespace mayday || true
	@./scripts/remove_stuck_namespaces.sh || true

## clean up frontend build artifacts
.PHONY: cleanfe
cleanfe:
	@rm -rf frontend/.next/
	@rm -rf frontend/node_modules/

## remove only images
.PHONY: prune
prune: kubeclean
	@yes|docker system prune -a
	@cargo clean

## remove k3s images
.PHONY: kubeclean
kubeclean:
	@k3s ctr images prune --all
# function k3simport () { IMAGES=$(docker-compose -f docker/docker-compose.yaml images | tail -n+2 | tr -s ' ' | cut -d ' ' -f 2); for IMAGE in ${IMAGES[@]}; do docker save $IMAGE | k3s ctr images import -; done; }

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
dbup: builddb startdb
	@:

#.PHONY: dbuponly
#dbuponly:
#	@cd migration/ && env DATABASE_URL=$(DATABASE_URL) cargo run -- up
#	@docker-compose --env-file docker/.env -f docker/docker-compose.yaml up db-migration

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
