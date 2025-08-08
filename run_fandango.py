from fandango import Fandango
from typing import Generator
import random
random.seed(0)

def setup(fan_file: str) -> Generator[bytes, None, None]:
    with open(fan_file) as f:
        fan = Fandango(f)
    generator = fan.generate_solutions()
    return generator

def next_input(generator: Generator[bytes, None, None]) -> bytes:
    return bytes(next(generator))

if __name__ == "__main__":
    gen = setup("even_numbers.fan")
    for i in range(10000):
        _ = next_input(gen)