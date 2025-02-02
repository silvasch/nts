#!/bin/sh

read -s -p "pwd> " PASSWORD < /dev/tty
HASH=$(echo $PASSWORD | tr -d \\n | tr -d " " | sha256sum | cut -d " " -f 1)
AUTH=$(echo ":$HASH" | base64 -w 0)

RESP=$(curl -s -H "Authorization: Basic $AUTH" localhost:9112/api/check-pwd)
if [ "$RESP" != "ok" ]; then
  exit 1
fi

curl -s -H "Authorization: Basic $AUTH" localhost:9112/api/get | ${PAGER:-less}
