#!/usr/bin/env bash
if [[ "$1" == "--online" ]]; then
    echo "Online mode enabled"
    ONLINE="true"
else
    echo "Offline mode enabled"
fi

BENCHMARK_CMD="hyperfine --warmup 5 "
RUST_DIR="../wxrs/target/release"
PY_DIR="python ../wxpy/wxpy"

# Chapter 3/4
if [[ $ONLINE ]]; then

    $BENCHMARK_CMD "$RUST_DIR/ch3 30 -140" \
    "$PY_DIR/ch3/fetch_api.py 30 -140" \
    --export-markdown ch3_fetch_api.md

    $BENCHMARK_CMD "$RUST_DIR/ch4_benchmark 30 -140" \
    "$PY_DIR/ch3/serialized_benchmark.py 30 -140" \
    --export-markdown ch3_fetch_api.md

fi


# Chapter 4
$BENCHMARK_CMD "$RUST_DIR/ch4_offline_benchmark" \
    "$PY_DIR/ch4/serialized_offline_benchmark.py" \
    --export-markdown ch4_offline_benchmark.md

# Chapter 5
$BENCHMARK_CMD "$RUST_DIR/ch5" \
    "$PY_DIR/ch5/birds.py" \
    --export-markdown ch5.md
