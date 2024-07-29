#!/bin/bash

source .env

docker_running() {
    local container_name="$1"
    if docker ps --format "{{.Names}}" | grep -q "^${container_name}$"; then
        return 0
    else
        return 1
    fi
}

docker_exists() {
    local container_name="$1"
    if docker ps -a --format "{{.Names}}" | grep -q "^${container_name}$"; then
        return 0
    else
        return 1
    fi
}

if docker_running $CONTAINER_NAME;then
    echo "$CONTAINER_NAME is running"
    exit 0
fi

if docker_exists $CONTAINER_NAME; then
    docker start $CONTAINER_NAME
    echo "$CONTAINER_NAME is started"
else
    docker run -d --rm --pull always --user $USER -v $DB_PATH:/database --name \
        $CONTAINER_NAME -p $DB_PORT:8000 surrealdb/surrealdb:latest start --log \
        trace --auth --user $DB_USER --pass $DB_PASS file:/database/$DB_NAME
    echo "$CONTAINER_NAME is started"
fi


