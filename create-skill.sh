#!/bin/sh

curl -XPOST \
    -H'Content-Type: application/json' \
    --basic \
    --user 'admin:nimda' \
    --data \
    '{"label": "'"$*"'"}' \
    http://localhost:8080/api/skills
