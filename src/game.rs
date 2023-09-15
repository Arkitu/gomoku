use kdtree::KdTree;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
enum Color {
    #[default]
    White,
    Black
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
struct Cell {
    color: Color,
    x: isize,
    y: isize
}

struct Board {
    cells: KdTree<>, 
}

struct Game {
    board: Board
}