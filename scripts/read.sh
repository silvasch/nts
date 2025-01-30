#!/bin/sh

HOST={{HOST}}

read -s -p "password: " PASSWORD < /dev/tty
PASSWORD=$(echo $PASSWORD | base64)

curl -s -H "Authorization: Basic $PASSWORD" $HOST/get | ${PAGER:-less}
