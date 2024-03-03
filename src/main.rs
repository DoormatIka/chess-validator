
use std::io;
use std::str::FromStr;
use chess::{Board, Game, GameResult, Piece};
use mimalloc::MiMalloc;

use pest::Parser;
use pest_derive::Parser;
pub mod uci_parser;
use uci_parser::uci_to_move;

#[derive(Parser)]
#[grammar = "chess.pest"]
struct ChessParser;

#[derive(Debug)]
struct PieceCount {
    pawn: u8,
    knight: u8,
    bishop: u8,
    rook: u8,
    queen: u8,
    king: u8
}

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let parsed = match ChessParser::parse(Rule::game, input.as_str()) {
            Ok(parsed) => parsed,
            Err(_err) => {
                println!("parse err");
                continue;
            },
        };

        let mut fen: String = String::new();
        let mut moves: Option<Vec<String>> = None;
        let mut verifymove: Option<String> = None;

        for pair in parsed {
            match pair.as_rule() {
                Rule::game => {
                    for inner_pair in pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::fen => {
                                fen = inner_pair.as_str().to_string();
                            }
                            Rule::moves => {
                                moves = Some(inner_pair.into_inner().map(|p| p.as_str().to_string()).collect());
                            }
                            Rule::verifymove => {
                                verifymove = Some(inner_pair.as_str().to_string());
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

                if let Some(moves) = moves {
                    for chess_move in moves {
                        let parsed_move = uci_to_move(&chess_move).unwrap();
                        game.make_move(parsed_move);
                    }
                }

                if let Some(verifymove) = verifymove {
                    match uci_to_move(&verifymove) {
                        Ok(verifymove) => {
                            let is_legal = game.current_position().legal(verifymove);
                            if is_legal {
                                println!("move legal");
                            } else {
                                println!("move illegal");
                            }
                        },
                        Err(_) => println!("move unknown"),
                    }
                }
                
                let piece_count = count_pieces(&game.current_position());
                let material = insufficient_material(&piece_count);

                let forced_draw = game.can_declare_draw();

                if forced_draw || material {
                    println!("res stalemate");
                } else {
                    match game.result() {
                        Some(res) => match res {
                            GameResult::WhiteCheckmates => println!("res white checkmate"),
                            GameResult::BlackCheckmates => println!("res black checkmate"),
                            GameResult::Stalemate => println!("res stalemate"),
                            _ => (),
                        }
                        None => println!("res ongoing")
                    };
                }
            },
            Err(_err) => println!("board err"),
        }

    }
}

fn insufficient_material(piece_count: &PieceCount) -> bool {
    piece_count.pawn <= 0 
        && piece_count.knight <= 0 
        && piece_count.bishop <= 0
        && piece_count.rook <= 0
        && piece_count.queen <= 0
        && piece_count.king == 2
}

fn count_pieces(board: &Board) -> PieceCount {
    let mut piece_count = PieceCount {
        pawn: 0,
        knight: 0,
        bishop: 0,
        rook: 0,
        queen: 0,
        king: 0,
    };

    for _ in board.pieces(Piece::Pawn).into_iter() {
        piece_count.pawn += 1;
    }
    for _ in board.pieces(Piece::Knight).into_iter() {
        piece_count.knight += 1;
    }
    for _ in board.pieces(Piece::Bishop).into_iter() {
        piece_count.bishop += 1;
    }
    for _ in board.pieces(Piece::Rook).into_iter() {
        piece_count.rook += 1;
    }
    for _ in board.pieces(Piece::Queen).into_iter() {
        piece_count.queen += 1;
    }
    for _ in board.pieces(Piece::King).into_iter() {
        piece_count.king += 1;
    }

    piece_count
}

