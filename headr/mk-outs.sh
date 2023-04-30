#! /usr/bin/env bash

set -u

ROOT="tests/inputs"
OUT_DIR="tests/expected"

ONE="$ROOT/one.txt"
TWO="$ROOT/two.txt"
THREE="$ROOT/three.txt"
TEN="$ROOT/ten.txt"
ALL="$ONE $TWO $THREE $TEN"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

for FILE in $ALL; do
    BASENAME=$(basename $FILE)

    head $FILE > ${OUT_DIR}/${BASENAME}.out
    head -n 2 $FILE > ${OUT_DIR}/${BASENAME}.n.out
done

