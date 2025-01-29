#!/bin/sh

HOST={{HOST}}

curl -s $HOST/get | less
