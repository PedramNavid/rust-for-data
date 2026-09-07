# Fetching from an API

One of the simplest examples to start with is fetching data from an API endpoint.
This is often the beginning of many data pipeline journeys.

We will use the [OpenWeatherMap Air Pollution API](https://openweathermap.org/api/air-pollution)
to fetch the current air pollution for a configurable location by providing a
latitude and longitude on the command line. Given that I'm in California, air
pollution felt like the natural place to start.

You will need to sign up for a free account to get an API key. Once you've
signed up, [create an API key](https://home.openweathermap.org/api_keys). The
free tier allows a fixed number of requests per day; check the
[current pricing page](https://openweathermap.org/price) for the limits before
you start running benchmarks in a loop.

The chapter walks through the same small program in both languages: read the
arguments, make the request, print the response, fail when something goes
wrong, and finally measure both.

## Where the Code Lives

If you followed the [Prerequisites](./chapter_2.md) chapter, you already have
the code checked out and both projects set up. The Rust project is `wxrs`, and
the Python project is `wxpy`. The code for this chapter lives at:

```text
wxrs/src/bin/ch3.rs
wxpy/wxpy/ch3/fetch_api.py
```

In Rust, a project usually has a `src/main.rs` file that runs the program, with
additional code imported as modules from other files. There is a good
[convention for package layouts in Rust](https://doc.rust-lang.org/cargo/guide/project-layout.html).
Since we want one runnable program per chapter, this project has no `main.rs`
at all. Every file in `src/bin` is compiled into its own binary, so `ch3.rs`
becomes a binary named `ch3`.

In Python, `wxpy/wxpy` is a package, and each chapter is a sub-package inside
it. That lets us run each chapter's file as a module with `python -m`.

### Starting a Project from Scratch

You do not need to run anything in this section, but it is worth seeing what
creating these projects looks like, because it is one of the first differences
between Rust and Python you will experience.

In Rust, this is as simple as running

```bash
# Create the project
cargo init wxrs

# Add a dependency
cd wxrs
cargo add reqwest --features blocking
```

This creates a new directory called `wxrs` with a Hello World example.

It also adds the `reqwest` crate to our dependencies, similar to `pip install`.
Unlike a bare `pip install` though, this will also update `Cargo.toml` with our
dependency, and create a `Cargo.lock` file that pins the `reqwest` crate to a
specific version.

The `--features` flag is used to express optional compilation features. Reqwest
has several options, described in the [crate's documentation](https://docs.rs/reqwest/latest/reqwest/#optional-features).

We will use the `blocking` feature, which gives us a simpler interface to
reqwest instead of futures that require an async runtime. We will eventually
use async to show the power of Rust's fearless concurrency.

Python too allows optional features, for example
`pip install snowflake-connector-python[pandas]`.

This used to be the part where I explained that Python makes you do all of this
by hand: create the directory, create a virtual environment, hand-write a
`pyproject.toml`, name your dependencies, then install the package locally. It
was a genuinely unflattering comparison.

`uv` has since closed most of that gap, and the Python side now looks a lot
like the Rust side:

```bash
# Create the project as an importable package
uv init --lib wxpy
cd wxpy

# Add a dependency
uv add requests
```

Like `cargo add`, `uv add` writes the dependency into `pyproject.toml` and
records the exact resolved version in a lockfile: `uv.lock` here, `Cargo.lock`
there. The `--lib` flag asks for a package layout rather than a single script,
which is what lets us organise chapters as sub-packages. The exact layout `uv`
generates has changed between versions, which is one more reason this book
ships the projects ready to go rather than asking you to recreate them.

Now, admittedly we can skip all of the above steps, create a random file
anywhere we want and run it with `python myfile.py`, but the goal here is to
build a more stable distribution that can be packaged, shared, and tested.

It is worth being honest about what this does to the comparison. For years the
Rust story here was simply better, and that was a real argument in Rust's
favour. It isn't much of one anymore.

The two manifests in the repository have grown a few dependencies for later
chapters, but the shape is the same:

```toml
# Cargo.toml
{{#include ../../wxrs/Cargo.toml}}
```

```toml
# pyproject.toml
{{#include ../../wxpy/pyproject.toml}}
```

## Fetching Air Pollution Data

To fetch from an API, we will use the `requests` package in Python and the
`reqwest` crate in Rust.

Both programs read the API key from the `OWM_APPID` environment variable, and
take the latitude and longitude from the command line arguments.

### Python

```python
# wxpy/wxpy/ch3/fetch_api.py
{{#include ../../wxpy/wxpy/ch3/fetch_api.py}}
```

### Rust

```rust,editable
// wxrs/src/bin/ch3.rs
{{#include ../../wxrs/src/bin/ch3.rs:all}}
```

A few choices are shared by both programs, and they matter more than they look:

- The URL uses `https`. The API key is part of the query string, and over plain
  `http` it would travel across the network in clear text.
- Both requests have an explicit ten second timeout. Requests has no timeout by
  default and will wait forever on a silent server. The blocking `reqwest`
  client defaults to thirty seconds. Setting it explicitly makes the two
  programs behave the same way.
- Both programs check the HTTP status before trusting the body. OpenWeatherMap
  returns a JSON error document with a `401` or `400` status when something is
  wrong, and without the check that error document would be printed as if it
  were a result.

## Setting the API Key

Both programs read the key from the environment. For a single session you can
export it directly:

```bash
export OWM_APPID=your-api-key
```

The repository also ignores a `.env` file at its root, so you can keep the key
there instead of in your shell history:

```bash
# .env, in the repository root
OWM_APPID=your-api-key
```

A `.env` file is just a list of assignments. Neither program reads it directly;
you load it into your shell before running anything. `set -a` marks every
variable defined afterwards for export, and `set +a` turns that back off:

```bash
set -a
. ./.env
set +a
```

The benchmark instructions later in the chapter assume you have done one of
these two things.

## Running the Program

Running the program is simple in both languages. We'll provide the latitude
and longitude of beautiful Fairfax, CA, [birthplace of mountain biking](https://mmbhof.org/the-museum/location/), and
nestled in the foothills of Mount Tamalpais.

Google gives the coordinates as `37.9871` and `-122.5889`

### Python

In Python, we can use `-m` to run the module directly. `uv run` takes care of
creating and using the project environment.

```bash
# in wxpy/
uv run python -m wxpy.ch3.fetch_api 37.9871 -122.5889

> {"coord":{"lon":-122.5889,"lat":37.9871},"list":[{"main":{"aqi":2},"components":{"co":181.29,"no":0.03,"no2":0.48,"o3":87.65,"so2":0.61,"pm2_5":2.63,"pm10":8.53,"nh3":0},"dt":1788747595}]}
```

### Rust

In Rust, we must first compile the program before running it. `cargo build`
builds a debug version of every binary in `src/bin` and places it under
`./target/debug`, so our `ch3.rs` becomes `./target/debug/ch3`.

We can also compile and run in one step with `cargo run`, naming the binary we
want.

```bash
# in wxrs/
cargo build
./target/debug/ch3 37.9871 -122.5889
> {"coord":{"lon":-122.5889,"lat":37.9871},"list":[{"main":{"aqi":2},"components":{"co":181.29,"no":0.03,"no2":0.48,"o3":87.65,"so2":0.61,"pm2_5":2.63,"pm10":8.53,"nh3":0},"dt":1788747595}]}

# or
cargo run --bin ch3 37.9871 -122.5889
> {"coord":{"lon":-122.5889,"lat":37.9871},"list":[{"main":{"aqi":2},"components":{"co":181.29,"no":0.03,"no2":0.48,"o3":87.65,"so2":0.61,"pm2_5":2.63,"pm10":8.53,"nh3":0},"dt":1788747595}]}
```

## Discussion

Looking at both programs, we can see a fairly similar approach to solving
this problem.

Both programs use an external library or crate (not-so-coincidentally named
requests/reqwest).

In both programs, we've created a function that takes a latitude and longitude,
fetches the results from an API and returns the results as text. We'll cover
handling structured data from JSON soon.

### Types

One obvious difference is that in Rust, we declare the types of the `lat` and
`lon` arguments, and in Python we do not.

```rust
{{#include ../../wxrs/src/bin/ch3.rs:fetch}}
```

Both `lat` and `lon` are `f32`, 32-bit floating-point numbers. That is plenty
of precision for a coordinate. The Python function has no such declaration; it
will accept whatever it is given, and the `url` line will happily format a
string, a number, or `None` into the query string.

It is easy to read too much into this. Three separate things are going on
when a program accepts a coordinate from the command line, and the languages
only differ on one of them.

**Parsing** turns the text `"37.9871"` into a number. Command line arguments
are always strings, in both languages, so this is a step somebody has to take.
Rust's `parse::<f32>()` and Python's `float()` do the same job and both reject
`"nice"`. An earlier version of the Python program skipped this step and sent
the strings straight to the API, which then rejected them with an HTTP 400.
That was a choice made in the program, not a limitation of Python.

**Type checking** asks whether the program is consistent about what it passes
around. In Rust, once `lat` is an `f32` it stays one, and calling
`get_air_pollution` with a string is a compile error. Python checks types at
runtime and only on the operations that care, so a wrong type surfaces when
something finally chokes on it, or never, if nothing does. `mypy` and similar
tools bring static checking to Python as an optional layer on top; they change
what gets caught before you run the program, not how the program runs.

**Domain validation** asks whether the value makes sense. A latitude of `91`
parses perfectly well as an `f32` and as a `float`, and neither of our programs
rejects it. Both send it to the API, which answers with a 400. Rust's type
system does not do this for you either. You could define a `Latitude` type
whose constructor refuses values outside -90 to 90, and later chapters will
lean on that pattern, but it is work you do on top of the language, in either
language.

So the honest summary is: Rust makes you parse, because there is no other way to
get an `f32`, and then holds you to that type for the rest of the program.
Python lets you defer parsing indefinitely, and it is on you to remember to do
it.

### Memory

The type declaration has one more consequence worth a paragraph. An `f32` is
four bytes, and the compiler knows that when it builds the program, so it can
lay out `lat` and `lon` without any bookkeeping at runtime. A Python `float` is
an object: the value plus a header holding a reference count and a type
pointer, around 24 bytes in total, allocated on the heap and freed when nothing
refers to it anymore. For two coordinates the difference is not worth
noticing. It becomes interesting once you have millions of them, and we will
return to memory properly when we look at ownership in the next chapter.

### Handling Errors

Another subtle but important difference is the handling of errors.

In Python, errors are exceptions. A function that can fail raises, and the
caller catches. Requests documents its exceptions well: everything it raises
inherits from
[`requests.RequestException`](https://requests.readthedocs.io/en/latest/api/#exceptions),
and `raise_for_status` raises an `HTTPError` for a `4xx` or `5xx` response.
Our program catches that one base class and exits. What the language does not
give you is any hint, at the call site, that `requests.get` can raise at all.
You know because you read the documentation, or because it raised on you once.

In Rust, errors are values. `get_air_pollution` returns
`Result<String, reqwest::Error>`, which says in the signature that the
function can fail and exactly how. Inside the function, each `?` returns the
error to the caller as soon as one appears, and `error_for_status` turns a bad
HTTP status into that same error type. The caller has to decide what to do
with the `Result`; `Result` is marked `#[must_use]`, so quietly ignoring one is
a compiler warning rather than something you silently overlook.

```rust
{{#include ../../wxrs/src/bin/ch3.rs:call}}
```

The two programs behave the same way with a bad key:

```bash
OWM_APPID=bad uv run python -m wxpy.ch3.fetch_api 37.9871 -122.5889
> Request failed: 401 Client Error: Unauthorized for url: https://api.openweathermap.org/data/2.5/air_pollution?lat=37.9871&lon=-122.5889&appid=bad
```

```bash
OWM_APPID=bad ./target/debug/ch3 37.9871 -122.5889
> Request failed: HTTP status client error (401 Unauthorized) for url (https://api.openweathermap.org/data/2.5/air_pollution?lat=37.9871&lon=-122.5889&appid=bad)
```

Both exit with a non-zero status, which is what lets the benchmark harness
later in the chapter trust that a timed run actually succeeded.

Not every failure in the Rust program is a `Result`, though. The argument
parsing in `main` uses a different tool:

```rust
{{#include ../../wxrs/src/bin/ch3.rs:parse}}
```

`expect` says that if the value is missing or `parse` fails, the program
should panic with the usage message. That is what happens with bad input:

```bash
./target/debug/ch3 nice birds

> thread 'main' panicked at src/bin/ch3.rs:37:10:
Usage: ./target/debug/ch3 [lat] [lon]: ParseFloatError { kind: Invalid }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The Python program does the equivalent with a `try`/`except ValueError` around
`float()` and prints the usage line without the backtrace. You will see `expect`
and its cousin `unwrap` used frequently in Rust. They are fine at the edge of a
program where the only sensible response is to quit, and useful while
debugging, but they are not error handling. Compare the two halves of `main`:
the fetch returns a `Result` and lets the caller choose, while the parsing
decides for the caller. We'll cover error handling in more detail soon.

## Benchmarks

Let me preface this by saying speed isn't everything. No doubt someone familiar
in Python will spend far more time learning Rust than they might ever save by
running a slightly more optimized program. But it is nice to get a sense of
the difference, and to watch how it changes as the programs get less trivial.

Let's use `hyperfine` to benchmark the two programs. We'll run each program
10 times after five warmups and take the average. Before we benchmark the Rust application,
we'll compile it using `--release` which builds a release rather than a
debug version and it should provide us with a faster application.

```bash
cargo build --release
```

Every benchmark in this book is generated by the `Makefile` in `benchmarks/`,
so you can reproduce them yourself:

```bash
# from the repo root, with OWM_APPID exported
make build-release
make -C benchmarks online 'BENCHMARK_CMD=hyperfine --warmup 5 --runs 10'
```

Under the hood that is just `hyperfine` comparing the two binaries:

```bash
hyperfine --warmup 5 --runs 10 \
    '../wxrs/target/release/ch3 30 -140' \
    '../wxpy/.venv/bin/python ../wxpy/wxpy/ch3/fetch_api.py 30 -140' \
    --export-markdown ch3_fetch_api.md
```

These results were generated against the live API after switching both
programs to HTTPS and adding the status check, using five warmups and ten
measured runs per implementation. Because both programs now exit non-zero on
an HTTP error, `hyperfine` aborts the benchmark if any run fails, so every
timed run below was a successful request.

{{#include ../../benchmarks/ch3_fetch_api.md}}

On this run, Python averaged 171.0ms and Rust 152.5ms, making Rust about 1.12x
faster for this complete command. Run-to-run standard deviations were 10.7ms
and 27.7ms respectively, and the Rust run was the noisier of the two. These
timings include startup, the TLS handshake, the HTTP request, and printing the
response.

That is a much smaller gap than the earlier plain `http` version of this
benchmark showed, and the reason is instructive. User CPU time was about
56.8ms for Python and 31.7ms for Rust. Over plain `http` the Rust program used
about 6ms of CPU; the rest is the cost of setting up a TLS connection, which
`reqwest` does with the pure-Rust `rustls` library in this build, while Python
uses the system OpenSSL through its `ssl` module. Python's interpreter startup
and imports are still a plausible contributor to the remaining difference, but
this benchmark does not isolate them from the other work each program does.

Both programs also wait on the network, and requests happen sequentially
against a live service. Latency variation affects the result, and with a gap
this small a different network day could plausibly reorder the two. A
long-lived process that reuses a client would be a different workload, and one
where the handshake cost is paid once rather than every run. No peak-memory
measurements were taken in this run.

Again, this is a trivial application with trivial requirements and performance
is not a key factor in deciding what language to build. But as we build more
intensive applications we'll keep an eye on memory and performance to see
how the gap changes.

## Summary

In this chapter we've built a simple application that fetches data from an API
and returns the results. We've seen how Rust and Python differ in their approach
to types and to handling errors, made both programs fail loudly and safely on
bad input or a bad response, and measured the runtime of both complete
programs against the live API.
