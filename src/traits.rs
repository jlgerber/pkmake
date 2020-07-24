pub trait Doit {
    type Err;

    fn doit(&mut self) -> Result<(), Self::Err>;

    fn construct_command(&mut self) -> Result<Vec<String>, Self::Err> {
        Ok(vec!["".to_string()])
    }
}
