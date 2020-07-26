use crate::PkMakeError;
pub trait Doit {
    type Err;

    fn doit(&mut self) -> Result<(), Self::Err>;

    fn build_cmd(&mut self) -> Result<Vec<String>, Self::Err> {
        Ok(vec!["".to_string()])
    }
}
