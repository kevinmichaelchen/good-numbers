# good-numbers
This repo contains functions written different languages, meant to serve as not-so-meaningful benchmarks.

The function counts the quantity of "good" numbers in the range from 1 to 10^9.
A good number is divisible by the sum of its digits.

Inspired by this [tweet](https://twitter.com/risboo6909/status/1075054497758629888).

Each subfolder has a Makefile.
To benchmark, just run `make` from inside one of the directories.

| Language | Time       |
| -------- |:----------:|
| Rust     | 24.38741s  |
| Go       |            |
