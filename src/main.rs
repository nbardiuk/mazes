use maze::Maze;
use sidewinder::Sidewinder;

mod binary_tree;
mod maze;
mod sidewinder;

fn main() -> std::io::Result<()> {
    let maze = Sidewinder::on(Maze::new(50, 50));
    svg::save("image.svg", &maze.to_svg(10))?;
    Ok(())
}
