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
```

This will create a new directory called `wxrs` with a Hello World example.


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
