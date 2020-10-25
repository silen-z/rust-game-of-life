mod conway;

use std::{
    marker::PhantomData,
    time::{Duration, Instant},
};

use conway::ConwayRules;
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use ndarray::{s, Array2, ArrayView2};
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

const UPDATE_FREQ: Duration = Duration::from_millis(100);

const BACKGROUND_COLOR: SolidSource = SolidSource {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};

const ALIVE_COLOR: Source = Source::Solid(SolidSource {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
});

const CURSOR_COLOR: Source = Source::Solid(SolidSource {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
});

type Board<'a> = ArrayView2<'a, bool>;
type Pos = (usize, usize);
trait Rules {
    fn next_cell(pos: Pos, board: Board) -> bool;
}

struct Game<R: Rules> {
    current: Array2<bool>,
    next: Array2<bool>,
    rules: PhantomData<R>,
}

impl<R: Rules> Game<R> {
    fn new(width: usize, height: usize) -> Game<R> {
        let board = Array2::default((width, height));
        Game {
            current: board.clone(),
            next: board,
            rules: Default::default(),
        }
    }

    fn set(&mut self, pos: Pos, alive: bool) {
        self.current[pos] = alive;
    }

    fn put(&mut self, shape: ArrayView2<bool>, pos: Pos) {
        for (pos, cell) in self
            .current
            .slice_mut(s![
                pos.0..pos.0 + shape.ncols(),
                pos.1..pos.1 + shape.nrows()
            ])
            .indexed_iter_mut()
        {
            *cell = shape[pos];
        }
    }

    fn step(&mut self) {
        for (pos, _alive) in self.current.indexed_iter() {
            self.next[pos] = R::next_cell(pos, self.current.view());
        }
        std::mem::swap(&mut self.next, &mut self.current);
    }
}

fn main() {
    let mut window = Window::new(
        "Game Of Life",
        WIDTH,
        HEIGHT,
        WindowOptions {
            ..WindowOptions::default()
        },
    )
    .unwrap();

    let mut game: Game<ConwayRules> = Game::new(50, 50);

    let glider = ConwayRules::glider();

    game.put(glider.view(), (0, 0));

    let size = window.get_size();

    let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);

    let mut running = false;
    let mut last_update = Instant::now();

    draw(game.current.view(), &mut dt);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        dt.clear(BACKGROUND_COLOR);

        if window.is_key_released(Key::Space) {
            running = !running;
        }

        if running && last_update.elapsed() >= UPDATE_FREQ {
            game.step();
            last_update = Instant::now();
        }

        draw(game.current.view(), &mut dt);

        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            let cell_width = (WIDTH / game.current.ncols()) as f32;
            let cell_height = (HEIGHT / game.current.nrows()) as f32;

            let x = (x / cell_width) as usize;
            let y = (y / cell_height) as usize;

            if window.get_mouse_down(MouseButton::Left) {
                game.set((x, y), true);
            }

            let mut pb = PathBuilder::new();

            pb.rect(
                x as f32 * cell_width,
                y as f32 * cell_height,
                cell_width,
                cell_height,
            );
            dt.fill(&pb.finish(), &CURSOR_COLOR, &DrawOptions::new());
        }

        window
            .update_with_buffer(dt.get_data(), size.0, size.1)
            .unwrap();
    }
}

fn draw(board: Board, dt: &mut DrawTarget) {
    let cell_width = (WIDTH / board.ncols()) as f32;
    let cell_height = (HEIGHT / board.nrows()) as f32;

    let mut pb = PathBuilder::new();

    for (pos, _) in board.indexed_iter().filter(|(_, alive)| **alive) {
        pb.rect(
            pos.0 as f32 * cell_width,
            pos.1 as f32 * cell_height,
            cell_width,
            cell_height,
        );
    }

    dt.fill(&pb.finish(), &ALIVE_COLOR, &DrawOptions::new());
}
