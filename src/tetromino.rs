use ratatui::style::Color;

pub type Shape = [[u8; 4]; 4];

#[derive(Clone, Copy)]
pub enum Kind {T, L, J, O, I, Z, S}

impl Kind {
    fn shape(self) -> Shape {
        match self {
            Kind::T => [
                [0, 1, 0, 0],
                [1, 1, 1, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0]
            ],
            Kind::L => [
                [0, 0, 1, 0],
                [1, 1, 1, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0]
            ],
            Kind::J => [
                [1, 0, 0, 0],
                [1, 1, 1, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0]
            ],
            Kind::O => [
                [0, 1, 1, 0],
                [0, 1, 1, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0]
            ],
            Kind::I => [
                [0, 0, 0, 0],
                [1, 1, 1, 1],
                [0, 0, 0, 0],
                [0, 0, 0, 0]
            ],
            Kind::Z => [
                [1, 1, 0, 0],
                [0, 1, 1, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0]
            ],
            Kind::S => [
                [0, 1, 1, 0],
                [1, 1, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0]
            ],
        }
    }

    pub fn color(self) -> Color {
            match self {
                Kind::I => Color::Cyan,
                Kind::O => Color::Yellow,
                Kind::T => Color::Magenta,
                Kind::S => Color::Green,
                Kind::Z => Color::Red,
                Kind::J => Color::Blue,
                Kind::L => Color::Rgb(255, 165, 0),
            }
        }
}


fn rotate(s: &Shape) -> Shape {
    let mut out = [[0u8; 4]; 4];
    for r in 0..4 {
        for c in 0..4 {
            out[c][3 - r] = s[r][c];
        }
    }
    out
}

pub struct Piece {
    pub kind: Kind,
    pub cells: Shape,
    pub x: i32,
    pub y: i32
}

impl Piece {
    pub fn spawn(kind: Kind) -> Self {
        Self {kind, cells: kind.shape(), x: 3, y: 0}
    }
    pub fn rotate_shape(&mut self) {
        self.cells = rotate(&self.cells);
    }
}
