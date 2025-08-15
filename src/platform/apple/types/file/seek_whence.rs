#[derive(Debug, Copy, Clone)]
pub enum SeekWhence {
    StartPos(u64),
    CurrentPos(i64),
    EndPos(i64),
}
