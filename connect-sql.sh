#!/bin/bash

source .env

surreal sql --endpoint ws://$DB_IP:$DB_PORT --username $DB_NAME --password $DB_PASS --ns test --db test --pretty
