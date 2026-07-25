BENCHMARK_CMD=hyperfine --warmup 5
RUST_DIR=../wxrs/target/release
# Call the project interpreter directly rather than going through `uv run`, so
# uv's own startup cost is not counted against Python in the benchmark.
PY=../wxpy/.venv/bin/python
PY_DIR=../wxpy/wxpy

# These are all .PHONY on purpose. They were plain file targets, which meant an
# existing (even empty) result file counted as up to date and was silently never
# regenerated. Each recipe also writes to a temp file and only moves it into
# place on success, so an interrupted run cannot leave a truncated table behind.
.PHONY: all online offline check-env clean \
	ch3_fetch_api.md ch4_serialized.md ch4_offline_benchmark.md ch5.md

all: online offline

# Only the benchmarks that actually call the OpenWeather API need a key.
online: check-env ch3_fetch_api.md ch4_serialized.md

# ch5 needs the bird data extracted first: see `make -C .. data`.
offline: ch4_offline_benchmark.md ch5.md

ch3_fetch_api.md:
	$(BENCHMARK_CMD) "$(RUST_DIR)/ch3 30 -140" \
		"$(PY) $(PY_DIR)/ch3/fetch_api.py 30 -140" \
		--export-markdown $@.tmp && mv $@.tmp $@

ch4_serialized.md:
	$(BENCHMARK_CMD) "$(RUST_DIR)/ch4_benchmark 30 -140" \
		"$(PY) $(PY_DIR)/ch4/serialized_benchmark.py 30 -140" \
		--export-markdown $@.tmp && mv $@.tmp $@

ch4_offline_benchmark.md:
	$(BENCHMARK_CMD) "$(RUST_DIR)/ch4_offline_benchmark" \
		"$(PY) $(PY_DIR)/ch4/serialized_offline_benchmark.py" \
		--export-markdown $@.tmp && mv $@.tmp $@

ch5.md:
	$(BENCHMARK_CMD) "$(RUST_DIR)/ch5" \
		"$(PY) $(PY_DIR)/ch5/ch5.py" \
		"$(PY) $(PY_DIR)/ch5/ch5_pandas.py" \
		--export-markdown $@.tmp && mv $@.tmp $@

check-env:
ifndef OWM_APPID
	$(error OWM_APPID is undefined)
endif

clean:
	rm -f *.md.tmp
