use crate::traits::Doit;
use crate::BuildEnv;
use anyhow::anyhow;
use anyhow::Error as AnyError;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Docs {
    pub dist_dir: Option<String>,
    pub dry_run: bool,
    pub verbose: bool,
}

impl Doit for Docs {
    type Err = AnyError;

    fn doit(&mut self) -> Result<(), Self::Err> {
        if self.verbose {
            println!("{:#?}", self);
        }
        let cmd = self.build_cmd()?;
        if self.dry_run || self.verbose {
            for c in cmd {
                println!("{}", c);
            }
        }
        Ok(())
    }
    fn build_cmd(&mut self) -> Result<Vec<String>, Self::Err> {
        let build_env = BuildEnv::new(".")?;

        let dist_dir_str = self.dist_dir_str(&build_env)?;
        Ok(vec![format!("pk run-recipe docs {}", dist_dir_str)])
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
}

impl std::default::Default for Docs {
    fn default() -> Self {
        Self {
            dist_dir: None,
            dry_run: false,
            verbose: false,
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

#[cfg(test)]
#[path = "./docs_test.rs"]
mod docs_test;
