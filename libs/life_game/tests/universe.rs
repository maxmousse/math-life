//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]

extern crate life_game;
extern crate wasm_bindgen_test;

use life_game::Universe;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
pub fn test_tick() {
    // Let's create a smaller Universe with a small spaceship to test!
    let mut input_universe = Universe::new(6, 6);
    input_universe.set_cells_alive(vec![(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);

    // This is what our spaceship should look like
    // after one tick in our universe.
    let mut expected_universe = Universe::new(6, 6);
    expected_universe.set_cells_alive(vec![(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);

    // Call `tick` and then see if the cells in the `Universe`s are the same.
    input_universe.tick();

    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}
