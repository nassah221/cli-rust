#!/usr/bin/env bash

set -u

ROOT="tests/inputs"
OUT_DIR="tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

EMPTY="$ROOT/empty.txt"
SPIDER="$ROOT/spider.txt"
FOX="$ROOT/fox.txt"
BUSTLE="$ROOT/bustle.txt"
ALL="$EMPTY $SPIDER $FOX $BUSTLE"

for FILE in $ALL; do
    BASENAME=$(basename "$FILE")
    cat    $FILE > ${OUT_DIR}/${BASENAME}.out
    cat -n $FILE > ${OUT_DIR}/${BASENAME}.n.out
    cat -b $FILE > ${OUT_DIR}/${BASENAME}.b.out
done

cat $ALL > $OUT_DIR/all.out
cat -n $ALL > $OUT_DIR/all.n.out
cat -b $ALL > $OUT_DIR/all.b.out

cat < $BUSTLE > $OUT_DIR/bustle.stdin.out
cat -n < $BUSTLE > $OUT_DIR/bustle.stdin.n.out
cat -b < $BUSTLE > $OUT_DIR/bustle.stdin.b.out