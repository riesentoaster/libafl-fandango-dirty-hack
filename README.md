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
cargo run --example run_fandango --release -- --python-interface-path examples/run_fandango.py --fandango-file  examples/even_numbers.fan
```

### Using it in a fuzzer

There are two ways of running libafl_fandango_dirty_hack in LibAFL: As a generator and as a pseudo-mutator. The former is the obvious and ideomatic answer, the latter is handy if you are building a mutational fuzzer anyway and just want to replace your mutator. Using it as a mutator will introduce a small performance benefit (running the scheduler, cloning the input to be mutated before it is immediately overwritten again, etc.), but compared to the overhead of running Python, I find this negligable. But it also requires the corpus to not be empty (it needs to be primed) and a mutational stage to be created (make sure to only run one mutation to prevent unnecessary runtime).

There are two example fuzzers: [baby_fuzzer_generator](./examples/baby_fuzzer_generator.rs) and [baby_fuzzer_mutator](./examples/baby_fuzzer_mutator.rs). The target for both is an in-process function that parses the input to a string and then a number and checks if it is even. It will consider any number that does not fit into 128 bits as a crash and thus produce a list of crashes after some time (in the crashes directory). They can be run with the following:

```bash
cargo run --example baby_fuzzer_generator --release -- --python-interface-path examples/run_fandango.py --fandango-file examples/even_numbers.fan --cores all
cargo run --example baby_fuzzer_mutator --release -- --python-interface-path examples/run_fandango.py --fandango-file examples/even_numbers.fan --cores all
```

## Known issues
For some reason, PyO3 struggles with matching the python interpreter to the one used in the shell – specifically when it comes to imports of dependencies. You may need to manually set the python path environment variable:

```
export PYTHONPATH=$(echo .venv/lib/python*/site-packages)
```
