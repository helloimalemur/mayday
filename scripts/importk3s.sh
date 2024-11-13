#!/bin/bash
IMAGES=$(docker-compose -f docker/docker-compose.yaml images | tail -n+2 | tr -s ' ' | cut -d ' ' -f 2); for IMAGE in ${IMAGES[@]}; do docker save $IMAGE | k3s ctr images import -; done;
