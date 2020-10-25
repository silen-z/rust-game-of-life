use crate::{Board, Pos, Rules};
use ndarray::{array, Array2, ArrayView2};
pub struct ConwayRules;

impl ConwayRules {
    pub fn glider() -> Array2<bool> {
        array![
            [false, true, false],
            [false, false, true],
            [true, true, true]
        ]
    }
}

impl Rules for ConwayRules {
    fn next_cell(pos: (usize, usize), board: ArrayView2<bool>) -> bool {
        let is_alive = board[pos];

        match (is_alive, alive_neighboroughs(board, pos)) {
            (true, n) if n == 2 || n == 3 => true,
            (false, 3) => true,
            _ => false,
        }
    }
}

const NEIGHBOROUGH_INDEXES: &[(isize, isize)] = &[
    (-1, -1),
    (1, 1),
    (-1, 1),
    (1, -1),
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
];

fn alive_neighboroughs(board: Board, (x, y): Pos) -> usize {
    let x = x as isize;
    let y = y as isize;

    let mut alive = 0;

    for (nx, ny) in NEIGHBOROUGH_INDEXES {
        if wrapping_get(board, (x + nx, y + ny)) {
            alive += 1;
        }
    }
    alive
}

fn wrapping_get(board: Board, pos: (isize, isize)) -> bool {
    let width = board.ncols() as isize;
    let height = board.nrows() as isize;

    let x = (pos.0 % width + width) % width;
    let y = (pos.1 % height + height) % height;

    board[(x as usize, y as usize)]
}
