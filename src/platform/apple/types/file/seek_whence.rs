pub enum SeekWhence {
    StartPos(usize),
    CurrentPos(isize),
    EndPos(isize),
}