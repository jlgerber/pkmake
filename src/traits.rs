pub trait Doit {
    type Err;

    fn doit(&mut self) -> Result<(), Self::Err>;

    fn construct_command(&self) -> Result<String, Self::Err> {
        Ok("".to_string())
    }
}
