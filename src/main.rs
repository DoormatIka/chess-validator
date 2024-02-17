
use std::str::FromStr;
use chess::{Board, BoardStatus};

use pest::Parser;
use pest_derive::Parser;
pub mod uci_parser;
use uci_parser::uci_to_move;
#[derive(Parser)]
#[grammar = "chess.pest"]
struct ChessParser;

fn main() {
    let uci_position = "fen rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1 moves e2e3 e4e5";
    let parsed = ChessParser::parse(Rule::game, uci_position).unwrap();

    let mut fen: String = String::new();
    let mut moves: Vec<String> = Vec::new();

    for pair in parsed {
        match pair.as_rule() {
            Rule::game => {
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::fen => {
                            fen = inner_pair.as_str().to_string();
                        }
                        Rule::moves => {
                            moves = inner_pair.into_inner().map(|p| p.as_str().to_string()).collect();
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    let board = Board::from_str(&fen);
    match board {
        Ok(board) => {
            for chess_move in moves {
                let parsed_move = uci_to_move(&chess_move).unwrap();
                // e2e4 => ChessMove
                // e2e4Q => ChessMove w/ promotion
                board.make_move_new(parsed_move);
            }
            match board.status() {
                BoardStatus::Stalemate => println!("result: stalemate"),
                BoardStatus::Checkmate => println!("result: checkmate"),
                BoardStatus::Ongoing => println!("result: ongoing"),
            }
        },
        Err(err) => println!("err: {}", err),
    }

    // println!("fen: {}", fen);
    // println!("moves: {:?}", moves);

    /*
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    */
}

