#!/usr/bin/env bash

set -u

ROOT="crates/cat/tests/inputs"
OUT_DIR="crates/cat/tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

EMPTY="$ROOT/empty.txt"
TABS="$ROOT/tabs.txt"
FOX="$ROOT/fox.txt"
SPIDERS="$ROOT/spiders.txt"
BUSTLE="$ROOT/the-bustle.txt"
ALL="$EMPTY $FOX $SPIDERS $BUSTLE"

for FILE in $ALL; do
    BASENAME=$(basename "$FILE")
    cat    $FILE > ${OUT_DIR}/${BASENAME}.out
    cat -n $FILE > ${OUT_DIR}/${BASENAME}.n.out
    cat -b $FILE > ${OUT_DIR}/${BASENAME}.b.out
    cat -E $FILE > ${OUT_DIR}/${BASENAME}.E.out
done

cat -T $TABS > ${OUT_DIR}/$(basename $TABS).T.out
cat -v $BUSTLE > ${OUT_DIR}/$(basename $BUSTLE).v.out

cat    $ALL > $OUT_DIR/all.out
cat -n $ALL > $OUT_DIR/all.n.out
cat -b $ALL > $OUT_DIR/all.b.out
cat -E $ALL > $OUT_DIR/all.E.out

cat    < $BUSTLE > $OUT_DIR/$(basename $BUSTLE).stdin.out
cat -n < $BUSTLE > $OUT_DIR/$(basename $BUSTLE).n.stdin.out
cat -b < $BUSTLE > $OUT_DIR/$(basename $BUSTLE).b.stdin.out
cat -E < $BUSTLE > $OUT_DIR/$(basename $BUSTLE).E.stdin.out

