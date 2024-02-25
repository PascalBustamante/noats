

#[derive(Debug, Clone, PartialEq, Copy, Hash)]
enum Buffer {
    Src,
    Add,
}

#[derive(Debug, Clone, PartialEq, Copy, Hash)]
enum Location {

}

#[derive(Debug, Clone, Copy, Hash)]
struct Piece {
    start: usize,
    offset: usize,
    buffer: Buffer,
}

#[derive(Debug, Clone, Hash)]
pub struct PieceTable<'a, T: 'a> {
    src: &'a [T],
    adds: Vec<T>,
    pieces: Vec<Piece>,
    last_idx: usize,
    length: usize,
    reusable_insert: Option<(usize, bool)>,
    reusable_remove: Option<(Location)>,
}

/// Struct for iterating the elements of a `PieceTable`.
pub struct Iter<'a, T: 'a>
{
    table: &'a PieceTable<'a, T>,
    piece_idx: usize,
    it: std::slice::Iter<'a, T>,
}

/// Struct for iterating a range of elements in a `PieceTable`.
pub struct Range<'a, T: 'a> {
    iter: Iter<'a, T>,
    idx: usize,
    to: usize,
}

impl<'a, T: 'a> PieceTable<'a, T> {
    pub fn new() -> PieceTable<'a, T> {
        Default::default()
    }
}

impl<'a, T> Default for PieceTable<'a, T> {
    fn default() -> PieceTable<'a, T> {
        PieceTable {
            original: &[],
            adds: Vec::new(),
            pieces: Vec::new(),
            last_idx: 0,
            length: 0,
            reusable_insert: None,
            reusable_remove: None,
        }
    }
}