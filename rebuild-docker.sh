#!/usr/bin/env bash

# set the image/container name
NAME=wasmd

# set the image version
VERSION=3.0.0-ibc2

# remove Docker container (if exists)
container_id="$(docker container list -af name=$NAME -q)"
if [ -z "$container_id" ]
then
  echo "$NAME container not found, skipping deletion"
else
  echo "$NAME container found, deleting"
  docker rm -f "$container_id"
  echo "$NAME container deleted"
fi

# remove Docker image (if exists)
image_id="$(docker images -f reference=$NAME -q)"
if [ -z "$image_id" ]
then
  echo "$NAME image not found, skipping deletion"
else
  echo "$NAME image found, deleting"
  docker rmi "$image_id"
  echo "$NAME image deleted"
fi

# build a new Docker image
docker build -t "$NAME:$VERSION" .

# start a new Docker container
docker run --rm --name $NAME -d "$NAME:$VERSION"

# display logs from running Docker container
docker logs -f "$NAME"

