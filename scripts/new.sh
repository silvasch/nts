#!/bin/sh

AUTH=$(./scripts/auth.sh)
echo ""

if ! ./scripts/check_password.sh $AUTH; then
  echo "invalid password"
  exit 1
fi

rm -rf .nts-note
${EDITOR:-nano} .nts-note
NOTE=$(cat .nts-note)
rm .nts-note

curl -vs -X POST -d "$NOTE" -H "Authorization: Basic $AUTH" localhost:9112/api/new
echo ""
