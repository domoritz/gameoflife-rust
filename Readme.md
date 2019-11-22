# Conway's Game of Life in Rust [![Build Status](https://travis-ci.org/domoritz/gameoflife-rust.svg)](https://travis-ci.org/domoritz/gameoflife-rust)

Implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway's_Game_of_Life) in an infinite space in Rust. Alive cells are stored in a set. To calculate the next iteration, we compute the number of neighbors for each cell that has neighbors.

I am implementing the Game of Life in different programming languages to learn about them. You can find [all of my implementations on GitHub](https://github.com/domoritz?tab=repositories&q=gameoflife).


### What I learned/liked/disliked about rust

* Why is there no fast way to initialize maps or sets in tests? I have to create the set and then insert a bunch of elements.
* `rustfmt` is slow so I cannot run it every time I save.


## Run an example

```
cargo run --verbose
```

## Run the tests

```
cargo test --verbose
```
