
use std::io;
use std::str::FromStr;
use chess::{Board, BoardStatus, Game, GameResult};
use mimalloc::MiMalloc;

use pest::Parser;
use pest_derive::Parser;
pub mod uci_parser;
use uci_parser::uci_to_move;
#[derive(Parser)]
#[grammar = "chess.pest"]
struct ChessParser;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let parsed = match ChessParser::parse(Rule::game, input.as_str()) {
            Ok(parsed) => parsed,
            Err(_err) => {
                println!("board err");
                continue;
            },
        };

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
                let mut game = Game::new_with_board(board);
                for chess_move in moves {
                    let parsed_move = uci_to_move(&chess_move).unwrap();
                    game.make_move(parsed_move);
                }
                let forced_draw = game.can_declare_draw();
                if forced_draw {
                    println!("res stalemate");
                } else {
                    match game.result() {
                        Some(res) => match res {
                            GameResult::WhiteCheckmates => println!("res white checkmate"),
                            GameResult::BlackCheckmates => println!("res black checkmate"),
                            GameResult::Stalemate => println!("res stalemate"),
                            _ => println!(""),
                        }
                        None => ()
                    };
                }
            },
            Err(_err) => println!("syntax err"),
        }

    }
}

