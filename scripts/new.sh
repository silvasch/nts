#!/bin/sh

HOST={{HOST}}

read -s -p "password: " PASSWORD < /dev/tty
PASSWORD=$(echo $PASSWORD | base64)

echo ""

RESP=$(curl -s -X POST -H "Authorization: Basic $PASSWORD" $HOST/api/check-pwd)
if [ "$RESP" != "ok" ]; then
  echo "invalid password"
  exit 1
fi

TEMPLATE=$(curl -s -H "Authorization: Basic $PASSWORD" $HOST/api/get-template)

rm -f .nts-note
echo $TEMPLATE > .nts-note
${EDITOR:-nano} .nts-note
NOTE=$(cat .nts-note)
rm .nts-note

curl -s -X POST -d "$NOTE" -H "Authorization: Basic $PASSWORD" $HOST/api/new
echo ""
