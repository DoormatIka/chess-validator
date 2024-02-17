use chess::{ChessMove, Error, File, Piece, Rank, Square};

pub fn uci_to_move(uci: &str) -> Result<ChessMove, Error> {
    let from = char_to_square(uci.chars().nth(0).unwrap(), uci.chars().nth(1).unwrap())?;
    let to = char_to_square(uci.chars().nth(2).unwrap(), uci.chars().nth(3).unwrap())?;
    let promotion = if uci.len() > 4 {
        char_to_piece(uci.chars().nth(4).unwrap())
    } else {
        None
    };
    let chess_move = ChessMove::new(from, to, promotion);
    Ok(chess_move)
}

pub fn char_to_square(file: char, rank: char) -> Result<Square, Error>{
    let file = char_to_file(file)?;
    let rank = char_to_rank(rank)?;
    let square = Square::make_square(rank, file);
    Ok(square)
}

pub fn char_to_file(file: char) -> Result<File, Error> {
    match file {
        'a' => Ok(File::A),
        'b' => Ok(File::B),
        'c' => Ok(File::C),
        'd' => Ok(File::D),
        'e' => Ok(File::E),
        'f' => Ok(File::F),
        'g' => Ok(File::G),
        'h' => Ok(File::H),
        _ => Err(Error::InvalidFile),
    }
}

pub fn char_to_rank(rank: char) -> Result<Rank, Error> {
    match rank {
        '1' => Ok(Rank::First),
        '2' => Ok(Rank::Second),
        '3' => Ok(Rank::Third),
        '4' => Ok(Rank::Fourth),
        '5' => Ok(Rank::Fifth),
        '6' => Ok(Rank::Sixth),
        '7' => Ok(Rank::Seventh),
        '8' => Ok(Rank::Eighth),
        _ => Err(Error::InvalidRank),
    }
}

pub fn char_to_piece(piece: char) -> Option<Piece>{
    match piece {
        'p' | 'P' => Some(Piece::Pawn),
        'n' | 'N' => Some(Piece::Knight),
        'b' | 'B'=> Some(Piece::Bishop),
        'r' | 'R'=> Some(Piece::Rook),
        'q' | 'Q'=> Some(Piece::Queen),
        'k' | 'K'=> Some(Piece::King),
        _ => None
    }
}