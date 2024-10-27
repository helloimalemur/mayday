#!/bin/bash
#docker-compose up -d --build
function send_discord_message() {
  MESSAGE=$1
  WEBHOOK_URL=https://discord.com/api/webhooks/1219461199536586803/OblHrddHhATm-UpBS5nbB2B7FIS6dusn7X0h9nutZ1ot55Z1wNDJZ-ZEVuGC7RX657FM
  JSON="{\"content\": \"$MESSAGE\"}"
  curl -d "$JSON" -H "Content-Type: application/json" "$WEBHOOK_URL"
}

# start via docker
docker build --pull --no-cache . -t mayday
RUNNING=$(docker ps | grep mayday | tr -s ' ' | cut -d ' ' -f1)
if [[ -z "$RUNNING" ]]; then echo 'not running'; else docker kill "$RUNNING"; fi

## docker depl
#docker-compose up -d

## start via kubernetes
kubectl apply -f namespace.yaml
docker save mayday:latest | k3s ctr images import -
kubectl delete --ignore-not-found=true -f mayday.yaml.yaml
sleep 7s
kubectl apply -f mayday.yaml.yaml


send_discord_message "mayday re-deploying"
