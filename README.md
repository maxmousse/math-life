# Math Life

This pedagogical project aims to provide a web assembly implementation of [Conway's game of life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life). It is based on the [rust/web assembly book](https://rustwasm.github.io/docs/book/introduction.html).

It's running in an angular SPA and rendered on a canvas.

## Installation

The installation steps presume that the next software are installed on your machine:

- Node 20 and npm
- Rust and cargo

After cloning the repository and moving at the root of the folder, you can:

- Install dependencies with `npm i`.
- Build the web assembly library with `npx nx build life_game`.
- Run the angular app with `npx nx serve pwa`.
- Go to `http://localhost:4200` =)

## Next steps

For the Conway's game of life

- Add a selector to choose an interesting initial state
- Implement the speed cursor to control the game speed
- Add a fps counter
- Update the game implementation and rendering to use diff between each tick
- Try to render the game with webGL

Also, have a look on how hard it would be to make a [Lenia simulator](https://chakazul.github.io/lenia.html)
