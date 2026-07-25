# Transforming Data using Polars

In this chapter, we'll look at how to transform data using Polars in both
Python and Rust.

Polars is a "blazing fast DataFrame library" available in both Python and
Rust. When I first wrote this chapter it was reasonable to describe it as a
faster pandas with fewer features; that framing has not aged well. Polars has
since reached 1.0 on the Python side and covers most of what you would reach
for pandas to do.

The [Polars documentation](https://docs.pola.rs/) is a great resource for
getting started, and the API docs have even more detail on syntax.

One thing worth knowing up front: the two languages are on different version
numbers for the same project. The Python package is at `1.43.0` and the Rust
crate is at `0.54.4`. They are not as far apart as that makes them look.

## Getting the data

This chapter uses the [Project FeederWatch](https://feederwatch.org/explore/raw-dataset-requests/)
dataset, which is checked into the repository as a `7z` archive because the
extracted CSV is about 1.4GB. Unpack it first:

```bash
# from the repo root
make data
```

## A note on lazy vs eager

Both languages give you two ways to work: eager, where each operation runs
immediately, and lazy, where you describe the whole query and let Polars
optimise it before running anything. Lazy is where the interesting work
happens — it can push our column selection and our `valid == 1` filter down
into the CSV reader, so it never materialises the columns and rows we are
going to throw away.

Both versions below use the lazy API, which keeps the comparison honest. It is
also how you would write this in practice.

Let's look at some key differences between the syntax in Python and Rust.

## Python

```python
{{#include ../../wxpy/wxpy/ch5/ch5.py}}
```

The Python code is very concise. `pl.scan_csv` gives us a lazy frame, columns
can be selected as a list of expressions, `sort` takes a simple `descending`
argument, and nothing actually runs until the final `collect()`.

I've also included an attempt at the same logic in pandas. While largely
similar, there are a few differences, for example, in how we filter for
valid results. Pandas has no lazy mode, so it does all of the work eagerly.

```python
{{#include ../../wxpy/wxpy/ch5/ch5_pandas.py}}
```

Now let's compare the above to Rust code.

## Rust

```rust
{{#include ../../wxrs/src/bin/ch5.rs}}
```

The shape of the query is identical — scan, select, filter, group, aggregate,
join, sort — but the Rust version is roughly 60% longer.

Almost all of that extra length is types and error handling rather than logic.
A few things worth pointing out:

- `main` returns `PolarsResult<()>`, which lets us use `?` after every fallible
  call. An earlier version of this chapter was littered with `unwrap`; this
  reads better and behaves better.
- `sort` takes a `SortMultipleOptions` builder rather than a bare keyword
  argument, because Rust has no keyword arguments.
- `LazyCsvReader::new` wants a `PlRefPath`, not a `PathBuf`, so the paths go
  through `.into()`.
- The paths themselves are built with `concat!(env!("CARGO_MANIFEST_DIR"), ..)`,
  which resolves them at compile time relative to the crate. Python gets the
  same effect at runtime from `__file__`.

Overall the APIs are close enough that translating between them is mostly
mechanical.

## Benchmarks

Let's look at some benchmarks for polars in both Python and Rust, as well
as similar code in Pandas.

{{#include ../../benchmarks/ch5.md}}

The first thing to take from this table is the thing that has not changed:
**both Polars versions comfortably beat pandas**. Rust-Polars is about 3x
faster than pandas here, and Python-Polars about 9.5x. If you came to this
chapter wondering whether Polars is worth adopting, that question is settled
regardless of which language you write it in.

The second thing is that **Python-Polars is the fastest of the three**,
finishing in about 0.43s against Rust's 1.37s. That is not the result the
earlier version of this chapter reported, and it is not the result I expected.

It would be easy to quietly drop that second finding. It is more interesting to
sit with it, because it points at something that is true in general and easy to
forget.

Both Polars versions run *the same engine*. Polars is written in Rust, and the
Python package is a thin binding over that same Rust core. This benchmark was
never really Rust versus Python. It is one build of a Rust library against
another build of the same Rust library, with a small amount of Python doing the
orchestration around it. The Python interpreter barely participates: look at
the user time in the table above and you'll see both Polars runs burning
several CPU-seconds in parallel inside the engine.

So why is our build slower? I checked the two most obvious explanations and
neither held up:

- **The allocator.** Polars' own docs recommend a custom allocator and say it
  can be worth up to 25%. Swapping in `mimalloc` changed the runtime by less
  than the run-to-run noise here, so I took it back out.
- **The engine.** Recent Polars has both an in-memory and a streaming engine,
  and Python's `collect()` chooses differently than Rust's does. Forcing the
  streaming engine in Rust with `collect_with_engine(Engine::Streaming)` took
  1.43s down to 1.34s — real, but nowhere near a 3x gap.

What is left is the build itself. The Python wheels are compiled with tuning
that a plain `cargo build --release` does not apply — the
[Polars performance notes](https://docs.rs/polars/latest/polars/#performance)
recommend a nightly compiler with the `simd` and `performant` features and
`RUSTFLAGS='-C target-cpu=native'`. I have not chased that here, partly because
`target-cpu=native` produces a binary tuned to whatever machine built it, which
is at odds with pinning everything else in this repository so the numbers
reproduce.

The lesson I would take from this is the same one from the `BufWriter` aside in
the last chapter, one level up. Reaching for Rust does not hand you
performance. When you call into a library that is already written in Rust,
choosing Rust as *your* language may buy you very little — you were always
running Rust, and what actually mattered was how somebody else compiled it.

Which is worth holding next to the pandas column. The 9.5x that separates
Python-Polars from pandas came from choosing a better tool. The 3x that
separates it from our Rust build came from choosing a better *build* of the
same tool. Neither of those is a fact about Python or Rust the languages, and
picking the right library will usually take you further than picking the right
language.
