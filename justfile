set quiet
set working-directory := "."

bun DAY PUZZLE:
  echo "Bun :: day {{DAY}} :: {{PUZZLE}}"
  sleep .5
  cd bun && bun run src/day-{{DAY}}/bun_{{PUZZLE}}.ts ./_inputs/day-{{DAY}}-{{PUZZLE}}.txt

rust DAY PUZZLE TESTORACTUAL='t':
  echo "Rust :: day {{DAY}} :: {{PUZZLE}}"
  sleep .5
  cd rust/day{{DAY}} && cargo run --bin day{{DAY}}-{{PUZZLE}} -- {{justfile_directory()}}/_inputs/day{{DAY}}-{{TESTORACTUAL}}.txt

rustnew DAY:
  cd rust && cargo new day{{DAY}} && cd day{{DAY}} && cargo add aoc --path "../aoc" &&\
  mkdir src/bin && touch src/bin/day{{DAY}}-1.rs && touch src/bin/day{{DAY}}-2.rs