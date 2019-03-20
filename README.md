# A Partial Game of Life in Rust

There's only a demo, hard-coded game board at the moment. If you just want to
see it, run:

    docker run -i --rm -e TERM mqsoh/game-of-life-in-rust

You might need to `reset` your terminal afterwards because I'm not properly
cleaning up ncurses.

The [doc directory](./doc) has a couple things I wrote as I was messing around.

If you want to hack on it, run `make shell` which will bring up a Docker
container with Rust installed. (It'll take a while the first time.) Then you
can `cargo run` and `cargo test`.
