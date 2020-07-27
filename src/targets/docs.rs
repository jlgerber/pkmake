use crate::traits::Doit;
use crate::traits::Tabulate;
use crate::utils::exec_cmd;
use crate::BuildEnv;
use anyhow::anyhow;
use anyhow::Error as AnyError;
use prettytable::{row, Table};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Docs {
    pub dist_dir: Option<String>,
    pub dry_run: bool,
    pub verbose: bool,
    pub defines: Option<Vec<String>>,
}

impl Doit for Docs {
    type Err = AnyError;

    fn doit(&mut self) -> Result<(), Self::Err> {
        if self.verbose {
            self.tabulate();
        }
        let cmd = self.build_cmd()?;
        if self.dry_run {
            for c in cmd {
                println!("{}", c);
            }
        } else {
            if self.verbose {
                for c in &cmd {
                    println!("{}", c);
                }
            }
            let cmd = cmd.join(" ; ");

            let exit_status = exec_cmd(cmd.as_str())?;
            println!("\nExit Status: {:?}", exit_status);
        }
        Ok(())
    }
    fn build_cmd(&mut self) -> Result<Vec<String>, Self::Err> {
        let build_env = BuildEnv::new(".")?;

        let dist_dir_str = self.dist_dir_str(&build_env)?;
        let defines_str = self.get_defines_str();

        Ok(vec![format!(
            "pk run-recipe docs {}{}",
            dist_dir_str, defines_str
        )])
    }
}

// private functions
impl Docs {
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

impl std::default::Default for Docs {
    fn default() -> Self {
        Self {
            dist_dir: None,
            dry_run: false,
            verbose: false,
            defines: None,
        }
    }
}

impl Docs {
    /// Optionally set the build directory.  
    pub fn dist_dir<I>(&mut self, input: Option<I>) -> &mut Self
    where
        I: Into<String>,
    {
        match input {
            None => self.dist_dir = None,
            Some(dir) => self.dist_dir = Some(dir.into()),
        }
        self
    }
    /// Set the dry_run field.
    ///
    /// # Example
    /// ```
    // /// # fn main() {
    /// use pk_make::Docs;
    /// let docs= Docs::default().dry_run(true).build();
    // /// # }
    /// ```
    pub fn dry_run(&mut self, input: bool) -> &mut Self {
        self.dry_run = input;
        self
    }

    /// Set verbose state
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

    /// Terminate a chain of calls with a build to return an owned instance.
    ///
    /// # Example
    /// ```
    // /// # fn main() {
    /// use pk_make::Build;
    /// let build = Build::default().verbose(true).with_docs(false).build();
    // /// # }
    /// ```
    pub fn build(&mut self) -> Self {
        let mut default = Self::default();
        std::mem::swap(self, &mut default);
        default
    }
}

//
// Tabulate implementation
//
impl Tabulate for Docs {
    fn create_table(&self) -> Table {
        let mut table = Table::new();
        table.add_row(row!["Field", "Value"]);
        table.add_row(row!["dist_dir", self.dist_dir.as_deref().unwrap_or("None")]);
        table.add_row(row!["verbose", self.verbose]);
        table.add_row(row!["dry_run", self.dry_run]);
        table.add_row(row![
            "defines",
            self.defines
                .as_ref()
                .map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n"))
                .unwrap_or(String::from("None"))
        ]);

        table
    }
}

#[cfg(test)]
#[path = "./docs_test.rs"]
mod docs_test;
