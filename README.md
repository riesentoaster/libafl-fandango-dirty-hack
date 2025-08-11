# libafl-fandango (dirty hack edition)

This will allow you to run [Fandango](https://github.com/fandango-fuzzer/fandango) as a [LibAFL](https://github.com/aflplusplus/libafl) Generator.

It works by calling a python script using the [PyO3 interpreter](https://pyo3.rs). That script is expected to implement two functions:

```python
def setup(fan_file: str) -> A:
    # setup Fandango, start generator

def next_input(generator: A) -> bytes
    # return the next input (make sure it's converted to bytes first!)
```

## Examples

### Using the Fandango Rust Interface

Look at [the example](./examples/run_fandango.rs) for how to use the Rust interface to run Fandango. Run it using the following:

```bash
cargo run --example run_fandango -- examples/run_fandango.py examples/even_numbers.fan
```

### Using it in a fuzzer

Look at the [baby_fuzzer](./examples/baby_fuzzer.rs) for how to build a fuzzer using LibAFL and libafl-fandango.

The target is an in-process function that parses the input to a string and then a number and checks if it is even. It will consider any number that does not fit into 128 bits as a crash and thus produce a list of crashes after some time (in the crashes directory).

```bash
cargo run --example baby_fuzzer -- examples/run_fandango.py examples/even_numbers.fan
```

## Known issues
For some reason, PyO3 struggles with matching the python interpreter to the one used in the shell – specifically when it comes to imports of dependencies. You may need to manually set the python path environment variable:

```
export PYTHONPATH=$(echo .venv/lib/python*/site-packages)
```
