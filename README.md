# good-numbers
This repo contains functions written different languages, meant to serve as not-so-meaningful benchmarks.

The function counts the quantity of "good" numbers in the range from 1 to 10^9.
A good number is divisible by the sum of its digits.

Inspired by this [tweet](https://twitter.com/risboo6909/status/1075054497758629888).

Each subfolder has a Makefile.
To benchmark, just run `make` from inside one of the directories.

| Language            | Time          |
|:-------------------:|:------------- |
| Rust (parallel)     | 24.387410000s |
| Rust (sequential)   | 123.57493100s |
| Go (sequential)     | 24.672643608s |

The "parallel" Rust program uses the Rayon 1.0.3's default out-of-the-box thread pool configuration.

