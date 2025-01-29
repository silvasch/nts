#!/bin/sh

HOST={{HOST}}

read -p "> " NOTE < /dev/tty

curl -X POST -d $NOTE $HOST/new
