#!/bin/sh

HOST={{HOST}}

read -p "> " NOTE < /dev/tty

read -s -p "password: " PASSWORD < /dev/tty
PASSWORD=$(echo $PASSWORD | base64)

curl -s -X POST -d $NOTE -H "Authorization: Basic $PASSWORD" $HOST/new
