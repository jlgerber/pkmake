//! Test
//!
//! This module contains the Test struct, which models the parameters for the
//! pk test command.
//!
//! Like other targets, it provides individual methods which follow the builder pattern.
//! That is, each setter method takes `self` by mutable reference, and returns a mutable
//! reference to `self` as well.
use crate::traits::Doit;
use crate::BuildEnv;
use anyhow::anyhow;
use anyhow::Error as AnyError;

/// Models the pk test target.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Test {
    pub dist_dir: Option<String>,
    pub dry_run: bool,
    pub verbose: bool,
    pub defines: Option<Vec<String>>,
}
// private functions
impl Test {
    fn dist_dir_str(&self, build_env: &BuildEnv) -> Result<String, AnyError> {
        match &self.dist_dir {
            None => {
                let dist_dir = build_env
                    .dist_dir
                    .to_str()
                    .ok_or_else(|| anyhow!("unable to get dist dir from environment"))?;
                Ok(format!(" --dist-dir={}=", dist_dir))
            }

            Some(dist_dir) => Ok(format!(" --dist-dir={}", dist_dir)),
        }
    }

    // build up the string representing the define flag invocation.
    fn get_defines_str(&self) -> String {
        // NB: The -D flag works differently in pk build in that it
        // follows posix convention for multiple values; it supports
        // multiple invocations of the flag.
        let mut defines_str = String::new();
        if self.defines.is_some() {
            for def in self.defines.as_ref().unwrap() {
                defines_str.push_str(&format!(" -D={}", def));
            }
        }
        defines_str
    }
}

impl Doit for Test {
    type Err = AnyError;

    fn doit(&mut self) -> Result<(), Self::Err> {
        if self.verbose {
            println!("{:#?}", self);
        }
        Ok(())
    }
    fn build_cmd(&mut self) -> Result<Vec<String>, Self::Err> {
        let build_env = BuildEnv::new(".")?;

        let dist_dir_str = self.dist_dir_str(&build_env)?;
        let defines_str = self.get_defines_str();

        Ok(vec![format!(
            "pk run-recipe test {}{}",
            dist_dir_str, defines_str
        )])
    }
}

impl Default for Test {
    fn default() -> Self {
        Self {
            dist_dir: None,
            dry_run: false,
            verbose: false,
            defines: None,
        }
    }
}

impl Test {
    /// Set the dist_dir. Note that one must wrap it in an Option.
    pub fn dist_dir<I>(&mut self, input: Option<I>) -> &mut Self
    where
        I: Into<String>,
    {
        match input {
            Some(dir) => self.dist_dir = Some(dir.into()),
            None => self.dist_dir = None,
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

    /// Set the defines and return a mutable reference to self per the
    /// builder pattern.
    pub fn defines<I>(&mut self, input: Option<Vec<I>>) -> &mut Self
    where
        I: Into<String>,
    {
        let input = input.map(|vec_i| {
            vec_i
                .into_iter()
                .map(|i_val| i_val.into())
                .collect::<Vec<_>>()
        });
        self.defines = input;
        self
    }
    /// Finalize a chain of calls by returning a modified instance of the Test instance.
    ///
    /// # Example
    /// ```
    // /// # fn main() {
    /// use pk_make::Test;
    /// let test = Test::default()
    ///             .dist_dir(Some("foo/bar"))
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
