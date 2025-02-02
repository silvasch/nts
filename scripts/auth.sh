#!/bin/sh

read -s -p "pwd> " PASSWORD < /dev/tty
HASH=$(echo $PASSWORD | tr -d \\n | tr -d " " | sha256sum | cut -d " " -f 1)
BASE64=$(echo ":$HASH" | base64 -w 0)
echo $BASE64
