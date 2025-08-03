#!/bin/sh

export DOCKER_SOCK_PATH="./docker.sock"
export DURATION_SECONDS="10"
export LISTEN_ADDR_PORT="127.0.0.1:7319"

./containers2ql
