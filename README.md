Recommended Dependency:
- [casey/just](https://github.com/casey/just): Task runner for Mac, Win, Linux. You can install it using `brew install just` on macOS, `winget install --id=Casey.Just -e` on Windows, `cargo install just` or visit the github link for more options.

## TL;DW

```sh
# install rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# if can't run rust, reopen terminal/IDE; ensure ~/.cargo/bin is in PATH

echo "SESSION_COOKIE=your_cookie" > .env
just getinputs 7
just rust 7 2 a
```

Running other solutions, say, `rust/src/bin/day6-2-res-2.rs` instead of `rust/src/bin/day6-2.rs`, where the files are available:
```sh
just rust 6 2-res-2 a
# just suffix everything after the day and part number
```

Run with any input data:
```sh
vim _inputs/day6-abcde.txt # paste the test data into a new or existing file for that day
just rust 6 2 abcde
# you can use 2-res-2 too
```

Run without using `just`:
```sh
cd rust/<day>
cargo run --bin day-6-2 ../../_inputs/day6-a.txt
# input arg: any path to input file from cwd will do
```

Bun (TS/JS), if there is a file for it:
```sh
# install bun:
# MacOS/Linux: curl -fsSL https://bun.sh/install | bash
# Windows: powershell -c "irm bun.sh/install.ps1 | iex"

cd bun && bun install && cd ..
just getinputs 4 # if you haven't for that day yet
just bun 4 2 a
```

Bun without using `just`:
```sh
cd bun
bun run src/day4/day4-2.ts ../_inputs/day4-a.txt # input files required; any abs/rel path to input file from cwd will do
```
