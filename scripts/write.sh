#!/bin/sh

HOST={{HOST}}

read -s -p "password: " PASSWORD < /dev/tty
PASSWORD=$(echo $PASSWORD | base64)

echo ""

rm -f .nts-note
touch .nts-note
${EDITOR:-nano} .nts-note
NOTE=$(cat .nts-note)
rm .nts-note

curl -s -X POST -d "$NOTE" -H "Authorization: Basic $PASSWORD" $HOST/new
echo ""
