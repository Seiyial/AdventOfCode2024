set quiet
set working-directory := "."
set dotenv-load

bun DAY PUZZLE TESTORACTUAL='t':
  echo "Bun :: day {{DAY}} :: {{PUZZLE}}"
  sleep .5
  cd bun && bun run src/day{{DAY}}/day{{DAY}}-{{PUZZLE}}.ts {{justfile_directory()}}/_inputs/day{{DAY}}-{{TESTORACTUAL}}.txt

bunnew DAY:
  cd bun && mkdir src/day{{DAY}} && touch src/day{{DAY}}/day{{DAY}}-1.ts && touch src/day{{DAY}}/day{{DAY}}-2.ts

rust DAY PUZZLE TESTORACTUAL='t':
  echo "Rust :: day {{DAY}} :: {{PUZZLE}}"
  sleep .5
  cd rust/day{{DAY}} && cargo run --bin day{{DAY}}-{{PUZZLE}} -- {{justfile_directory()}}/_inputs/day{{DAY}}-{{TESTORACTUAL}}.txt

rustnew DAY:
  cd rust && cargo new day{{DAY}} && cd day{{DAY}} && cargo add aoc --path "../aoc" &&\
  mkdir src/bin && touch src/bin/day{{DAY}}-1.rs && touch src/bin/day{{DAY}}-2.rs

inputsnew DAY:
  cd _inputs && touch day{{DAY}}-t.txt && touch day{{DAY}}-t2.txt && touch day{{DAY}}-a.txt && curl -H "cookie: session=${AOC_SITE_COOKIE_FOR_FETCH}" https://adventofcode.com/2024/day/{{trim_end_match(s, 0)}}/input | day{{DAY}}-a.txt

