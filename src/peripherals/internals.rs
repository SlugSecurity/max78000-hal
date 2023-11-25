use core::cell::Cell;

pub struct Handle<'a, T> {
    // we can also clone handles but make sure we increase ref counter
}

#[derive(Debug)]
pub struct Counted<T> {
    counter: Cell<usize>,
    object: T,
}
