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
    head -n 2 $FILE > ${OUT_DIR}/${BASENAME}.n2.out
    head -n 4 $FILE > ${OUT_DIR}/${BASENAME}.n4.out
    head -c 1 $FILE > ${OUT_DIR}/${BASENAME}.c1.out
    head -c 2 $FILE > ${OUT_DIR}/${BASENAME}.c2.out
    head -c 4 $FILE > ${OUT_DIR}/${BASENAME}.c4.out
done

head $ALL > ${OUT_DIR}/all.txt.out
head -n 2 $ALL > ${OUT_DIR}/all.txt.n2.out
head -n 4 $ALL > ${OUT_DIR}/all.txt.n4.out
head -c 1 $ALL > ${OUT_DIR}/all.txt.c1.out
head -c 2 $ALL > ${OUT_DIR}/all.txt.c2.out
head -c 4 $ALL > ${OUT_DIR}/all.txt.c4.out
