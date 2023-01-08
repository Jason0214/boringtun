#!/bin/bash
DOCKER_FILE_DIR=$1

docker rmi boringtun-test
DOCKER_BUILDKIT=1 docker build -t boringtun-test --progress=plain $1
