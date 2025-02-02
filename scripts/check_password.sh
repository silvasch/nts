#!/bin/sh

AUTH=$1
RESP=$(curl -s -H "Authorization: Basic $AUTH" localhost:9112/api/check-pwd)
if [ "$RESP" != "ok" ]; then
  exit 1
fi
