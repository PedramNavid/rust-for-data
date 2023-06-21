# Fetching from an API

One of the simplest examples to start with is fetching data from an API endpoint.
This is often the beginning of many data pipeline journeys.

In our first case, we will use the [OpenWeatherMap API](https://openweathermap.org/api) to fetch the current weather in
a configurable location by providing latitude and longitude.

You will need to sign up for a free account to get an API key, once you've
signed up, [create an API key](https://home.openweathermap.org/api_keys).


## Starting a Project

One of the first differences between Rust and Python you will experience is
through initializing a project.

### Rust

In Rust, this is as simple as running

```bash
cargo init wxrs
cargo add reqwest
```

This will create a new directory called `wxrs` with a Hello World example.
It will all add the reqwest crate to our dependencies


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


```python
# wxpy/wxpy/main.py
{{#include ../../wxpy/wxpy/main.py}}
```


### Rust

```rust,editable
// wxrs/src/main.rs
{{#include ../../wxrs/src/main.rs}}
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
python -m wxpy.main 37.9871 -122.5889

> {"coord":{"lon":-122.5889,"lat":37.9871},"list":[{"main":{"aqi":2},"components":{"co":178.58,"no":0.1,"no2":0.47,"o3":70.81,"so2":0.64,"pm2_5":2.58,"pm10":4.18,"nh3":0},"dt":1687221287}]}
```

### Rust

In Rust, we must first compile the program before running it. If we run
`cargo build` Rust will create a binary for us in `./target/debug/wxrs`

We can also compile and run it with one command `cargo run`


```bash
# in wxrs
cargo build
./target/debug/wxrs 37.9871 -122.5889
> {"coord":{"lon":-122.5889,"lat":37.9871},"list":[{"main":{"aqi":2},"components":{"co":178.58,"no":0.1,"no2":0.47,"o3":70.81,"so2":0.64,"pm2_5":2.58,"pm10":4.18,"nh3":0},"dt":1687221453}]}

# or
cargo run 37.9871 -122.5889
> {"coord":{"lon":-122.5889,"lat":37.9871},"list":[{"main":{"aqi":2},"components":{"co":178.58,"no":0.1,"no2":0.47,"o3":70.81,"so2":0.64,"pm2_5":2.58,"pm10":4.18,"nh3":0},"dt":1687221453}]}

```

## Discussion

Looking at both programs, we can see a fairly similar approach to solving
this problem.

Both programs use an external library or crate (not-so-coincidentally named request/reqwest).

In both programs, we've created a function that takes a latitude and longitude,
fetches the results from an API and returns the results as text. We'll cover handling
structured data from JSON soon.

### Types

One obvious difference is that in Rust, we declare the types of the lat and lon
arguments, and in Python we do not.

In Rust, as the program is compiled we know exactly how much space we need for
lat and lng in memory: 32 bits or 4 bytes. We could opt for greater precision
by using a 64-bit float or `f64` in Rust which would take 8 bytes of memory.
Because we know how much memory we need, Rust stores the values on the stack,
instead of the heap (more on this soon).

In Python, we don't know how much memory lat and lon need until runtime. Python
will allocate these values on the heap, and it turns out that Python allocates
about 24 bytes for each float.

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
python -m wxpy.main nice birds
> ❯ python -m wxpy.main nice birds
{"cod":"400","message":"wrong latitude"}
```

```bash
rust ./target/debug/wxrs nice birds

> thread 'main' panicked at 'Usage: ./wxrs/target/debug/wxrs [lat] [lon]: ParseFloatError { kind: Invalid }', src/main.rs:24:10
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
{{#include ../../wxrs/src/main.rs:20:24}}
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
hyperfine --warmup 3 --min-runs 10 'python -m wxpy.main 37.9871 -122.5889' --export-markdown ../benchmarks/chapter_3_python_wxpy.md
```

{{#include ../../benchmarks/chapter_3_python_wxpy.md}}


```bash
# in wxrs
hyperfine --warmup 3 --min-runs 10 './target/release/wxrs 37.9871 -122.5889' --export-markdown rust.md
```

{{#include ../../benchmarks/chapter_3_rust_wxrs.md}}

On my system, the Python application took an average of 336.4ms to complete,
while the Rust application was 1.4x faster at 240.7ms. Memory consumption
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
