Cellular Automata in Rust
===

Simple cellular automata in rust, so far

- compute: takes a rule which is a u8 and a value which is a u8, returns 1 or 0.  Rule is the [Wolfram Code](http://en.wikipedia.org/wiki/Wolfram_code) as a byte and value is the 3 values also as a byte. Returns what the central value should be.

build with

```bash
rustc --opt-level=3 -o automata  main.rs
```

run with

```bash
./automata 30 101
```

note the different format, the command line takes 3 digits each 1 or 0