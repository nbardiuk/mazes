use maze::Maze;
use sidewinder::Sidewinder;

mod binary_tree;
mod maze;
mod rand;
mod sidewinder;

fn main() {
    println!("{}", Sidewinder::on(Maze::new(27, 8)));
}
