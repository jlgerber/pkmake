use crate::build_env::BuildEnv;
use crate::flavor::Flavor;
use crate::platform::Platform;
use crate::traits::{Doit, Tabulate};
use crate::OverridePair;
use anyhow::anyhow;
use anyhow::Error as AnyError;
//use std::collections::HashSet;
// IndexSet provides consistent ordering of keys based on insertion
// order
use indexmap::IndexSet as HashSet;
//use subprocess::Exec;
//use subprocess::Redirection;
use crate::utils::exec_cmd;
//use crate::utils::exec_in_shell;
use prettytable::{row, Table};
use std::convert::TryInto;
/// build target
#[derive(Debug, PartialEq, Eq)]
pub struct Build {
    pub clean: bool,
    pub with_docs: bool,
    pub dry_run: bool,
    pub dist_dir: Option<String>,
    pub flavors: Option<HashSet<Flavor>>,
    pub level: Option<String>,
    pub metadata_only: bool,
    pub overrides: Option<Vec<OverridePair>>,
    pub platforms: Option<HashSet<Platform>>,
    pub verbose: bool,
    pub defines: Option<Vec<String>>,
    pub work: bool,
    pub package_root: Option<std::path::PathBuf>,
}

impl Doit for Build {
    type Err = AnyError;

    fn doit(&mut self) -> Result<(), Self::Err> {
        if self.verbose {
            self.tabulate();
        }
        let cmd = self.build_cmd()?;
        if self.dry_run {
            for c in &cmd {
                println!("{}", c);
            }
        } else {
            if self.verbose {
                for c in &cmd {
                    println!("{}", c);
                }
            }
            let cmd = cmd.join(" ; ");

            let exit_status = exec_cmd(cmd.as_str(), self.get_package_root())?;
            println!("\nExit Status: {:?}", exit_status);
        }
        Ok(())
    }

    /// construct the command which will be executed
    fn build_cmd(&mut self) -> Result<Vec<String>, Self::Err> {
        let package_root = self
            .package_root
            .as_deref()
            .unwrap_or_else(|| std::path::Path::new("."));

        let build_env = BuildEnv::new(package_root)?;

        let clean_str = if self.clean { " --clean" } else { "" };

        let defines_str = self.get_defines_str();

        // if the use supplied the dist_dir, great. Otherwise, grab it from the env
        let dist_dir_str = self.get_dist_dir_str(&build_env)?;

        let docs_str = self.get_docs_str();

        let flavor_str = self.get_flavor_str();

        let level_str = self.get_level_str();

        let metadata_only_str = if self.metadata_only {
            " --metadata-only"
        } else {
            ""
        };

        let overrides_str = self.get_overrides_str();

        let platform_str = self.get_platform_str();

        let work_str = if self.work { " --work" } else { "" };

        let result = vec![format!(
            "pk audit && pk build{}{}{}{}{}{}{}{}{}{}",
            clean_str,
            dist_dir_str,
            docs_str,
            flavor_str,
            level_str,
            metadata_only_str,
            overrides_str,
            platform_str,
            defines_str,
            work_str
        )];
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
            .ok_or_else(|| anyhow!("unable to fetch dist_dir from env"))?
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
        if self.flavors.is_some() {
            format!(" --flavour={}", &flavors)
        } else {
            "".to_string()
        }
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
        if self.platforms.is_some() {
            format!(" --platform={}", &platforms)
        } else {
            "".to_string()
        }
    }

    fn get_overrides_str(&self) -> String {
        // wow this one is fun. we need to convert Option<T> -> Option<&T> then unwrap,
        // get a vector of Flavors, them convert them to strs, and join them into a string
        let overrides = if self.overrides.is_some() {
            self.overrides
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
        if self.overrides.is_some() {
            format!(" --override={}", &overrides)
        } else {
            "".to_string()
        }
    }

    // retreive the package root directory
    fn get_package_root(&self) -> &std::path::Path {
        self.package_root
            .as_deref()
            .unwrap_or_else(|| std::path::Path::new("."))
    }
}

impl Default for Build {
    fn default() -> Self {
        Self {
            clean: false,
            with_docs: true,
            dry_run: false,
            dist_dir: None,
            flavors: None,
            level: None,
            metadata_only: false,
            overrides: None,
            platforms: None,
            verbose: false,
            defines: None,
            work: false,
            package_root: None,
        }
    }
}

// Implementation of the Setter methods
impl Build {
    /// Set the clean value and return a mutable reference to self per the builder pattern.
    pub fn clean(&mut self, value: bool) -> &mut Self {
        self.clean = value;
        self
    }
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
   
