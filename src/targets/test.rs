//! Test
//!
//! This module contains the Test struct, which models the parameters for the
//! pk test command.
//!
//! Like other targets, it provides individual methods which follow the builder pattern.
//! That is, each setter method takes `self` by mutable reference, and returns a mutable
//! reference to `self` as well.
use crate::traits::Doit;
use anyhow::Error as AnyError;

/// Models the pk test target.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Test {
    pub build_dir: Option<String>,
    pub dry_run: bool,
    pub verbose: bool,
}

impl Doit for Test {
    type Err = AnyError;

    fn doit(&mut self) -> Result<(), Self::Err> {
        if self.verbose {
            println!("{:#?}", self);
        }
        Ok(())
    }
}

impl Default for Test {
    fn default() -> Self {
        Self {
            build_dir: None,
            dry_run: false,
            verbose: false,
        }
    }
}

impl Test {
    /// Set the build_dir. Note that one must wrap it in an Option.
    pub fn build_dir<I>(&mut self, input: Option<I>) -> &mut Self
    where
        I: Into<String>,
    {
        match input {
            Some(dir) => self.build_dir = Some(dir.into()),
            None => self.build_dir = None,
        }
        self
    }

    pub fn dry_run(&mut self, input: bool) -> &mut Self {
        self.dry_run = input;
        self
    }

    pub fn verbose(&mut self, input: bool) -> &mut Self {
        self.verbose = input;
        self
    }
    /// Finalize a chain of calls by returning a modified instance of the Test instance.
    ///
    /// # Example
    /// ```
    // /// # fn main() {
    /// use pk_make::Test;
    /// let test = Test::default()
    ///             .build_dir(Some("foo/bar"))
    ///             .dry_run(true)
    ///             .verbose(true)
    ///             .build();
    // /// # }
    /// ```
    pub fn build(&mut self) -> Self {
        let mut dup = Self::default();
        std::mem::swap(self, &mut dup);
        dup
    }
}

#[cfg(test)]
#[path = "./test_test.rs"]
mod test_test;
