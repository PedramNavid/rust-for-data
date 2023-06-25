BENCHMARK_CMD=hyperfine --warmup 5
RUST_DIR=../wxrs/target/release
PY_DIR=python ../wxpy/wxpy

.PHONY: check-env

all: check-env ch3_fetch_api.md ch4_serialized_md ch4_offline_benchmark.md ch5.md

ch3_fetch_api.md:
	$(BENCHMARK_CMD) "$(RUST_DIR)/ch3 30 -140" \
		"$(PY_DIR)/ch3/fetch_api.py 30 -140" \
		--export-markdown ch3_fetch_api.md

ch4_serialized_md:
	$(BENCHMARK_CMD) "$(RUST_DIR)/ch4_benchmark 30 -140" \
		"$(PY_DIR)/ch4/serialized_benchmark.py 30 -140" \
		--export-markdown ch4_serialized.md

ch4_offline_benchmark.md:
	$(BENCHMARK_CMD) "$(RUST_DIR)/ch4_offline_benchmark" \
		"$(PY_DIR)/ch4/serialized_offline_benchmark.py 30 -140" \
		--export-markdown ch4_offline_benchmark.md

ch5.md:
	$(BENCHMARK_CMD) "$(RUST_DIR)/ch5" \
		"$(PY_DIR)/ch5/ch5.py" \
		"$(PY_DIR)/ch5/ch5_pandas.py" \
		--export-markdown ch5.md

check-env:
ifndef OWM_APPID
	$(error OWM_APPID is undefined)
endif

clean:
	rm -rf *.md

