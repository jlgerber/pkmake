use crate::build_env::BuildEnv;
use crate::flavor::Flavor;
use crate::traits::Doit;
use anyhow::anyhow;
use anyhow::Error as AnyError;
use std::collections::HashSet;
use subprocess::Exec;
use subprocess::Redirection;

/// build target
#[derive(Debug, PartialEq, Eq)]
pub struct Build {
    pub with_docs: bool,
    pub dry_run: bool,
    pub dist_dir: Option<String>,
    pub flavors: Option<HashSet<Flavor>>,
    pub verbose: bool,
}

impl Doit for Build {
    type Err = AnyError;

    fn doit(&mut self) -> Result<(), Self::Err> {
        if self.verbose {
            println!("{:#?}", self);
        }
        let cmd = self.construct_command()?;
        if self.dry_run || self.verbose {
            println!("{}", cmd);
        }
        Ok(())
    }
    /// construct the command which will be executed
    fn construct_command(&self) -> Result<String, Self::Err> {
        let build_env = BuildEnv::new(".")?;
        // get the build dir from the build_env
        let env_dist_dir = build_env
            .dist_dir
            .to_str()
            .ok_or(anyhow!("unable to fetch dist_dir from env"))?
            .into();
        // if the use supplied the dist_dir, great. Otherwise, grab it from the env
        let dist_dir = self.dist_dir.as_ref().unwrap_or(&env_dist_dir);
        let dist_dir_str = if self.dist_dir.is_some() {
            format!(" --dist-dir={}", dist_dir)
        } else {
            "".to_string()
        };
        // wow this one is fun. we need to convert Option<T> -> Option<&T> then unwrap,
        // get a vector of Flavors, them convert them to strs, and join them into a string
        let flavor = if self.flavors.is_some() {
            self.flavors
                .as_ref()
                .unwrap()
                .iter()
                .collect::<Vec<_>>()
                .iter()
                .map(|v| v.as_str())
                .collect::<Vec<_>>()
                .join(",")
        } else {
            "".to_string()
        };
        let docs_str = if self.with_docs { " --with-docs" } else { "" };
        let flavor_str = if self.flavors.is_some() {
            format!(" --flavor={}", &flavor)
        } else {
            "".to_string()
        };
        if self.verbose {
            println!(
                "dist_dir: '{}' docs_str: '{}' flavor_str: '{}'",
                &dist_dir_str, &docs_str, &flavor_str
            );
        }
        let result = format!("pk build {}{}{}", dist_dir_str, docs_str, flavor_str);
        Ok(result)
    }
}

impl Default for Build {
    fn default() -> Self {
        Self {
            with_docs: true,
            dry_run: false,
            dist_dir: None,
            flavors: None,
            verbose: false,
        }
    }
}

impl Build {
    pub fn with_docs(&mut self, value: bool) -> &mut Self {
        self.with_docs = value;
        self
    }

    pub fn dry_run(&mut self, value: bool) -> &mut Self {
        self.dry_run = value;
        self
    }

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
    /// insert flavors.
    pub fn flavors(&mut self, input: Option<Vec<Flavor>>) -> &mut Self {
        match input {
            None => self.flavors = None,
            Some(flavors) => {
                if self.flavors.is_none() {
                    self.flavors = Some(HashSet::new())
                }
                if let Some(ref mut flavors_hs) = self.flavors {
                    flavors.into_iter().for_each(|flav| {
                        flavors_hs.insert(flav);
                        ()
                    });
                }
            }
        }
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
    pub fn can_construct_default() {
        let result = Build::default();
        let expected = Build {
            with_docs: true,
            dry_run: false,
            dist_dir: None,
            flavors: None,
            verbose: false,
        };
        assert_eq!(result, expected);
    }

    #[test]
    pub fn can_set_with_docs() {
        let mut result = Build::default();
        result.with_docs(false);
        let expected = Build {
            with_docs: false, // set by with_docs above
            dry_run: false,
            dist_dir: None,
            flavors: None,
            verbose: false,
        };
        assert_eq!(result, expected);
    }

    #[test]
    pub fn can_set_dry_run() {
        let mut result = Build::default();
        result.dry_run(true);
        let expected = Build {
            with_docs: true, // set by with_docs above
            dry_run: true,
            dist_dir: None,
            flavors: None,
            verbose: false,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn can_set_dist_dir() {
        let mut result = Build::default();
        result.dist_dir(Some("foo/bar"));
        let expected = Build {
            with_docs: true, // set by with_docs above
            dry_run: false,
            dist_dir: Some("foo/bar".to_string()),
            flavors: None,
            verbose: false,
        };
        assert_eq!(result, expected);
        // now test it with a String
        let mut result = Build::default();
        result.dist_dir(Some("foo/bar".to_string()));
        assert_eq!(result, expected);
    }

    #[test]
    fn can_set_flavors() {
        let mut result = Build::default();
        result.flavors(Some(vec![
            Flavor::Vanilla,
            Flavor::Named("foo".to_string()),
        ]));
        let mut flavs = HashSet::new();
        flavs.insert(Flavor::Vanilla);
        flavs.insert(Flavor::Named("foo".to_string()));
        let expected = Build {
            with_docs: true, // set by with_docs above
            dry_run: false,
            dist_dir: None,
            flavors: Some(flavs),
            verbose: false,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn setting_flavors_none_clears() {
        let mut result = Build::default();
        result.flavors(Some(vec![
            Flavor::Vanilla,
            Flavor::Named("foo".to_string()),
        ]));
        result.flavors(None);
        let expected = Build {
            with_docs: true, // set by with_docs above
            dry_run: false,
            dist_dir: None,
            flavors: None,
            verbose: false,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn can_set_verbose() {
        let mut result = Build::default();
        result.verbose(true);
        let expected = Build {
            with_docs: true,
            dry_run: false,
            dist_dir: None,
            flavors: None,
            verbose: true,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn can_build() {
        let result = Build::default()
            .with_docs(false)
            .dry_run(true)
            .dist_dir(Some("foo/bar"))
            .flavors(Some(vec![Flavor::Vanilla]))
            .verbose(true)
            .build();
        let mut flavs = HashSet::new();
        flavs.insert(Flavor::Vanilla);
        let expected = Build {
            with_docs: false, // set by with_docs above
            dry_run: true,
            dist_dir: Some("foo/bar".to_string()),
            flavors: Some(flavs),
            verbose: true,
        };
        assert_eq!(result, expected);
    }
}
/*
#[derive(Debug, PartialEq, Eq)]
pub struct BuildBuilder {
    pub with_docs: bool,
    pub dry_run: bool,
    pub dist_dir: Option<String>,
    pub flavors: Option<HashSet<flavor::Flavor>>,
    pub verbose: bool
}

impl Default for BuildBuilder {
    fn default() -> Self {
        Self {
            with_docs: true,
            dry_run: false,
            dist_dir: None,
            flavors: None,
            verbose: false
        }
    }
}
*/
