use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use ratatui::style::Color;

use crate::tetromino::{Kind, Piece};

pub const BOARD_H: usize = 20;
pub const BOARD_W: usize = 10;

pub struct Game {
    pub board: [[Option<Color>; BOARD_W]; BOARD_H],
    pub current: Piece,
    pub next: Kind,
    pub score: u32,
    pub lines: u32,
    pub game_over: bool,
    last_fall: Instant,
    rng: u64,
}

impl Game {
    pub fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x2545_F491_4F6C_DD1D)
            | 1;
        let mut game = Self {
            board: [[None; BOARD_W]; BOARD_H],
            current: Piece::spawn(Kind::I),
            next: Kind::O,
            score: 0,
            lines: 0,
            game_over: false,
            last_fall: Instant::now(),
            rng: seed
        };
        game.current = Piece::spawn(game.random_kind());
        game.next = game.random_kind();
        game
    }

    fn random_kind(&mut self) -> Kind {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        Kind::ALL[(self.rng % Kind::ALL.len() as u64) as usize]
    }

    pub fn move_piece(&mut self, dx: i32, dy: i32) -> bool {
        let mut moved = self.current;
        moved.x += dx;
        moved.y += dy;
        if self.fits(&moved) {
            self.current = moved;
            true
        } else {
            false
        }
    }

    pub fn drop_one(&mut self) {
        let dropped = self.move_piece(0, 1);
        if !dropped {
            self.lock_piece();
        }
    }

    pub fn rotate(&mut self) {
        let mut rotated = self.current;
        rotated.rotate_shape();
        for move_about in [0, -1, 1, -2, 2] {
            let mut candidate = rotated;
            candidate.x += move_about;
            if self.fits(&candidate) {
                self.current = candidate;
                return;
            }
        }
    }

    pub fn update(&mut self) {
        if self.game_over {
            return;
        }
        if self.last_fall.elapsed() >= self.fall_interval() {
            self.last_fall = Instant::now();
            let dropped = self.move_piece(0, 1);
            if !dropped {
                self.lock_piece();
            }
        }
    }

    fn fall_interval(&self) -> Duration {
        let level = (self.lines / 10) as u64;
        Duration::from_millis(800u64.saturating_sub(level).max(200))
    }


    /// Generated via Claude
    /// Cells above the board (y < 0) are allowed so pieces can rotate at the top.
    fn fits(&self, piece: &Piece) -> bool {
        piece.blocks().all(|(x, y)| {
            x >= 0
                && x < BOARD_W as i32
                && y < BOARD_H as i32
                && (y < 0 || self.board[y as usize][x as usize].is_none())
        })
    }

    /// Generated via Claude
    fn lock_piece(&mut self) {
        let color = self.current.kind.color();
        for (x, y) in self.current.blocks() {
            if y < 0 {
                self.game_over = true;
                return;
            }
            self.board[y as usize][x as usize] = Some(color);
        }
        self.clear_lines();

        self.current = Piece::spawn(self.next);
        self.next = self.random_kind();
        self.last_fall = Instant::now();
        if !self.fits(&self.current) {
            self.game_over = true;
        }
    }

    /// Generated via Claude
    fn clear_lines(&mut self) {
        let mut write = BOARD_H;
        for read in (0..BOARD_H).rev() {
            if self.board[read].iter().all(|c| c.is_some()) {
                continue;
            }
            write -= 1;
            self.board[write] = self.board[read];
        }
        for row in &mut self.board[..write] {
            *row = [None; BOARD_W];
        }

        let cleared = write as u32;
        self.lines += cleared;
        self.score += match cleared {
            1 => 5,
            2 => 10,
            3 => 15,
            4 => 20,
            _ => 0,
        };
    }
}
