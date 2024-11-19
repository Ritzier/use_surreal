#!/bin/bash

source .env

surreal sql --endpoint ws://$DB_IP:$DB_PORT -u "${DB_USER}" -p "${DB_PASS}" --ns test --db test --pretty
