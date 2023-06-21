# Serializing Data

In the last chapter we fetched data from the OpenWeather API in order to get
Air Pollution data. The astute observe will have noticed that we parsed the
response as pure text, although the response was in JSON format.

The goal of this chapter is to walk through how we would take raw data and
serialize it into a structured data format, such as JSON.

We'll dive into theory in a little but let's start with practice.

## Serialization

Serialization is the process of taking data and encoding it into a known format
that can later be retrieved. There are many ways to encode data, but largely
these are broken into human-readable and binary formats.

CSVs, JSON, XML, and YAML are all human-readable serialization formats. Conversely,
many binary formats exist, such as Parquet, Avro, and Protcol Buffers. Binary
formats trade reduced readability for improved performance and size.

In the end, any data that needs to be persisted outside of a computer's memory
requires some type of serialization.

Let's look at how serialization varies across both Rust and Python.

### Python

In Python, we can serialize nearly any arbitrary data structure to JSON
using the `json` module.

```python
In [1]: import json

In [2]: my_obj = [{'a': 1, 'b': None}, "foo", "bar", ("baz", "baz")]

In [3]: json.dumps(my_obj)
Out[3]: '[{"a": 1, "b": null}, "foo", "bar", ["baz", "baz"]]'

```

Here's the updated project code that serializes the response from the OpenWeather
API.


```python
{{#include ../../wxpy/wxpy/ch4/serialized.py:all}}
```

There are a few key things to note here.

first, we're assuming the request was successful and that there is a json
response body, and that it can parse correctly. if any of these assumptions are
incorrect an exception will be raised, and we have no obvious way of knowing
what these exceptions are or which method might raise one.

```python
{{#include ../../wxpy/wxpy/ch4/serialized.py:parse_air}}
```
When parsing the response, we slice into the response body to get
various components. We're explicitly fetching keys from a dictionary under
the assumption that the payload is properly formed. There are safer dictionary
methods to use, such as `.get()` which will return `None` if the key is missing
rather than an exception, but in our case an Exception is warranted since we
can't do anything with the data if it's missing.


We also haven't explicitly typed the response from the API. This is something
we can do with `mypy` or other tools like `pydantic`, but the Python interpret
itself has no type-guarantees.

Let's look at how we might do this in Rust.

### Rust

In Rust, we'll need to install the `serde` crate as well as the `json` feature
for `reqwest`.

```bash
cargo add serde --features derive
cargo add serde_json
cargo add reqwest --features json
```

Because Rust is a typed language, we will define the struct that represents
the data we expect. The API response looks like the following:

```json
{
    "coord": {
        "lon": -122.5889,
        "lat": 37.9871
    },
    "list": [
        {
            "main": {
                "aqi": 2
            },
            "components": {
                "co": 168.56,
                "no": 0.14,
                "no2": 0.75,
                "o3": 80.11,
                "so2": 0.7,
                "pm2_5": 3.48,
                "pm10": 5.58,
                "nh3": 0
            },
            "dt": 1687308878
        }
    ]
}
```

We can define a struct that represents this data as follows:

```rust
{{#include ../../wxrs/src/bin/ch4.rs:structs}}
```

