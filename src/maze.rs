use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Cell {
    x: usize,
    y: usize,
}
impl Cell {
    fn new(x: usize, y: usize) -> Cell {
        Cell { x, y }
    }
}

#[derive(Debug, PartialEq)]
pub struct Maze {
    width: usize,
    height: usize,
    links: Vec<Vec<usize>>,
}
impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        let size = width * height;
        Maze {
            width,
            height,
            links: vec![vec![]; size],
        }
    }

    pub fn cells(&self) -> Vec<Cell> {
        let mut result = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                result.push(Cell::new(x, y));
            }
        }
        result
    }

    pub fn rows(&self) -> Vec<Vec<Cell>> {
        let mut result = vec![];
        for y in 0..self.height {
            let mut row = vec![];
            for x in 0..self.width {
                row.push(Cell::new(x, y));
            }
            result.push(row);
        }
        result
    }

    pub fn neighbours(&self, cell: &Cell) -> HashMap<Direction, Cell> {
        let mut result = HashMap::new();
        if cell.y > 0 {
            result.insert(Direction::North, Cell::new(cell.x, cell.y - 1));
        }
        if cell.y + 1 < self.height {
            result.insert(Direction::South, Cell::new(cell.x, cell.y + 1));
        }
        if cell.x > 0 {
            result.insert(Direction::West, Cell::new(cell.x - 1, cell.y));
        }
        if cell.x + 1 < self.width {
            result.insert(Direction::East, Cell::new(cell.x + 1, cell.y));
        }
        result
    }

    pub fn link(&mut self, cell: &Cell, neighbour: &Cell) {
        let c = cell.y * self.width + cell.x;
        let n = neighbour.y * self.width + neighbour.x;
        self.links[c].push(n);
        self.links[n].push(c);
    }

    pub fn links(&self, cell: Cell) -> Vec<Cell> {
        self.links[cell.y * self.width + cell.x]
            .iter()
            .map(|index| Cell::new(index % self.width, index / self.width))
            .collect()
    }

    fn is_linked(&self, cell: Cell, direction: Direction) -> bool {
        if let Some(neighbour) = self.neighbours(&cell).get(&direction) {
            self.links(cell).contains(neighbour)
        } else {
            false
        }
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "+")?;
                if self.is_linked(Cell::new(x, y), Direction::North) {
                    write!(f, "   ")?;
                } else {
                    write!(f, "---")?;
                }
            }
            writeln!(f, "+")?;

            for x in 0..self.width {
                if self.is_linked(Cell::new(x, y), Direction::West) {
                    write!(f, " ")?;
                } else {
                    write!(f, "|")?;
                }
                write!(f, "   ")?;
            }
            writeln!(f, "|")?;
        }

        for x in 0..self.width {
            write!(f, "+---")?;
        }
        write!(f, "+")?;

        Ok(())
    }
}

#[cfg(test)]
mod spec {
    use super::*;

    #[test]
    fn empty_maze_has_no_cells() {
        let maze = Maze::new(0, 0);

        assert_eq!(maze.cells(), vec![])
    }

    #[test]
    fn maze_provides_cells() {
        let maze = Maze::new(3, 4);

        assert_eq!(
            maze.cells(),
            vec![
                Cell::new(0, 0),
                Cell::new(1, 0),
                Cell::new(2, 0),
                Cell::new(0, 1),
                Cell::new(1, 1),
                Cell::new(2, 1),
                Cell::new(0, 2),
                Cell::new(1, 2),
                Cell::new(2, 2),
                Cell::new(0, 3),
                Cell::new(1, 3),
                Cell::new(2, 3),
            ]
        )
    }

    #[test]
    fn maze_provides_rows() {
        let maze = Maze::new(3, 4);

        assert_eq!(
            maze.rows(),
            vec![
                vec![Cell::new(0, 0), Cell::new(1, 0), Cell::new(2, 0),],
                vec![Cell::new(0, 1), Cell::new(1, 1), Cell::new(2, 1),],
                vec![Cell::new(0, 2), Cell::new(1, 2), Cell::new(2, 2),],
                vec![Cell::new(0, 3), Cell::new(1, 3), Cell::new(2, 3),]
            ]
        )
    }

    #[test]
    fn maze_knows_cell_neighbours() {
        let maze = Maze::new(3, 4);

        assert_eq!(
            maze.neighbours(&Cell::new(1, 2)),
            [
                (Direction::North, Cell::new(1, 1)),
                (Direction::South, Cell::new(1, 3)),
                (Direction::West, Cell::new(0, 2)),
                (Direction::East, Cell::new(2, 2)),
            ]
            .iter()
            .cloned()
            .collect()
        )
    }

    #[test]
    fn maze_cell_neighbours_on_edge() {
        let maze = Maze::new(3, 4);

        assert_eq!(
            maze.neighbours(&Cell::new(0, 0)),
            [
                (Direction::South, Cell::new(0, 1)),
                (Direction::East, Cell::new(1, 0)),
            ]
            .iter()
            .cloned()
            .collect()
        );
        assert_eq!(
            maze.neighbours(&Cell::new(2, 3)),
            vec![
                (Direction::North, Cell::new(2, 2)),
                (Direction::West, Cell::new(1, 3)),
            ]
            .iter()
            .cloned()
            .collect()
        );
    }

    #[test]
    fn maze_links_cells() {
        let mut maze = Maze::new(3, 4);

        assert_eq!(maze.links(Cell::new(0, 0)), vec![]);

        maze.link(&Cell::new(0, 0), &Cell::new(0, 1));
        assert_eq!(maze.links(Cell::new(0, 1)), vec![Cell::new(0, 0)]);
        assert_eq!(maze.links(Cell::new(0, 0)), vec![Cell::new(0, 1)]);

        maze.link(&Cell::new(0, 0), &Cell::new(1, 0));
        assert_eq!(maze.links(Cell::new(0, 1)), vec![Cell::new(0, 0)]);
        assert_eq!(
            maze.links(Cell::new(0, 0)),
            vec![Cell::new(0, 1), Cell::new(1, 0)]
        );
    }

    #[test]
    fn display_singleton_maze() {
        let maze = Maze::new(1, 1);
        assert_eq!(
            maze.to_string(),
            "+---+\n\
             |   |\n\
             +---+"
        )
    }
    #[test]
    fn display_maze_without_links() {
        let maze = Maze::new(3, 3);
        assert_eq!(
            maze.to_string(),
            "+---+---+---+\n\
             |   |   |   |\n\
             +---+---+---+\n\
             |   |   |   |\n\
             +---+---+---+\n\
             |   |   |   |\n\
             +---+---+---+"
        )
    }
    #[test]
    fn display_maze_with_links() {
        let mut maze = Maze::new(3, 3);
        maze.link(&Cell::new(0, 0), &Cell::new(1, 0));
        maze.link(&Cell::new(0, 0), &Cell::new(0, 1));
        maze.link(&Cell::new(2, 2), &Cell::new(1, 2));
        maze.link(&Cell::new(2, 2), &Cell::new(2, 1));

        assert_eq!(
            maze.to_string(),
            "+---+---+---+\n\
             |       |   |\n\
             +   +---+---+\n\
             |   |   |   |\n\
             +---+---+   +\n\
             |   |       |\n\
             +---+---+---+"
        );
    }
}
