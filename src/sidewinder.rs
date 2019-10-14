use crate::maze::Direction::{East, North};
use crate::maze::Maze;
use crate::rand::sample;
use rand::{thread_rng, Rng};

pub struct Sidewinder;
impl Sidewinder {
    pub fn on(mut maze: Maze) -> Maze {
        let mut rng = thread_rng();
        for row in maze.rows() {
            let mut run = vec![];
            for cell in row {
                run.push(cell.clone());

                let neighbours = maze.neighbours(&cell);
                let at_eastern_boundary = !neighbours.contains_key(&East);
                let at_northen_boundary = !neighbours.contains_key(&North);

                let should_close_out = at_eastern_boundary || (!at_northen_boundary && rng.gen());

                if should_close_out {
                    if let Some(member) = sample(rng, &run) {
                        if let Some(north) = maze.neighbours(member).get(&North) {
                            maze.link(member, &north);
                        }
                    }
                    run.clear();
                } else {
                    if let Some(e) = neighbours.get(&East) {
                        maze.link(&cell, &e);
                    }
                }
            }
        }
        maze
    }
}
#[cfg(test)]
mod spec {
    use super::*;

    #[test]
    fn sidewinder_does_nothing_on_empty_maze() {
        let maze = Maze::new(0, 0);

        let maze = Sidewinder::on(maze);

        assert_eq!(maze, Maze::new(0, 0));
    }

    #[test]
    fn sidewinder_tree_does_nothing_on_singleton_maze() {
        let maze = Maze::new(1, 1);

        let maze = Sidewinder::on(maze);

        assert_eq!(maze, Maze::new(1, 1));
    }

    #[test]
    fn sidewinder_tree_connects_every_cell_of_maze() {
        let maze = Maze::new(20, 20);

        let maze = Sidewinder::on(maze);

        for cell in maze.cells() {
            assert_ne!(maze.links(cell).len(), 0);
        }
    }
}
