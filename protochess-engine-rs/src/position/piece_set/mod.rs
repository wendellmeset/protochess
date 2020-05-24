//Pieces that a player has
pub mod movement_pattern;
use crate::types::bitboard::Bitboard;
use crate::types::{PieceType};
use crate::position::piece_set::movement_pattern::MovementPattern;

/// Represents a set of pieces for a player
/// custom is a vec of custom piece types
/// holding tuples mapping Bitboards to their char representation and MovementPattern
/// chars should not contain k, q, r, p, or n
pub struct PieceSet {
    pub occupied: Bitboard,
    pub king: Bitboard,
    pub queen: Bitboard,
    pub bishop: Bitboard,
    pub knight: Bitboard,
    pub rook: Bitboard,
    pub pawn: Bitboard,
    pub custom: Vec<(char, Bitboard, MovementPattern)>,
}

impl PieceSet {
    pub fn new() -> PieceSet {
        PieceSet {
            occupied: Bitboard::zero(),
            king: Bitboard::zero(),
            queen: Bitboard::zero(),
            bishop: Bitboard::zero(),
            knight: Bitboard::zero(),
            rook: Bitboard::zero(),
            pawn: Bitboard::zero(),
            custom: Vec::new(),
        }
    }

    pub fn piece_bb_at(&mut self, index: usize) -> Option<&mut Bitboard>{
        if self.king.bit(index).unwrap(){
            Some(&mut self.king)
        }else if self.queen.bit(index).unwrap(){
            Some(&mut self.queen)
        }else if self.bishop.bit(index).unwrap(){
            Some(&mut self.bishop)
        }else if self.knight.bit(index).unwrap(){
            Some(&mut self.knight)
        }else if self.rook.bit(index).unwrap(){
            Some(&mut self.rook)
        }else if self.pawn.bit(index).unwrap(){
            Some(&mut self.pawn)
        }else{
            for (_c, b, m_p) in self.custom.iter_mut(){
                if b.bit(index).unwrap(){
                    return Some(b);
                }
            }
            None
        }
    }

    pub fn piecetype_at(&self, index: usize) -> Option<PieceType>{
        if self.king.bit(index).unwrap(){
            Some(PieceType::King)
        }else if self.queen.bit(index).unwrap(){
            Some(PieceType::Queen)
        }else if self.bishop.bit(index).unwrap(){
            Some(PieceType::Bishop)
        }else if self.knight.bit(index).unwrap(){
            Some(PieceType::Knight)
        }else if self.rook.bit(index).unwrap(){
            Some(PieceType::Rook)
        }else if self.pawn.bit(index).unwrap(){
            Some(PieceType::Pawn)
        }else{
            for (c, b, m_p) in self.custom.iter(){
                if b.bit(index).unwrap(){
                    return Some(PieceType::Custom(*c));
                }
            }
            None
        }
    }

    //Recomputes occupied bb
    pub fn update_occupied(&mut self){
        self.occupied = Bitboard::zero();
        self.occupied |= &self.king;
        self.occupied |= &self.queen;
        self.occupied |= &self.bishop;
        self.occupied |= &self.knight;
        self.occupied |= &self.rook;
        self.occupied |= &self.pawn;
        for (_c, bb, m_p) in &self.custom {
            self.occupied |= bb;
        }
    }
}