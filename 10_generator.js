const S = `
 ###
#
 ##
   #
   #
###
`;

const E = `
####
#
###
#
#
####
`;

const N = `
#  #
## #
## #
# ##
# ##
#  #
`;

const D = `
##
# #
#  #
#  #
# #
##
`;

const U = `
#  #
#  #
#  #
#  #
#  #
 ##
`;

const space = "";

const MESSAGE_TO_DISPLAY = "SEND NUDES";

function getCodes(char) {
  const lines = char.split("\n");
  const result = [];
  for (let y = 0; y < lines.length - 1; y++) {
    const chars = lines[y + 1].split("");
    for (let x = 0; x < chars.length; x++) {
      if (chars[x] === "#") {
        result.push([x, y]);
      }
    }
  }
  return result;
}

const codes = Object.fromEntries(
  Object.entries({ S, E, N, D, U, " ": space }).map(([key, value]) => [
    key,
    getCodes(value),
  ])
);

function type(str) {
  const chars = str.split("").map((c) => codes[c]);
  const result = [];
  let base_x = 2; // the first 2 chars are unreliable
  let base_y = 0;

  for (c of chars) {
    c.forEach(([x, y]) => result.push([base_x + x, base_y + y]));
    base_x += c.length === 0 ? 3 : 5;
  }

  return [result, base_x - 1];
}

let [coords, max_x] = type(MESSAGE_TO_DISPLAY);

const scanWidth = Math.ceil(max_x / 10) * 10;
const scanHeight = 6;
const frames = scanWidth * scanHeight;

const points = new Set(coords.map((c) => c[0] + c[1] * scanWidth));
// console.log(points);

let position = 1;
// start at 2: the first two can't be changed and all it matters is from that point
let crt = 2;
let lastLine = 0;

const operations = [];
while (crt < frames) {
  const wouldLit =
    Math.abs(position - crt) <= 1 ||
    (position === 0 && crt % scanWidth === scanWidth - 1) ||
    (position === scanWidth - 1 && crt % scanWidth === 0);
  const shouldBeLit = points.has(crt);

  // console.log(crt, wouldLit, shouldBeLit);

  const jmpOffset = Math.floor(crt / scanWidth) !== lastLine ? -scanWidth : 0;
  lastLine = Math.floor(crt / scanWidth);

  if (wouldLit && !shouldBeLit) {
    const move =
      (points.has(crt + 1) ? 1 : 4 + Math.floor(Math.random() * 10)) +
      jmpOffset;
    operations.push(`addx ${move}`);
    position += move;
    crt += 2;
  } else if (!wouldLit && shouldBeLit) {
    const offset = points.has(crt + 1) ? 0 : -1;
    const move = ((crt + offset) % scanWidth) - position + jmpOffset;
    operations.push(`addx ${move}`);
    position += move;
    crt += 2;
  } else if (!shouldBeLit && !points.has(crt + 1)) {
    // put in an `add` just for the lulz (it adds in more randomness)
    const move = 8 + Math.floor(Math.random() * 10) + jmpOffset;
    operations.push(`addx ${move}`);
    position += move;
    crt += 2;
  } else if (jmpOffset !== 0) {
    operations.push(`addx ${jmpOffset}`);
    position += jmpOffset;
  } else {
    operations.push(`noop`);
    crt++;
  }
  // console.log(crt, position, jmpOffset);
}

console.log(operations.join("\n"));
console.log("");
console.log("Config: width = " + scanWidth);

// Print result to check everything looks alright
// const result = [];
// coords.forEach(([x, y]) => {
//   if (!result[x]) {
//     result[x] = [];
//   }
//   result[x][y] = "#";
// });

// for (let y = 0; y < 8; y++) {
//   let str = "";
//   for (let x = 0; x < base_x; x++) {
//     str += result[x]?.[y] ?? ".";
//   }
//   console.log(str);
// }

// Unfold
// let maxX = base_x - 1;
// let maxY = 7;

// const folds = [];
// for (let f = 0; f < 12; f++) {
//   const fold = (() => {
//     let ret;
//     if (Math.random() > 0.5) {
//       ret = ["y", maxY];
//       maxY = maxY * 2;
//     } else {
//       ret = ["x", maxX];
//       maxX = maxX * 2;
//     }
//     return ret;
//   })();

//   coords.sort(() => (Math.random() > 0.5 ? 1 : -1));
//   const newCoords = coords.slice(0, Math.floor((coords.length * 2) / 3));
//   // Duplicate the last half
//   coords.slice(Math.floor(coords.length / 2)).forEach(([x, y]) => {
//     if (fold[0] === "x") {
//       newCoords.push([fold[1] + (fold[1] - x), y]);
//     } else {
//       newCoords.push([x, fold[1] + (fold[1] - y)]);
//     }
//   });
//   coords = newCoords;
//   folds.push(fold);
// }

// const coords_s = coords.map((c) => c.join(",")).join("\n");
// const folds_s = folds
//   .reverse()
//   .map((f) => `fold along ${f[0]}=${f[1]}`)
//   .join("\n");

// console.log(`${coords_s}

// ${folds_s}`);
