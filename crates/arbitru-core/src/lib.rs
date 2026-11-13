//! ARBITRU core: the `Game` trait, a reference game, perft-style counters and the `.arb` v0 header.

/// One of the two sides.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    First,
    Second,
}

impl Player {
    /// The opposing side.
    pub fn other(self) -> Player {
        match self {
            Player::First => Player::Second,
            Player::Second => Player::First,
        }
    }
}

/// Result of a finished game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Win(Player),
    Draw,
}

/// A game position that can generate and apply moves. Every rule set implements this.
pub trait Game: Clone {
    type Move: Copy;
    /// Legal moves; empty when the game is over.
    fn moves(&self) -> Vec<Self::Move>;
    fn apply(&mut self, m: Self::Move);
    /// `Some` once the game is over.
    fn outcome(&self) -> Option<Outcome>;
}

/// Number of leaf nodes at `depth` plies (finished games count as leaves).
pub fn perft<G: Game>(g: &G, depth: u32) -> u64 {
    if depth == 0 || g.outcome().is_some() {
        return 1;
    }
    g.moves()
        .into_iter()
        .map(|m| {
            let mut next = g.clone();
            next.apply(m);
            perft(&next, depth - 1)
        })
        .sum()
}

/// `(first-player wins, second-player wins, draws)` over every complete game.
pub fn count_outcomes<G: Game>(g: &G) -> (u64, u64, u64) {
    if let Some(o) = g.outcome() {
        return match o {
            Outcome::Win(Player::First) => (1, 0, 0),
            Outcome::Win(Player::Second) => (0, 1, 0),
            Outcome::Draw => (0, 0, 1),
        };
    }
    let mut total = (0, 0, 0);
    for m in g.moves() {
        let mut next = g.clone();
        next.apply(m);
        let c = count_outcomes(&next);
        total = (total.0 + c.0, total.1 + c.1, total.2 + c.2);
    }
    total
}

const LINES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

/// Reference implementation used to test the framework (will be replaced by `.arb` data).
#[derive(Clone)]
pub struct TicTacToe {
    cells: [Option<Player>; 9],
    turn: Player,
}

impl Default for TicTacToe {
    fn default() -> Self {
        TicTacToe {
            cells: [None; 9],
            turn: Player::First,
        }
    }
}

impl TicTacToe {
    pub fn new() -> Self {
        Self::default()
    }

    fn winner(&self) -> Option<Player> {
        LINES.iter().find_map(|l| match self.cells[l[0]] {
            Some(p) if self.cells[l[1]] == Some(p) && self.cells[l[2]] == Some(p) => Some(p),
            _ => None,
        })
    }
}

impl Game for TicTacToe {
    type Move = usize;

    fn moves(&self) -> Vec<usize> {
        if self.outcome().is_some() {
            return Vec::new();
        }
        (0..9).filter(|&i| self.cells[i].is_none()).collect()
    }

    fn apply(&mut self, m: usize) {
        self.cells[m] = Some(self.turn);
        self.turn = self.turn.other();
    }

    fn outcome(&self) -> Option<Outcome> {
        if let Some(p) = self.winner() {
            Some(Outcome::Win(p))
        } else if self.cells.iter().all(Option::is_some) {
            Some(Outcome::Draw)
        } else {
            None
        }
    }
}

/// Header of an `.arb` v0 file: `key: value` lines, `#` comments, `game` and `board` required.
#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    pub game: String,
    pub board: String,
}

/// Parse the header of an `.arb` v0 file.
pub fn parse_header(text: &str) -> Result<Header, String> {
    let (mut game, mut board) = (None, None);
    for (n, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((k, v)) = line.split_once(':') else {
            return Err(format!("line {}: expected `key: value`", n + 1));
        };
        match k.trim() {
            "game" => game = Some(v.trim().to_string()),
            "board" => board = Some(v.trim().to_string()),
            _ => {}
        }
    }
    match (game, board) {
        (Some(game), Some(board)) => Ok(Header { game, board }),
        (None, _) => Err("missing `game`".into()),
        (_, None) => Err("missing `board`".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perft_counts() {
        let g = TicTacToe::new();
        let want = [1, 9, 72, 504, 3024, 15120];
        for (d, &w) in want.iter().enumerate() {
            assert_eq!(perft(&g, d as u32), w, "depth {d}");
        }
    }

    #[test]
    fn all_games() {
        let g = TicTacToe::new();
        assert_eq!(perft(&g, 9), 255_168);
        assert_eq!(count_outcomes(&g), (131_184, 77_904, 46_080));
    }

    #[test]
    fn header() {
        let h = parse_header("# demo\ngame: Tic-tac-toe\nboard: grid 3x3\nplayers: 2\n").unwrap();
        assert_eq!(h.game, "Tic-tac-toe");
        assert_eq!(h.board, "grid 3x3");
        assert!(parse_header("game: x").is_err());
        assert!(parse_header("nonsense").is_err());
    }
}
