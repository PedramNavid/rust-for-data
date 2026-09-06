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
numbers for the same project. The Python package is at `1.44.1` and the Rust
crate is at `0.55.2`. They are not as far apart as that makes them look.

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

These results were regenerated after updating to Rust Polars 0.55.2 and
Python Polars 1.44.1. The three implementations were checked against each
other: all 7,724 aggregate rows agree across all five columns after sorting
independently of display order.

**Both Polars versions beat pandas on this query.** Rust Polars takes about
1.390s, Python Polars 0.436s, and pandas 4.237s. That makes Rust Polars about
3.0x faster than pandas and Python Polars about 9.7x faster. This is evidence
for this workload, rather than a guarantee for every pandas program.

**Python Polars is still the fastest of the three**, about 3.2x faster than
our Rust build. It would be easy to quietly drop that finding. It is more
interesting to sit with it.

Both Polars implementations execute their data operations in Rust. The Python
package provides bindings to Polars, so this is not a comparison between a
Python loop and a Rust loop. It compares the Python distribution of Polars
with our local Rust build, including their execution settings. The user CPU
time exceeds wall time for both Polars programs, consistent with work running
across multiple threads inside the engine.

An earlier investigation tried a different Rust allocator and switched the
Rust query to the streaming engine. Neither change closed the gap in that
run. Those experiments were not repeated during this dependency refresh;
the checked-in Rust example continues to request the streaming engine, while
Python calls `collect()` with its default settings.

Compiler tuning, enabled features, and execution settings are possible
contributors to the remaining difference. We have not isolated their effects,
so the timings do not establish that build tuning is the cause. Pinning the
package versions alone does not make these two implementations identical.

The lesson is similar to the `BufWriter` example in the last chapter:
reaching for Rust does not hand you performance. When Python already calls a
native library, changing your application language may buy you little.
Measure the complete workload and verify that the results agree before
interpreting the timing differences.
