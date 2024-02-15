# Math Life

This project contains different implementations of several [cellular automaton](https://en.wikipedia.org/wiki/Cellular_automaton) like [Conway's game of life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) or [Lenia](https://fr.wikipedia.org/wiki/Lenia).

It all started as a pedagogical project to learn rust and web assembly with the [rust/web assembly book](https://rustwasm.github.io/docs/book/introduction.html).

The different simulations are running in an angular SPA and are rendered on a canvas.

## Installation

The installation steps presume that the next software are installed on your machine:

- Node 20 and npm
- Rust and cargo

After cloning the repository and moving at the root of the folder, you can:

- Install dependencies with `npm i`.
- Build the project with `npx nx build life_game` (nx will take care of the build dependency graph!).
- Run the angular app with `npx nx serve pwa`.
- Go to `http://localhost:4200`

## Next steps

- Generalize the Lenia implementation to many kernels and channels ([see extended lenia here](<https://colab.research.google.com/github/OpenLenia/Lenia-Tutorial/blob/main/Tutorial_From_Conway_to_Lenia_(w_o_results).ipynb#scrollTo=EBSBtfHlPI64>))
- The current Lenia implementation was made as a pedagogical project, without libraries optimized for mathematics. Therefore, it is pretty slow. It could be updated and use optimized libraries like [ndarray](https://docs.rs/ndarray/latest/ndarray/) and [rustfft](https://docs.rs/rustfft/latest/rustfft/)
- Add better simulation visualization and controls
- Have a look on [Flow lenia](https://sites.google.com/view/flowlenia/) that seems to be amazing!

## Useful references

All credits go to the next awesome references:

- [rust/web assembly book](https://rustwasm.github.io/docs/book/introduction.html)
- [Lenia](https://chakazul.github.io/lenia.html)
- Science Étonnante youtube channel [https://www.youtube.com/watch?v=S-W0NX97DB0] and [https://www.youtube.com/watch?v=PlzV4aJ7iMI]
- [Flow lenia](https://sites.google.com/view/flowlenia/)
