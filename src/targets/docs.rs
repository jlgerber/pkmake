use crate::traits::Doit;
use anyhow::Error as AnyError;


#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Docs {
    pub build_dir: Option<String>,
    pub dry_run: bool,
    pub verbose: bool,
}

impl Doit for Docs {
    type Err = AnyError;

    fn doit(&mut self) -> Result<(),Self::Err> {

        Ok(())
    }
}


impl std::default::Default for Docs {
    fn default() -> Self {
        Self {
            build_dir: None,
            dry_run: false,
            verbose: false,
        }
    }
}

impl Docs {
    /// Optionally set the build directory.  
    pub fn build_dir<I>(&mut self, input: Option<I>) -> &mut Self where I: Into<String>
    {   
        match input {
            None => self.build_dir = None,
            Some(dir) => self.build_dir = Some(dir.into()),
        }
        self
    }
  
    /// Set the dry_run field.
    ///
    /// # Example 
    /// ```
    /// # fn main() {
    /// use pk_make::Docs;
    /// let docs= Docs::default().dry_run(true).build();
    /// # }
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
    /// # fn main() {
    /// use pk_make::Build;
    /// let build = Build::default().verbose(true).with_docs(false).build();
    /// # }
    /// ```
    pub fn build(&mut self) -> Self {
        let mut default = Self::default();
        std::mem::swap(self, &mut default);
        default 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn can_build_default() {
        let result = Docs::default();
        let expected = Docs {
            dry_run: false,
            build_dir: None,
            verbose: false
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn can_update_build() {
        let result = Docs::default()
                        .build_dir(Some("foo/bar"))
                        .dry_run(true)
                        .verbose(true)
                        .build();
        let expected = Docs {
            build_dir: Some("foo/bar".to_string()),
            dry_run: true,
            verbose: true
        };
        assert_eq!(result, expected);
    }
}
