use std::time::{Instant, SystemTime, UNIX_EPOCH};
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
}
