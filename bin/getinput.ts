
const day = process.argv[2]

fetch(`https://adventofcode.com/2024/day/${day}/input`, {headers: {cookie: `session=${process.env.AOC_SESSION}`})
