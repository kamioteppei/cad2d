mod model;
mod controller;
mod usecase;
mod r#const;
mod test;

use usecase::translate_figure;
// use usecase::validate_structure;

fn main() {

    translate_figure::run();
    // validate_structure::run();
}