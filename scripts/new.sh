#!/bin/sh

read -s -p "pwd> " PASSWORD < /dev/tty
HASH=$(echo $PASSWORD | tr -d \\n | tr -d " " | sha256sum | cut -d " " -f 1)
AUTH=$(echo ":$HASH" | base64 -w 0)
echo ""

RESP=$(curl -s -H "Authorization: Basic $AUTH" localhost:9112/api/check-pwd)
if [ "$RESP" != "ok" ]; then
  echo "invalid password"
  exit 1
fi

rm -rf .nts-note
${EDITOR:-nano} .nts-note
NOTE=$(cat .nts-note)
rm .nts-note

curl -s -X POST -d "$NOTE" -H "Authorization: Basic $AUTH" localhost:9112/api/new
echo ""
