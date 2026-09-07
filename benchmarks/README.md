# Benchmark provenance

The offline tables were regenerated on 2026-09-05 after refreshing both
dependency lockfiles. The online tables were regenerated on 2026-09-06 after
the chapter 3 and 4 programs switched to HTTPS with an explicit ten second
timeout and an HTTP status check. The online tables use the live OpenWeather
API. Only the chapter 4 buffering comparison remains historical.

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
- On 2026-09-06 the four timed online programs were run with an invalid key.
  All four exited non-zero on the resulting HTTP 401 instead of printing the
  error document, so hyperfine now aborts rather than timing a failed request.
- `make data` extracted the archive into a scratch directory, and the resulting
  CSV matched the existing dataset byte for byte.

Compare library choices and complete implementations using these results. The
timings alone do not identify the cause of differences between the Python wheel
and the local Rust build.

## Live API run

The four timed commands completed successfully over HTTPS, with five warmups
and ten measured requests each: 60 benchmark requests in total. Every program
exits non-zero on a non-2xx status and hyperfine aborts on a failing command,
so all sixty timed responses were HTTP 200.

| Command | Mean wall time | User CPU | System CPU |
|:---|---:|---:|---:|
| Rust current API | 152.5 ms | 31.7 ms | 4.9 ms |
| Python current API | 171.0 ms | 56.8 ms | 11.2 ms |
| Rust forecast | 183.4 ms | 34.3 ms | 6.4 ms |
| Python forecast | 213.5 ms | 61.6 ms | 12.8 ms |

The previous plain-HTTP run (2026-09-05) measured 95.9 ms and 156.9 ms for the
current API, with about 6 ms of Rust user CPU. The TLS handshake accounts for
most of the added Rust CPU time: this build of reqwest uses rustls, while
Python uses the system OpenSSL through the standard library. The gap between
the two languages is correspondingly narrower, and the Rust runs were the
noisier of the two on this pass.

The generated tables contain standard deviations and ranges. These measure
live requests, a TLS handshake, startup, and printing; network latency is not
controlled. The Python forecast program also prints the full decoded response,
so the forecast commands do not perform identical output work. No new memory
measurements were taken.
