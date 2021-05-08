#!/bin/bash

if [[ -z $3 ]]; then
    echo "Please provide username, password and the image to deploy to Dockerhub"
    exit 1
fi

USERNAME=$1
PASSWORD=$2
IMAGE=$3

docker login -u "$USERNAME" -p "$PASSWORD"
docker build -t "$USERNAME"/"$IMAGE":latest .
docker push "$USERNAME"/"$IMAGE":latest
