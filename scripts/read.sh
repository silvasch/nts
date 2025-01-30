#!/bin/sh

HOST={{HOST}}

read -s -p "password: " PASSWORD < /dev/tty
PASSWORD=$(echo $PASSWORD | base64)

echo ""

curl -s -H "Authorization: Basic $PASSWORD" $HOST/get | ${PAGER:-less}
echo ""
