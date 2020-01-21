#!/usr/bin/env bash

FILE=$1
awk '{ print $0 "," }' < $FILE > tmp
truncate -s-2  tmp
echo "[" `cat tmp` "]" > $1
rm tmp
