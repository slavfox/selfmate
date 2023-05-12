use crate::SelfMateResult::Unknown;
use pgn_reader::{BufferedReader, Outcome, RawHeader, SanPlus, Skip, Visitor};
use shakmaty::san::Suffix::Checkmate;
use shakmaty::{CastlingMode, Chess, Color, File, Position, Rank, Role, Square};
use std::mem::replace;

#[derive(Copy, Clone, Eq, PartialEq, Default)]
enum SelfMateResult {
    SelfMate,
    ForcedMate,
    #[default]
    Unknown,
}

#[derive(Default)]
struct SelfMate {
    pos: Chess,
    last_pos: Option<Chess>,
    result: SelfMateResult,
}

trait SelfMateCheck {
    fn is_selfmate(&self) -> bool;
}

impl SelfMateCheck for Chess {
    fn is_selfmate(&self) -> bool {
        let legal_moves = self.legal_moves();
        !legal_moves.is_empty()
            && legal_moves.iter().all(|m| {
                let mut pos = self.clone();
                pos.play_unchecked(m);
                pos.is_checkmate()
            })
    }
}

impl Visitor for SelfMate {
    type Result = bool;

    fn san(&mut self, san_plus: SanPlus) {
        // Skip games for which we already know the result
        if self.result != Unknown {
            return;
        }

        if let Ok(m) = san_plus.san.to_move(&self.pos) {
            let legal_moves = self.pos.legal_moves();
            if legal_moves.is_empty() {
                return;
            }
            if legal_moves.iter().all(|m| {
                let mut pos = self.pos.clone();
                pos.play_unchecked(m);
                pos.is_selfmate()
            }) {
                self.result = SelfMateResult::ForcedMate;
            } else {
                self.pos.play_unchecked(&m);
                if self.pos.is_selfmate() {
                    self.result = SelfMateResult::SelfMate;
                }
            }
        }
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true)
    }

    fn end_game(&mut self) -> Self::Result {
        self.pos = Chess::default();
        self.result == SelfMateResult::SelfMate
    }
}

fn find_selfmates(pgn: String) {
    let mut reader = BufferedReader::new_cursor(&pgn[..]);
    let mut visitor = SelfMate::default();
    while let Ok(result) = reader.read_game(&mut visitor) {
        if result.is_none() {
            break;
        }
        if result.unwrap() {
            println!("{}\n", pgn);
        }
    }
}

fn main() {
    let searchdir = std::env::args().nth(1).expect("No search directory given");
    let path = std::path::Path::new(&searchdir);
    for entry in std::fs::read_dir(path).expect("Failed to read directory") {
        let entry = entry.unwrap();
        let path = entry.path();
        // Split file on every second newline
        if let Ok(pgn) = std::fs::read_to_string(path) {
            eprintln!("Processing {:?}", entry.path());
            let pgns = pgn.split("\n\n").collect::<Vec<&str>>();
            pgns.chunks(2)
                .map(|game| game.join("\n"))
                .for_each(find_selfmates)
        }
    }
}
