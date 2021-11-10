# A Rust Bitcoin Vanity Address Generator

This Bitcoin vanity address generator is a simple, not particular optimized or efficient program to produce vanity addresses.
The main program is single threaded and simply generates random private keys and computes addresses for those keys until a particular prefix is met.
To make better use of computing resources, a python script is provided which will run multiple instances of the vanity address generator.

## Supported Address Types

* P2TR (bech32m)

## Usage

```
rust-vanitygen prefix [merkle]

prefix - The address prefix to search for
merkle - (optional) The merkle root of the Taproot tree
```

```
usage: run_multi.py [-h] [--processes PROCESSES] [--merkle MERKLE] prefix

positional arguments:
  prefix                Prefix to search for

optional arguments:
  -h, --help            show this help message and exit
  --processes PROCESSES, -j PROCESSES
                        Number of processes
  --merkle MERKLE       Merkle root
```

## License

This project is available under the MIT License, Copyright Andrew Chow
