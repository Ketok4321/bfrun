pub enum Op {
    Inc(i8),
    MvPtr(isize),
    In,
    Out,
    LoopStart(usize),
    LoopEnd(usize)
}