    pub fn flavors<I>(&mut self, value: Option<Vec<I>>) -> Result<&mut Self, AnyError>
    where
        I: TryInto<Flavor> + std::fmt::Debug + Clone,
    {
        match value {
            None => self.flavors = None,
            Some(flavors) => {
                let flavors: Result<Vec<_>, _> =
                    flavors.into_iter().map(|i_val| i_val.try_into()).collect();
                match flavors {
                    Err(_) => return Err(anyhow!("failed to convert one or more flavors")),
                    Ok(val) => match self.flavors {
                        Some(ref mut flavors) => {
                            for v in val {
                                flavors.insert(v);
                            }
                        }
                        None => {
                            let mut hset = HashSet::new();
                            for v in val {
                                hset.insert(v);
                            }
                            self.flavors = Some(hset);
                        }
                    },
                }
            }
        }
        Ok(self)
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
    /// Set the overrides value and return a mutable reference to self
    pub fn overrides(&mut self, value: Option<Vec<OverridePair>>) -> &mut Self {
        self.overrides = value;
        self
    }
    
    pub fn package_root<I>(&mut self, input: Option<I>) -> &mut Self
    where
        I: Into<std::path::PathBuf>,
    {
        match input {
            None => self.package_root = None,
            Some(proot) => self.package_root = Some(proot.into()),
        }
        self
    }
    /// Add a vec of platforms to the list of platforms on the BUild struct. This may be called
    /// multiple times to accumulate platforms.
    ///
    /// # Example
    /// ```
    /// # fn main() -> Result<(),Box<dyn std::error::Error>> {
    /// # use pk_make::Build;
    /// let build = Build::default()
    ///                 .platforms(Some(vec!["cent7", "cent6"]))?
    ///                 .build();
    /// # Ok(())
    /// # }
    /// ```
    pub fn platforms<I>(&mut self, value: Option<Vec<I>>) -> Result<&mut Self, AnyError>
    where
        I: TryInto<Platform>,
    {
        match value {
            None => self.platforms = None,
            Some(plats) => {
                let plats: Result<Vec<_>, _> =
                    plats.into_iter().map(|i_val| i_val.try_into()).collect();
                match plats {
                    Err(_) => return Err(anyhow!("failed to convert to platform")),
                    Ok(val) => match self.platforms {
                        Some(ref mut platforms) => {
                            for v in val {
                                platforms.insert(v);
                            }
                        }
                        None => {
                            let mut hset = HashSet::new();
                            for v in val {
                                hset.insert(v);
                            }
                            self.platforms = Some(hset);
                        }
                    },
                }
            }
        }
        Ok(self)
    }

    /// Set verbose state and return a mutable reference to self
    /// per the builder pattern.
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
        let input = input.map(|v| v.into_iter().map(|s| s.into()).collect::<Vec<_>>());
        self.defines = input;
        self
    }
    pub fn work(&mut self, input: bool) -> &mut Self {
        self.work = input;
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
impl Tabulate for Build {
    fn create_table(&self) -> Table {
        let mut table = Table::new();
        table.add_row(row!["Field", "Value"]);
        table.add_row(row!["clean", self.clean]);
        table.add_row(row!["with_docs", self.with_docs]);
        table.add_row(row!["dry_run", self.dry_run]);
        table.add_row(row!["dist_dir", self.dist_dir.as_deref().unwrap_or("None")]);
        table.add_row(row![
            "flavors",
            self.flavors
                .as_ref()
                .map(|v| v.iter().map(|x| x.as_str()).collect::<Vec<_>>().join("\n"))
                .unwrap_or_else(|| "None".to_string())
        ]);
        table.add_row(row!["level", self.level.as_deref().unwrap_or("None")]);
        table.add_row(row!["metadata_only", self.metadata_only]);
        table.add_row(row![
            "overrides",
            self.overrides
                .as_ref()
                .map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n"))
                .unwrap_or(String::from("None"))
        ]);
        table.add_row(row![
            "platforms",
            self.platforms
                .as_ref()
                .map(|v| v.iter().map(|x| x.as_str()).collect::<Vec<_>>().join("\n"))
                .unwrap_or_else(|| "None".to_string())
        ]);
        table.add_row(row!["verbose", self.verbose]);
        table.add_row(row![
            "defines",
            self.defines
                .as_ref()
                .map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n"))
                .unwrap_or(String::from("None"))
        ]);
        table.add_row(row!["work", self.work]);
        table.add_row(row![
            "platforms",
            self.platforms
                .as_ref()
                .map(|v| v.iter().map(|x| x.as_str()).collect::<Vec<_>>().join("\n"))
                .unwrap_or_else(|| "None".to_string())
        ]);
        table.add_row(row![
            "flavors",
            self.flavors
                .as_ref()
                .map(|v| v.iter().map(|x| x.as_str()).collect::<Vec<_>>().join("\n"))
                .unwrap_or_else(|| "None".to_string())
        ]);
        table.add_row(row![
            "package_root",
            self.package_root
                .as_deref()
                .unwrap_or_else(|| std::path::Path::new("."))
                .to_str()
                .unwrap()
        ]);

        table
    }
}

#[cfg(test)]
#[path = "./build_test.rs"]
mod build_test;
