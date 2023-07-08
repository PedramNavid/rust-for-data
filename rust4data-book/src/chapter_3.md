# Fetching from an API

One of the simplest examples to start with is fetching data from an API endpoint.
This is often the beginning of many data pipeline journeys.

In our first case, we will use the [OpenWeatherMap API](https://openweathermap.org/api) to fetch the current weather in
a configurable location by providing a latitude and longitude on the command line.

You will need to sign up for a free account to get an API key, once you've
signed up, [create an API key](https://home.openweathermap.org/api_keys).


## Starting a Project

One of the first differences between Rust and Python you will experience is
through initializing a project.

### Rust

In Rust, this is as simple as running

```bash
# Create the project
cargo init wxrs

# Add a dependency
cd wxrs
cargo add reqwest --features blocking
```

This will create a new directory called `wxrs` with a Hello World example.

It will also add the `reqwest` crate to our dependencies, similar to `pip install`.
Unlike `pip install` though, this will also update `Cargo.toml` with our dependency,
and create a `Cargo.lock` file that will lock the `reqwest` crate to a specific version.

The `--features` flag is used to express optional compilation features. Reqwest
has several options, described in the [crate's documentation](https://docs.rs/reqwest/latest/reqwest/#optional-features).

We will use the `blocking` feature, which will allow us to use the blocking API.
It gives us a simpler interface to reqwest instead of futures that require
an async runtime. We will eventually use async to show the power of Rust's
fearless concurrency.

In Python, we have to manually maintain dependencies and create lockfiles through
updating `setup.py` or `pyproject.toml`, or using a tool like `pipenv` or `poetry`.

We are not using any specific features, but Python too allows optional features,
for example `pip install snowflake-connector-python[pandas]`.

```toml
# Cargo.toml
{{#include ../../wxrs/Cargo.toml}}
```


### Python

In Python, we create the directory manually, create a virtual environment,
hand-write a [pyproject.toml](https://setuptools.pypa.io/en/latest/userguide/quickstart.html)
file, name our dependencies, and then install the Python
package locally.

```bash
# Python venv stuff
pyenv virtualenv wxpy
pyenv shell wxpy
pip install --upgrade pip build

# Create the project directory and pyproject.toml
mkdir -p wxpy/wxpy
cd wxpy
vim pyproject.toml
```

```toml
# pyproject.toml
{{#include ../../wxpy/pyproject.toml}}
```

```bash

pip install -e .
```

Now, admittedly we can skip all of the above steps, create a random file
anywhere we want and run it with `python myfile.py`, but the goal here is to
build a more stable distribution that can be packaged, shared, and tested.

## Fetching the Weather

Now that we have a project, let's fetch the weather. To fetch from an API,
we will use the `requests` package in Python and the `reqwest` package in Rust.

We will read the API key from the environment variable, and get the latitude
and longitude from the command line arguments.

Given that I'm in California, it only makes sense to start with the
[Air Pollution API](https://openweathermap.org/api/air-pollution).


### Python

In Python, we'll create a folder for this chapter to keep code organized,
and then run that file directly.

```bash
mkdir wxpy/wxpy/ch3
```

```python
# wxpy/wxpy/ch3/fetch_api.py
{{#include ../../wxpy/wxpy/ch3/fetch_api.py}}
```


### Rust

In Rust, usually we'll have a `main.rs` file that runs our code, with
additional code imported as modules from other files. There's a great
[convention for package layouts in Rust](https://doc.rust-lang.org/cargo/guide/project-layout.html).

But since we want to execute our code directly, and we'll have multiple
binaries, we'll create a `bin` folder and save the code for `ch3` there.

```bash
mkdir wxrs/src/bin
```

```rust,editable
// wxrs/src/bin/ch3.rs
{{#include ../../wxrs/src/bin/ch3.rs}}
```

## Running the Program

Running the program is simple in both languages. We'll provide the latitude
and longitude of beautiful Fairfax, CA, [birthplace of mountain biking](https://mmbhof.org/the-museum/location/), and
nestled in the foothills of Mount Tamalpais.

Google gives the coordinates as `37.9871` and `-122.5889`

### Python

In Python, we can use `-m` to run the module directly.

```bash
# in wxpy/wxpy
# export OPENWEATHERMAP_API_KEY=your-api-key
python -m wxpy.ch3.fetch_api 37.9871 -122.5889

> {"coord":{"lon":-122.5889,"lat":37.9871},"list":[{"main":{"aqi":2},"components":{"co":178.58,"no":0.1,"no2":0.47,"o3":70.81,"so2":0.64,"pm2_5":2.58,"pm10":4.18,"nh3":0},"dt":1687221287}]}
```

### Rust

In Rust, we must first compile the program before running it. If we run
`cargo build` Rust will create a binary for us in `./target/debug/wxrs`

We can also compile and run it with one command `cargo run`

When using `cargo build` Rust will a debug version of our application in
`./target/debug` for both the `main.rs` file which will be named `wxrs` as
well for any files located in `src/bin`, such as `ch3.rs`


```bash
# in wxrs/
cargo build
./target/debug/ch3 37.9871 -122.5889
> {"coord":{"lon":-122.5889,"lat":37.9871},"list":[{"main":{"aqi":2},"components":{"co":178.58,"no":0.1,"no2":0.47,"o3":70.81,"so2":0.64,"pm2_5":2.58,"pm10":4.18,"nh3":0},"dt":1687221453}]}

# or
cargo run --bin ch3 37.9871 -122.5889
> {"coord":{"lon":-122.5889,"lat":37.9871},"list":[{"main":{"aqi":2},"components":{"co":178.58,"no":0.1,"no2":0.47,"o3":70.81,"so2":0.64,"pm2_5":2.58,"pm10":4.18,"nh3":0},"dt":1687221453}]}

```

## Discussion

Looking at both programs, we can see a fairly similar approach to solving
this problem.

Both programs use an external library or crate (not-so-coincidentally named
request/reqwest).

In both programs, we've created a function that takes a latitude and longitude,
fetches the results from an API and returns the results as text. We'll cover
handling structured data from JSON soon.

### Types

One obvious difference is that in Rust, we declare the types of the lat and lon
arguments, and in Python we do not. The trouble with talking about types is
that it inevitably leads to a discussion of memory, which can devolve into a
conversation around null pointer references, which we will largely avoid
until the next chapter, but here's a light introduction.

In the Rust code, we've very explicitly defined the types for our function:

```rust
{{#include ../../wxrs/src/bin/ch3.rs:1}}
```

Both `lat` and `lon` are `f32` or 32-bit floats. These are floating-point numbers
that take exactly 32-bits of memory. The compiler knows exactly how much space
to reserve for these values: 32-bits, or 4-bytes.

Given that `lat` and `lon` doesn't require much precision beyond a few
decimals, `f32` seems like the best choice for our code. We could even opt for
greater precision by using a 64-bit float or `f64` in Rust which would take 8
bytes of memory.

Because we know exactly how much memory we need for these variables, Rust
is able to store these values on the heap.

In Python, we don't know how much memory lat and lon need until runtime because
Python will accept anything in this function.

```python
{{#include ../../wxpy/wxpy/ch3/fetch_api.py:7:8}}
```

We could pass it a string, numbers, another function, or even `None`.

```python
>>> def join_two(a, b):
...     return f"a+b={a}+{b}"
...
>>> join_two(1,2)
'a+b=1+2'
>>> join_two(None, None)
'a+b=None+None'
>>> join_two(join_two, join_two)
'a+b=<function join_two at 0x7f8f7f7de980>+<function join_two at 0x7f8f7f7de980>'
>>> join_two(join_two, join_two(join_two, join_two))
'a+b=<function join_two at 0x7f8f7f7de980>+a+b=<function join_two at 0x7f8f7f7de980>+<function join_
two at 0x7f8f7f7de980>'
```

Even the `url` line will not fail, because in Python duck-typing allows
us great flexibility in what we do with variables. We can pass numbers into an
f-string for concatenation just as easily as we can pass characters.

Python
will allocate these values on the heap, and it turns out that Python allocates
about 24 bytes for each float there. The actual values are stored in a private
heap.

Now, the difference between 24 bytes and 8 bytes is trivial for an application
such as this, and even on the most memory-constrained devices it's not worth
noting. But it's important to know that heap allocation is slower, and even
small applications may iterate over millions of values. Small differences can
add up.

You might then ask yourself: what about mypy? Doesn't that give us typing?
Mypy is a static type checker, but it doesn't change the underlying compilation
of Python code. It can provide hints as to what you expect the types to be, but
it doesn't change how memory is allocated.

### Handling Errors

Another subtle but important difference is the handling of errors. In Python,
errors are handled as exceptions that are caught. Knowing when to catch an
exception is mostly an art. It's difficult to know which functions throw exceptions,
what exceptions to expect, and when to deal with them.

In Rust, errors are handled as values that are returned. This is a much more
explicit approach, and it's easier to know what errors to expect and how to
handle them. In fact, if a function returns a `Result` type, the compiler will
force you to handle the error. This is a huge benefit to Rust, and it's one of
the reasons why Rust is so reliable.

Let's take a closer look at an exception we haven't caught yet. If we run the
Python program with invalid arguments, we get a `ValueError` exception.

```bash
python -m wxpy.ch3.fetch_api nice birds
> ❯ python -m wxpy.ch3.fetch_api nice birds
{"cod":"400","message":"wrong latitude"}
```

```bash
./target/debug/ch3 nice birds

> thread 'main' panicked at 'Usage: ./target/debug/ch3 [lat] [lon]: ParseFloatError { kind: Invalid }', src/main.rs:24:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

What happened here?

In Python, we didn't check that our input is valid, and so the application
send incorrect values to the API, which returned an error message. Fortunately,
we get a million API requests for free a month, so this one doesn't cost us much.

In Rust, the application panicked because it could not parse the inputs we provided
as a float. On line 23:24 we call `parse` on the arguments, and we expect them
to be floats.

```rust
{{#include ../../wxrs/src/bin/ch3.rs:20:24}}
```

the `expect` method tells Rust that if `parse` failed to convert the input,
then the application must panic. We output the `usage` message and exit.

You will see `expect` and its cousin `unwrap` used frequently in Rust. They are
useful for debugging, but they are not the best way to handle errors. We'll
cover error handling in more detail soon.


## Benchmarks

Let me preface this by saying speed isn't everything. No doubt someone familiar
in Python will spend far more time learning Rust than they might ever save by
running a slightly more optimized program. But it is nice to get a sense of


Let's use `hyperfine` to benchmark the two programs. We'll run each program
10 times and take the average. Before we benchmark the Rust application,
we'll compile it using `--release` which builds a release rather than a
debug version and it should provide us with a faster application.

```bash
cargo build --release
```

```bash
# in wxpy
hyperfine --warmup 3 --min-runs 10 \
    'python -m wxpy.ch3.fetch_api 37.9871 -122.5889' \
    './target/release/ch3 37.9871 -122.5889' \
    --export-markdown ../benchmarks/ch3_fetch_api.md
```

{{#include ../../benchmarks/ch3_fetch_api.md}}

On my system, the Python application took an average of 335ms to complete,
while the Rust application was 1.7x faster at 198ms. Memory consumption
was also lower in Rust, with the Python application using 26MB vs only 10MB in
Rust.

Again, this is a trivial application with trivial requirements and performance
is not a key factor in deciding what language to build. But as we build more
intensive applications we'll keep an eye on memory and performance to see
how the gap changes.

## Summary

In this chapter we've built a simple application that fetches data from an API
and returns the results. We've seen how Rust and Python differ in their approach
to handling errors and types, and we've seen how Rust can be faster and more
memory efficient than Python.
