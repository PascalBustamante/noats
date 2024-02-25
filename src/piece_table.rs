

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