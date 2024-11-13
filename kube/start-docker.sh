#!/bin/bash
#docker-compose up -d --build

#docker-compose build
docker build --pull --no-cache . -t mayday_frontend
RUNNING=$(docker ps | grep mayday_frontend | tr -s ' ' | cut -d ' ' -f1)
if [[ -z "$RUNNING" ]]; then echo 'not running'; else docker kill "$RUNNING"; fi

## start via docker
#docker-compose up -d

## start via kubernetes
kubectl apply -f namespace.yaml
docker save mayday_frontend:latest | k3s ctr images import -
kubectl delete --ignore-not-found=true -f kube_deploy_and_service.yaml
sleep 7s
kubectl apply -f kube_deploy_and_service.yaml


send_discord_message "mayday re-deploying"
