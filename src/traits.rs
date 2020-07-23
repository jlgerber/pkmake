pub trait Doit {
    type Err;

    fn doit(&mut self) -> Result<(),Self::Err>;
}
