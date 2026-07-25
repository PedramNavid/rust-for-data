# Concurrent Programming

One of Rust's major goals as a language is to enable *fearless concurrency*.
So much so that an entire chapter of the [Rust Book](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
is devoted to it.

In Python, concurrency is possible however we are impacted by the [GIL](https://wiki.python.org/moin/GlobalInterpreterLock).

What's really fascinating (to me, anyways) is how decisions about how memory
is managed in both languages has a direct impact on how concurrency is handled.

Before we dig into concurrency, let's take a step back and talk about memory.

## Memory

Every programming language stores objects in memory. Whether it's variables,
functions, or other data, we store these in memory to allow fast access to them
when we need them.

How languages manage memory defines the flavor and performance characteristics
of the language.

### The GIL and Python's Memory Management

In Python, the infamous Global Interpreter Lock (or GIL)
exists because objects in Python are reference counted. This means that every
object has a counter associated with it that is incremented as it is referenced
and decremented as it is removed from scope. When an object has 0 references,
it is cleared from memory, freeing up space.

Those counters are not themselves thread-safe: if two threads incremented or
decremented the same count at once, the object could be freed while still in
use, or leak forever. Rather than lock every object individually, CPython takes
a single global lock, the GIL, which guarantees that only one thread executes
Python bytecode at a time. This has the effect of serializing execution and
effectively making CPU-bound Python code single-threaded, no matter how many
threads you spawn.

To work around these limitations, CPU-bound Python code has to reach for
separate processes, typically via the `multiprocessing` module, which sidesteps
the GIL by giving each process its own interpreter. That comes with its own set
of limitations and overhead costs, since data has to be pickled and copied
between processes rather than simply shared.


