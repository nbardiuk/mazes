use crate::maze::Direction::{East, North};
use crate::maze::Maze;
use crate::rand::sample;
use rand::{thread_rng, Rng};

pub struct BinaryTree;
impl BinaryTree {
    pub fn on(mut maze: Maze) -> Maze {
        let mut rng = thread_rng();
        for cell in maze.cells() {
            let neighbours = maze
                .neighbours(&cell)
                .into_iter()
                .filter(|(direction, _)| [North, East].contains(direction))
                .map(|(_, neighbour)| neighbour)
                .collect::<Vec<_>>();

            if let Some(neighbour) = sample(rng, &neighbours) {
                maze.link(&cell, &neighbour);
            }
        }
        maze
    }
}
#[cfg(test)]
mod spec {
    use super::*;

    #[test]
    fn binary_tree_does_nothing_on_empty_maze() {
        let maze = Maze::new(0, 0);

        let maze = BinaryTree::on(maze);

        assert_eq!(maze, Maze::new(0, 0));
    }

    #[test]
    fn binary_tree_does_nothing_on_singleton_maze() {
        let maze = Maze::new(1, 1);

        let maze = BinaryTree::on(maze);

        assert_eq!(maze, Maze::new(1, 1));
    }

    #[test]
    fn binary_tree_connects_every_cell_of_maze() {
        let maze = Maze::new(20, 20);

        let maze = BinaryTree::on(maze);

        for cell in maze.cells() {
            assert_ne!(maze.links(cell).len(), 0);
        }
    }
}