As you can see, the struct mirrors the underlying JSON structure. The `serde`
crate gives us a lot of flexibility here, in particular the section on
[Attributes](https://serde.rs/attributes.html) and the
[Examples](https://serde.rs/examples.html) are worth spending some time on.

The `reqwest` crate also provides a `json` method that will automatically
deserialize the response body into a struct.

```rust
{{#include ../../wxrs/src/bin/ch4.rs:get_json}}
```

Our function now returns an `AirPollution` struct, instead of a String, and
`reqwest`'s `json` method will automatically deserialize the response body
to the correct type.

Rust uses type inference to reduce the amount of syntax required. While
function parameters and signatures always require types, local variables
can usually be inferred by the compiler.

Let's look at how returning a typed Struct changes how we interact with
the data

```rust
{{#include ../../wxrs/src/bin/ch4.rs:parse_air}}
```

We can access the underlying fields in the struct directly. Unlike a Python
dictionary, the compiler will ensure that the fields we're accessing exist.

If we add a missing field, for example:

```rust
let foo = &body.list[0].foo;
```

And run `cargo check` we'll get the following error:

```bash

error[E0609]: no field `foo` on type `List`
  --> src/bin/ch4.rs:65:29
   |
65 |     let foo = &body.list[0].foo;
   |                             ^^^ unknown field
   |
   = note: available fields are: `main`, `components`, `dt`

For more information about this error, try `rustc --explain E0609`.
error: could not compile `wxrs` (bin "ch4") due to previous error
```

Compare to Python where we'd only get a run-time error if we tried to
access a missing field, unless we opt-in to type hints using `mypy`.

Here's the full Rust code for reference

```rust
{{#include ../../wxrs/src/bin/ch4.rs:all}}
```

### Serialization Formats

Something worth mentioning about the Rust `serde` crate is that it does not
come with any built-in serialization formats. Instead, it provides a framework
for serialization. We installed `serde_json` but there are many other formats
available, such as `serde_yaml` and `serde_avro`.

###  Why Bother?

You might be wondering why we'd go through the trouble of defining a struct
and serializing the response body into a struct. In Python, we avoid the
boilerplate, we access fields directly, we can throw a little type-hinting at
our code, we get to use `# type: ignore` freely, and if our application crashes,
well, we'll just fix it and run it again.

You are absolutely right! This is all true. However, any seasoned Python
programmer is also aware of all the ways that poorly typed code can go wrong.

If you've ever created a compute-intensive application that operates on many
gigabytes of data, you've probably run into a situation where you've had to
re-run the application because it crashed. Type-safety helps prevent these types
of issues, but types also provide another nice benefit: improved performance.

The compiler can optimize code based on the types it knows about. In Python,
we can use type-hints to help the compiler, but ultimately the Python
interpreter is still dynamically resolving types at runtime. In Rust, the
compiler knows the types at compile-time and can optimize prior to running.

## What's that little & doing?

Ah, yes, the `&`. Now we are getting into the heart of Rust. Let's look
at the code for parsing air pollution again:

```rust
{{#include ../../wxrs/src/bin/ch4.rs:parse_air}}
```

`parse_air_pollution` is a function that takes a reference to an `AirPollution`
struct. The `&` is the syntax for creating a reference. In Rust, references
are a way of passing a value to a function without transferring ownership of
the value. This is a key concept in Rust, and it's what allows Rust to
guarantee memory safety.

In Python, values are passed around using counters. Every time you use a
variable, Python's Garbage Collector keeps track of how many times it's been
used. Whenever a function that used a reference exists, the counter is
decremented. A Garbage Collector occasionally runs and cleans up all unused
references.

In Rust, there is no garbage collector. Instead, the compiler keeps track of
the lifetime of every variable. When a variable goes out of scope, the
compiler will automatically free the memory associated with the variable.

This means that you cannot use a variable after transferring ownership. For
a deeper dive into the concept of ownership, read the
[Rust Book](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html).

For example, if we tried print the value of body after assigning it,
the compiler would give us an error:

```rust
fn parse_air(body: AirPollution) {
    let foo = body;
    println!("{:?}", body);
}
```

```
error[E0382]: borrow of moved value: `body`
  --> src/bin/ch4.rs:71:22
   |
69 | fn parse_air(body: AirPollution) {
   |              ---- move occurs because `body` has type `AirPollution`, which does not implement the `Copy` trait
70 |     let foo = body;
   |               ---- value moved here
71 |     println!("{:?}", body);
   |                      ^^^^ value borrowed here after move
```

It's beyond the scope of this post to explain all the details of ownership
and references, but it's important to understand that Rust's compiler is
keeping track of the lifetime of every variable, and will not allow you to
use a variable after it's been moved.

Instead, we can use a reference to a variable. This keeps the underlying
data in the same place in memory, but allows us to pass it to a function
as a reference to the original value.

```rust
fn parse_air(body: &AirPollution) {
    let foo = body;
    println!("{:?}", body);
}
```

This has some really nice benefits when it comes to processing large amounts
of data, as data engineers tend to do.

In Python, it's not always clear when data is being copied, moved, or
referenced. In Rust, copying code is very explicit. If we didn't want to
borrow a reference in the code above, we could also copy.

```rust
fn parse_air(body: AirPollution) {
    let foo = body.clone();
    println!("{:?}", body);
}
```

For the above code to work, we would also need to implement the `Clone` trait
for the `AirPollution` struct and all of its fields:


```rust
#[derive(Debug, Clone, Deserialize)]
pub struct AirPollution {
...

```

Understanding ownership, references, and borrowing can be an uphill battle
for new Rust programmers who are used to dynamically-typed languages, but
with time and patience, it will come to you too.

## Performance

To benchmark our code, we're going to change our code to fetch an entire
forecast rather than a single day, increasing the payload from 0.5kb to
about 13kb.

In Python, we change the url and then iterate over every element in the list
provided.

```python
{{#include ../../wxpy/wxpy/ch4/serialized_benchmark.py:forecast}}
```

In Rust, we also change the url and use the common `iter().map().collect()`
pattern.

```rust
{{#include ../../wxrs/src/bin/ch4_benchmark.rs:forecast}}
```

Here are the results of the benchmarks:

{{#include ../../benchmarks/ch4_serialized.md}}

Again, we see a 1.7x improvement in performance, or about 58% faster.

### Offline Benchmarks

Benchmarking against a network connection can be a bit iffy. It also makes
it hard to test larger and larger payloads, so we'll create a large
payload file and use that for an offline benchmark.

I've created a 9mb JSON file that mirrors the payload from the OpenWeather
API, and created offline versions of the Rust and Python code to read from
a local file. The code for both can be found in the sample repository under
`wxpy/wxpy/ch4/serialized_offline_benchmark.py` and `wxrs/src/bin/ch4_offline_benchmark.rs`.

Here are the results of the offline benchmarks:

{{#include ../../benchmarks/ch4_offline_benchmark.md}}

Rust is now running twice as fast as Python for these larger payloads.

