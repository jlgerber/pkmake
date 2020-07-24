use crate::build_env::BuildEnv;
use crate::flavor::Flavor;
use crate::platform::Platform;
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
    pub level: Option<String>,
    pub metadata_only: bool,
    pub platforms: Option<HashSet<Platform>>,
    pub verbose: bool,
    pub defines: Option<Vec<String>>,
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

        let defines_str = self.get_defines_str();

        // if the use supplied the dist_dir, great. Otherwise, grab it from the env
        let dist_dir_str = self.get_dist_dir_str(&build_env)?;

        let docs_str = self.get_docs_str();

        let flavor_str = self.get_flavor_str();

        let level_str = self.get_level_str();

        let metadata_only_str = if self.with_docs {
            " --metadata-only"
        } else {
            ""
        };

        let platform_str = self.get_platform_str();

        if self.verbose {
            println!(
                "dist_dir: '{}' docs_str: '{}' flavor_str: '{}' defines_str: '{}'\n",
                &dist_dir_str, &docs_str, &flavor_str, &defines_str
            );
        }
        let result = format!(
            "pk audit && pk build {}{}{}{}{}{}{}",
            dist_dir_str,
            docs_str,
            flavor_str,
            level_str,
            metadata_only_str,
            platform_str,
            defines_str
        );
        Ok(result)
    }
}

// implementation of private convenience methods
impl Build {
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

    fn get_dist_dir_str(&self, build_env: &BuildEnv) -> Result<String, AnyError> {
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
        Ok(dist_dir_str)
    }
    fn get_docs_str(&self) -> &str {
        if self.with_docs && !self.metadata_only {
            " --with-docs"
        } else {
            ""
        }
    }
    fn get_flavor_str(&self) -> String {
        // wow this one is fun. we need to convert Option<T> -> Option<&T> then unwrap,
        // get a vector of Flavors, them convert them to strs, and join them into a string
        let flavors = if self.flavors.is_some() {
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
        let flavor_str = if self.flavors.is_some() {
            format!(" --flavor={}", &flavors)
        } else {
            "".to_string()
        };
        flavor_str
    }

    fn get_level_str(&self) -> String {
        match self.level.as_ref() {
            Some(level) => format!(" --level={}", level),
            None => "".to_string(),
        }
    }

    fn get_platform_str(&self) -> String {
        // wow this one is fun. we need to convert Option<T> -> Option<&T> then unwrap,
        // get a vector of Flavors, them convert them to strs, and join them into a string
        let platforms = if self.platforms.is_some() {
            self.platforms
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
        let platform_str = if self.platforms.is_some() {
            format!(" --platform={}", &platforms)
        } else {
            "".to_string()
        };
        platform_str
    }
}

impl Default for Build {
    fn default() -> Self {
        Self {
            with_docs: true,
            dry_run: false,
            dist_dir: None,
            flavors: None,
            level: None,
            metadata_only: false,
            platforms: None,
            verbose: false,
            defines: None,
        }
    }
}

// Implementation of the Setter methods
impl Build {
    /// Set the with_docs value and return a mutable reference to self per the
    /// builder pattern.
    pub fn with_docs(&mut self, value: bool) -> &mut Self {
        self.with_docs = value;
        self
    }

    /// Set the dry_run value and return a mutable reference to self
    /// as per the builder pattern.
    pub fn dry_run(&mut self, value: bool) -> &mut Self {
        self.dry_run = value;
        self
    }

    /// Set the dist_dir value and return a mutable reference to
    /// self, per the builder pattern.
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
    /// Set flavors per the builder pattern
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

    /// Set the level value and return a mutable reference to
    /// self, per the builder pattern.
    pub fn level<I>(&mut self, input: Option<I>) -> &mut Self
    where
        I: Into<String>,
    {
        match input {
            None => self.level = None,
            Some(level) => self.level = Some(level.into()),
        }
        self
    }

    /// Set the metadata_only value and return a mutable reference to self
    /// as per the builder pattern.
    pub fn metadata_only(&mut self, value: bool) -> &mut Self {
        self.metadata_only = value;
        self
    }

    /// Set platforms per the builder pattern
    pub fn platforms(&mut self, input: Option<Vec<Platform>>) -> &mut Self {
        match input {
            None => self.platforms = None,
            Some(platforms) => {
                if self.platforms.is_none() {
                    self.platforms = Some(HashSet::new())
                }
                if let Some(ref mut platforms_hs) = self.platforms {
                    platforms.into_iter().for_each(|flav| {
                        platforms_hs.insert(flav);
                        ()
                    });
                }
            }
        }
        self
    }
    /// Set verbose state and return a mutable reference to self
    /// per the builder pattern.
    pub fn verbose(&mut self, input: bool) -> &mut Self {
        self.verbose = input;
        self
    }
    /// Set the defines and return a mutable reference to self per the
    /// builder pattern.
    pub fn defines(&mut self, input: Option<Vec<String>>) -> &mut Self {
        self.defines = input;
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
            level: None,
            metadata_only: false,
            platforms: None,
            verbose: false,
            defines: None,
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
            level: None,
            metadata_only: false,
            platforms: None,
            verbose: false,
            defines: None,
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
            level: None,
            metadata_only: false,
            platforms: None,
            verbose: false,
            defines: None,
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
            level: None,
            metadata_only: false,
            platforms: None,
            verbose: false,
            defines: None,
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
            level: None,
            metadata_only: false,
            platforms: None,
            verbose: false,
            defines: None,
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
            level: None,
            metadata_only: false,
            platforms: None,
            verbose: false,
            defines: None,
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
            level: None,
            metadata_only: false,
            platforms: None,
            verbose: true,
            defines: None,
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
            level: None,
            metadata_only: false,
            platforms: None,
            verbose: true,
            defines: None,
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
