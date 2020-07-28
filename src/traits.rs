//! Traits
//!
//! The package provides a couple of traits to ensure that the target implementations
//! are handled consistently. 
//!
//! # Doit Trait
//! The Doit trait is intended to provide a means for each target to construct pk recipes
//! in a testable fashion, and to execute them consistently. 
//! 
//! The Doit trait provides two methods:
//! - **doit** - responsible for executing a target's underlying pk command(s)
//! - **build_cmd** - responsible for generating a vector of pk command strs
//!
//! # Tabulate Trait
//! The Tabulate trait is responsible for generating and styling a table. Used to report internal
//! state by the targets.
//! It provides two methods and a free function. However, all but the ```create_table``` method 
//! have default implementations that should not need to be overridden. 

// External crate imports
use prettytable::{format, Table};

/// Doit - execute pk commands in a subshell.
pub trait Doit {
    type Err;

    /// Responsible for pk command(s) execution 
    fn doit(&mut self) -> Result<(), Self::Err>;

    /// Responsible for construction of a vector of pk command strings 
    /// which are intended to be executed in a subshell.
    fn build_cmd(&mut self) -> Result<Vec<String>, Self::Err>;
}

/// Build and print a table. Used to report on the state of each target
/// struct after construction but before generation of pk command strings.
/// This is intended to be used for verbose reporting.
pub trait Tabulate {
    fn tabulate(&self) {
        let mut table = self.create_table();
        Self::style_table(&mut table);
        table.printstd();
    }
    /// The only method requiring implementation by the End User
    fn create_table(&self) -> Table;
    /// Provide the default styling, used by the ```tabulate``` function to 
    /// consistently style the tables.
    fn style_table(table: &mut Table) {
        table.set_format(*format::consts::FORMAT_BORDERS_ONLY);
    }
}
