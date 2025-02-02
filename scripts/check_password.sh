#!/bin/sh

AUTH=$(./scripts/auth.sh)
RESP=$(curl -s -H "Authorization: Basic $AUTH" localhost:9112/check-pwd)
if [ "$RESP" != "ok" ]; then
  exit 1
fi
