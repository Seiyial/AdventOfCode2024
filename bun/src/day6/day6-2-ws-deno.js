import { readFileSync } from 'node:fs'

const f = readFileSync('./_inputs/day6-a.txt').toString('utf-8')
console.time('exec')
const aoa = f
  .split('\n')
  .filter(a => a)
  .map(r => r.split(''))
const maxX = aoa[0].length - 1
const maxY = aoa.length - 1
const startingPosition = aoa.reduce((acc, cur, index) => {
  if (acc[0] >= 0) {
    return acc
  }
  const colIndex = cur.indexOf('^')
  if (colIndex === -1) { return acc }
  return [colIndex, index]
}, [-1, -1])

// Printed map is cool.
const printMap = (map) => {
  console.log(new Array(map[0].length).fill('=').join(''))
  console.log(map.map(a => a.map(b => b.length > 0 ? 'X' : '.').join('')).join('\n'))
  console.log(new Array(map[0].length).fill('=').join(''))
}

// Q2
{
  let position = startingPosition

  // x=0,y=0 is at top left
  // x=9,y=9 is at bottom right
  let direction = [0, -1] // initial direction is up
  let numberOfLoop = 0

  // Initialised array to keep tracked of visited pos
  const visited = []
  for (let i = 0; i <= maxY; i++) {
    visited[i] = []
    for (let j = 0; j <= maxX; j++) {
      visited[i][j] = []
    }
  }

  // Returns number representing direction
  // 1 = up
  // 2 = right
  // 3 = down
  // 4 = left
  const directionToNumber = ([dx, dy]) => {
    if (dx === 0 && dy === 1) {
      return 3
    }
    if (dx === 1 && dy === 0) {
      return 2
    }
    if (dx === 0 && dy === -1) {
      return 1
    }
    if (dx === -1 && dy === 0) {
      return 4
    }
    throw new Error(`[directionToNumber] Invalid direction - [${dx},${dy}]`)
  }

  // Set starting position as visited
  // Stores an array of direction visited.
  visited[position[1]][position[0]] = [directionToNumber([0, -1])]

  const isOutOfMap = (pos) => !pos || pos.x < 0 || pos.x > maxX || pos.y < 0 || pos.y > maxY

  const turnRight = ([dx, dy]) => {
    if (dx === 0 && dy === 1) {
      return [-1, 0]
    }
    if (dx === 1 && dy === 0) {
      return [0, 1]
    }
    if (dx === 0 && dy === -1) {
      return [1, 0]
    }
    if (dx === -1 && dy === 0) {
      return [0, -1]
    }
    throw new Error(`[turnRight] Should not reach here - [${dx},${dy}]`)
  }

  // Returns true if complete the map.
  const walk = () => {
    const [dx, dy] = direction
    const [x, y] = position
    const [targetX, targetY] = [x + dx, y + dy]
    const item = aoa[targetY]?.[targetX]
    if (isOutOfMap(item)) {
      return true
    }

    if (item === '#') {
      direction = turnRight(direction)
      return
    }

    position = [targetX, targetY]
    if (!visited[targetY][targetX]) {
      visited[targetY][targetX] = []
    }
    if (!visited[targetY][targetX].includes(directionToNumber(direction))) {
      visited[targetY][targetX].push(directionToNumber(direction))
    }
  }

  console.time('walk')
  do {
    if (walk()) {
      break
    }

    // numberOfLoop++
  } while (true)

  // printMap(visited)
  // console.log('q2 visited:', visited.reduce((acc, cur) => acc + cur.reduce((a, c) => a + (c.length > 0 ? 1 : 0), 0), 0))
  console.timeEnd('walk')

  // Return true if it ends with infinite loop
  const walkWithModify = ([x, y]) => {
    const clonedAoa = structuredClone(aoa)
    clonedAoa[y][x] = '#'
    let direction = [0, -1] // initial direction is up
    let position = startingPosition

    // Initialise 2D array to keep tracked of visited pos
    const mVisited = []
    for (let i = 0; i <= maxY; i++) {
      mVisited[i] = []
      for (let j = 0; j <= maxX; j++) {
        mVisited[i][j] = []
      }
    }
    // Set starting position as visited
    // Stores an array of direction visited.
    mVisited[position[1]][position[0]] = [directionToNumber(direction)]

    // Returns 1 if complete the map.
    // Returns 0 if continue
    // Returns -1 if infinite
    const walk = () => {
      const [dx, dy] = direction
      const [x, y] = position
      const [targetX, targetY] = [x + dx, y + dy]
      const item = clonedAoa[targetY]?.[targetX]
      if (isOutOfMap(item)) {
        return 1
      }

      if (item === '#') {
        direction = turnRight(direction)
        return 0
      }

      position = [targetX, targetY]
      if (!mVisited[targetY][targetX]) {
        mVisited[targetY][targetX] = []
      }
      if (!mVisited[targetY][targetX].includes(directionToNumber(direction))) {
        mVisited[targetY][targetX].push(directionToNumber(direction))
      } else {
        return -1
      }
    }

    let mCount = 0
    do {
      const result = walk()
      if (result === 1) {
        return false
      }
      if (result === -1) {
        return true
      }
      mCount++
      // console.log(`[${mCount}] pos:`, position)
    } while (true)
  }

  console.time('walkAll')
  let numberOfObstruction = 0
  for (let i = 0; i <= maxY; i++) {
    for (let j = 0; j <= maxX; j++) {
      // console.log("visited[j][i]:", visited[j][i]);
      if (visited[i][j].length > 0) {
        const res = walkWithModify([j, i])
        // console.log(`visited[${j}][${i}]:`, visited[j][i], res);
        if (res) {
          numberOfObstruction++
        }
      }
    }
  }

  console.timeEnd('walkAll')
  console.timeEnd('exec')
  console.log('numberOfObstruction:', numberOfObstruction)
}