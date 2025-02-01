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

curl -s -H "Authorization: Basic $PASSWORD" $HOST/api/get | ${PAGER:-less}
echo ""
