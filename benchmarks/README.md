# Benchmark provenance

The offline and online tables were regenerated on 2026-09-05 after refreshing
both dependency lockfiles. The online tables use the live OpenWeather API.
Only the chapter 4 buffering comparison remains historical.

## Environment

| Item | Version / configuration |
|:---|:---|
| Machine | Apple M5 Pro, 24 GiB RAM |
| OS | macOS 26.6.2, aarch64 |
| Rust / Cargo | 1.97.0 |
| Python | CPython 3.13.14 |
| uv | 0.11.32 |
| hyperfine | 1.20.0 |
| Rust Polars | 0.55.2 |
| Python Polars | 1.44.1 |
| pandas | 3.0.5 |
| NumPy | 2.5.2 |
| Rust build | Standard release profile; `RUSTFLAGS` unset |
| Polars threads | Default; `POLARS_MAX_THREADS` unset |

The remaining dependency versions are recorded in `wxrs/Cargo.lock` and
`wxpy/uv.lock`. The compiler and machine are not pinned by these files.

## Reproduction

From the repository root, with hyperfine installed:

```sh
make setup
make data
make build-release
make benchmarks-offline
```

For the online run, export `OWM_APPID` from your local `.env` (never commit the
key), then run:

```sh
set -a
. ./.env
set +a
make -C benchmarks online 'BENCHMARK_CMD=hyperfine --warmup 5 --runs 10'
```

The harness runs commands serially, with five warmups. The offline run used
hyperfine's automatic run count (at least ten); the online run explicitly used
ten measured runs per command. It invokes Python directly from the project virtual
environment, excluding uv startup, and uses optimized Rust binaries. Compilation
and archive extraction are outside the timed commands. Warmups warm filesystem
caches; these are not cold-disk measurements. Program stdout is discarded by
hyperfine, but output formatting and writes still take time.

The chapter 4 run used 138 Rust repetitions and 28 Python repetitions. Hyperfine
flagged statistical outliers in the Rust run; the table retains the measured
range and standard deviation. The chapter 5 run used ten repetitions per
implementation. No new peak-memory measurements were taken.

## Correctness checks during the refresh

- Both offline parsers matched all 24,576 input records, including timestamps,
  AQI, and component values (floating-point tolerance for Rust's `f32`). Their
  output labels and formatting differ, so the timing is not a pure JSON parser
  comparison.
- Rust Polars, Python Polars, and pandas agreed on all 7,724 aggregate rows,
  comparing all five output columns after sorting independently of display
  order. This checks this dataset, not every possible null or malformed input.
- All six online example commands ran against a local HTTP fixture. This checks
  request/response handling, not live authentication or service availability.
- `make data` extracted the archive into a scratch directory, and the resulting
  CSV matched the existing dataset byte for byte.

Compare library choices and complete implementations using these results. The
timings alone do not identify the cause of differences between the Python wheel
and the local Rust build.

## Live API run

Both endpoint preflights returned HTTP 200 with valid records: one current
record (175 bytes) and 96 forecast records (13,097 bytes). The four timed
commands completed successfully, with five warmups and ten measured requests
each: 60 benchmark requests plus two preflight requests.

| Command | Mean wall time | User CPU | System CPU |
|:---|---:|---:|---:|
| Rust current API | 95.9 ms | 6.4 ms | 6.6 ms |
| Python current API | 156.9 ms | 55.2 ms | 11.9 ms |
| Rust forecast | 107.0 ms | 6.7 ms | 7.3 ms |
| Python forecast | 170.1 ms | 56.9 ms | 12.0 ms |

The generated tables contain standard deviations and ranges. These measure
live requests, startup, and printing; network latency is not controlled.
The Python forecast program also prints the full decoded response, so the
forecast commands do not perform identical output work. No new memory
measurements were taken. The current-fetch programs do not reject HTTP errors;
the successful preflight does not prove every timed response had HTTP 200.
