//! Traits
//!
//! # Doit Trait
//! The Doit trait provides two methods:
//! - **doit** - responsible for executing a target's underlying pk command(s)
//! - **build_cmd** - responsible for generating a vector of pk command strs
//!
//! # Tabulate Trait
//! Responsible for generating and styling a table. Used to report internal
//! state by the targets.
use prettytable::{format, Table};
pub trait Doit {
    type Err;

    fn doit(&mut self) -> Result<(), Self::Err>;

    fn build_cmd(&mut self) -> Result<Vec<String>, Self::Err> {
        Ok(vec!["".to_string()])
    }
}

pub trait Tabulate {
    fn tabulate(&self) {
        let mut table = self.create_table();
        Self::style_table(&mut table);
        table.printstd();
    }
    /// The only method requiring implementation by the End User
    fn create_table(&self) -> Table;

    fn style_table(table: &mut Table) {
        table.set_format(*format::consts::FORMAT_BORDERS_ONLY);
    }
}
