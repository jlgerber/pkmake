pub trait Doit {
    type Err;

    fn doit(&self) -> Result<(),Self::Err>;
}
