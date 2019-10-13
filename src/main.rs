use binary_tree::BinaryTree;
use maze::Maze;

mod binary_tree;
mod maze;

fn main() {
    print!("{}", BinaryTree::on(Maze::new(20, 8)));
}
