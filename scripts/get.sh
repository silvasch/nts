#!/bin/sh

AUTH=$(./scripts/auth.sh)
echo ""

if ! ./scripts/check_password.sh $AUTH; then
  echo "invalid password"
  exit 1
fi

curl -s -H "Authorization: Basic $AUTH" localhost:9112/api/get | ${PAGER:-less}
