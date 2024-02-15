import { LifeGameAnnotatedPattern } from './life-game.types';

export const lifeGamePatterns = [
  {
    name: 'Empty',
    description: 'Empty pattern',
    pattern: '',
  },
  {
    name: 'Block',
    description: 'Still block',
    pattern: `
      !Name: Block
      !The most common still life.
      !www.conwaylife.com/wiki/index.php?title=Block
      OO
      OO
    `,
  },
  {
    name: 'Bee hive',
    description: 'Bee hive',
    pattern: `
      !Name: Beehive
      !Author: John Conway
      !The second most common still life.
      !www.conwaylife.com/wiki/index.php?title=Beehive
      .OO
      O..O
      .OO
    `,
  },
  {
    name: 'Loaf',
    description: 'Loaf',
    pattern: `
      !Name: Loaf
      !The third most common still life.
      !www.conwaylife.com/wiki/index.php?title=Loaf
      .OO
      O..O
      .O.O
      ..O
    `,
  },
  {
    name: 'Boat',
    description: 'Boat',
    pattern: `
      !Name: Boat
      !The only 5-cell still life.
      !www.conwaylife.com/wiki/index.php?title=Boat
      OO
      O.O
      .O
    `,
  },
  {
    name: 'Tub',
    description: 'Tub',
    pattern: `
      !Name: Tub
      !A very common still life.
      !www.conwaylife.com/wiki/index.php?title=Tub
      .O
      O.O
      .O
`,
  },
  {
    name: 'Blinker',
    description: 'Period 2',
    pattern: `
      !Name: Blinker
      !Author: John Conway
      !The smallest and most common oscillator.
      !www.conwaylife.com/wiki/index.php?title=Blinker
      OOO
    `,
  },
  {
    name: 'Toad',
    description: 'Period 2',
    pattern: `
      !Name: Toad
      !Author: Simon Norton
      !The second most common oscillator (after the blinker).
      !www.conwaylife.com/wiki/index.php?title=Toad
      .OOO
      OOO
    `,
  },
  {
    name: 'Beacon',
    description: 'Period 2',
    pattern: `
      !Name: Beacon
      !Author: John Conway
      !The third most common oscillator (after the blinker and toad).
      !www.conwaylife.com/wiki/index.php?title=Beacon
      OO
      O
      ...O
      ..OO
    `,
  },
  {
    name: 'Pulsar',
    description: 'Period 3',
    pattern: `
      !Name: Pulsar
      !Author: John Conway
      !Despite its size, this is the fourth most common oscillator (and by far the most common of period greater than 2).
      !www.conwaylife.com/wiki/index.php?title=Pulsar
      ..OOO...OOO
      .
      O....O.O....O
      O....O.O....O
      O....O.O....O
      ..OOO...OOO
      .
      ..OOO...OOO
      O....O.O....O
      O....O.O....O
      O....O.O....O
      .
      ..OOO...OOO
    `,
  },
  {
    name: 'Penta-decathlon',
    description: 'Period 15',
    pattern: `
      !Name: Pentadecathlon
      !Author: John Conway
      !10 cells placed in a row evolve into this object, which is the most natural oscillator of period greater than 3. In fact, it is the fifth or sixth most common oscillator overall, being about as frequent as the clock, but much less frequent than the blinker, toad, beacon or pulsar.
      !www.conwaylife.com/wiki/index.php?title=Pentadecathlon
      ..O....O
      OO.OOOO.OO
      ..O....O
    `,
  },
  {
    name: 'Glide spaceship',
    description: '',
    pattern: `
      !Name: Glider
      !Author: Richard K. Guy
      !The smallest, most common, and first discovered spaceship.
      !www.conwaylife.com/wiki/index.php?title=Glider
      .O
      ..O
      OOO
    `,
  },
  {
    name: 'Light-weight spaceship',
    description: '',
    pattern: `
      !Name: LWSS
      !Author: John Conway
      !The smallest known orthogonally moving spaceship, and the second most common spaceship (after the glider).
      !www.conwaylife.com/wiki/index.php?title=Lightweight_spaceship
      .O..O
      O
      O...O
      OOOO
    `,
  },
  {
    name: 'Middle-weight spaceship',
    description: '',
    pattern: `
      !Name: MWSS
      !Author: John Conway
      !The third most common spaceship (after the glider and lightweight spaceship).
      !www.conwaylife.com/wiki/index.php?title=Middleweight_spaceship
      ...O
      .O...O
      O
      O....O
      OOOOO
    `,
  },
  {
    name: 'Heavy-weight spaceship',
    description: '',
    pattern: `
      !Name: HWSS
      !Author: John Conway
      !The fourth most common spaceship (after the glider, lightweight spaceship and middleweight spaceship).
      !www.conwaylife.com/wiki/index.php?title=Heavyweight_spaceship
      ...OO
      .O....O
      O
      O.....O
      OOOOOO`,
  },
  {
    name: 'R-pentomino',
    description: '',
    pattern: `
      !Name: R-pentomino
      !The most active polyomino with less than six cells; all of the others stabilize in at most 10 generations, but the R-pentomino does not do so until generation 1103, by which time it has a population of 116.
      !www.conwaylife.com/wiki/index.php?title=R-pentomino
      .OO
      OO
      .O
    `,
  },
  {
    name: 'Diehard',
    description: '',
    pattern: `
      !Name: Die hard
      !A methuselah that vanishes at generation 130, which is conjectured to be maximal for patterns of 7 or fewer cells.
      !https://www.conwaylife.com/wiki/index.php?title=Die_hard
      ......O
      OO
      .O...OOO
    `,
  },
  {
    name: 'Acorn',
    description: '',
    pattern: `
      !Name: Acorn
      !Author: Charles Corderman
      !A methuselah that stabilizes after 5206 generations.
      !www.conwaylife.com/wiki/index.php?title=Acorn
      .O
      ...O
      OO..OOO
    `,
  },
  {
    name: 'Gosper glider gun',
    description: '',
    pattern: `
      !Name: Gosper glider gun
      !Author: Bill Gosper
      !The first known gun and the first known finite pattern with unbounded growth.
      !www.conwaylife.com/wiki/index.php?title=Gosper_glider_gun
      ........................O
      ......................O.O
      ............OO......OO............OO
      ...........O...O....OO............OO
      OO........O.....O...OO
      OO........O...O.OO....O.O
      ..........O.....O.......O
      ...........O...O
      ............OO
    `,
  },
  {
    name: 'Simkin glider gun',
    description: '',
    pattern: `
      ! Simkin glider gun
      ! Michael Simkin
      ! A true period 120 glider gun, found on April 28, 2015.
      ! www.conwaylife.com/wiki/Simkin_glider_gun
      OO.....OO........................
      OO.....OO........................
      .................................
      ....OO...........................
      ....OO...........................
      .................................
      .................................
      .................................
      .................................
      ......................OO.OO......
      .....................O.....O.....
      .....................O......O..OO
      .....................OOO...O...OO
      ..........................O......
      .................................
      .................................
      .................................
      ....................OO...........
      ....................O............
      .....................OOO.........
      .......................O.........
    `,
  },
] satisfies LifeGameAnnotatedPattern[];
