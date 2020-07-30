//! Test
//!
//! This module contains the Test struct, which models the parameters for the
//! pk test command.
//!
//! Like other targets, it provides individual methods which follow the builder pattern.
//! That is, each setter method takes `self` by mutable reference, and returns a mutable
//! reference to `self` as well.

// Internal crate imports
use crate::BuildEnv;
use crate::Flavor;
use crate::Platform;
use crate::traits::Doit;
use crate::traits::Tabulate;
use crate::utils::exec_cmd;

// external crate imports
use anyhow::anyhow;
use anyhow::Error as AnyError;
use indexmap::IndexSet as HashSet;
use prettytable::{row, Table};
use std::convert::TryInto;
use std::path::PathBuf;

/// Models the pk test target.
#[derive(Debug, PartialEq, Eq)]
pub struct Test {
    pub dist_dir: Option<String>,
    pub dry_run: bool,
    pub verbose: bool,
    pub defines: Option<Vec<String>>,
    pub platforms: Option<HashSet<Platform>>,
    pub flavors: Option<HashSet<Flavor>>,
    pub package_root: Option<PathBuf>,
}
//
// Private Methods - used to construct pk test arguements
//
impl Test {
    fn dist_dir_str(&self, _build_env: &BuildEnv) -> Result<String, AnyError> {
        match &self.dist_dir {
            None => {
                // No longer build default dist-dir from the environment. That is pk's job
                // let dist_dir = build_env
                //     .dist_dir
                //     .to_str()
                //     .ok_or_else(|| anyhow!("unable to get dist dir from environment"))?;
                // Ok(format!(" --dist-dir={}=", dist_dir))
                Ok(String::new())
            }

            Some(dist_dir) => Ok(format!(" --dist-dir={}", dist_dir)),
        }
    }

    // build up the string representing the define flag invocation.
    fn get_defines_str(&self) -> String {
        // NB: The --define flag works differently in pk build in that it
        // follows posix convention for multiple values; it supports
        // multiple invocations of the flag.
        let mut defines_str = String::new();
        if self.defines.is_some() {
            for def in self.defines.as_ref().unwrap() {
                defines_str.push_str(&format!(" --define={}", def));
            }
        }
        defines_str
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

    fn get_platform_str(&self, _build_env: &BuildEnv) -> String {
        // wow this one is fun. we need to convert Option<T> -> Option<&T> then unwrap,
        // get a vector of Flavors, them convert them to strs, and join them into a string
        match self.platforms {
            Some(ref platforms) => format!(
                " --platform={}",
                platforms
                    .iter()
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            //None => format!(" --platform={}", build_env.dd_os.as_str()),
            None => String::new()
        }
    }

    // retreive the package root directory
    fn get_package_root(&self) -> &std::path::Path {
        self.package_root
            .as_deref()
            .unwrap_or_else(|| std::path::Path::new("."))
    }
}

//
// Doit Trait Implementation
//
impl Doit for Test {
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

            let exit_status = exec_cmd(cmd.as_str(), self.get_package_root())?;
            println!("\nExit Status: {:?}", exit_status);
        }
        Ok(())
    }

    fn build_cmd(&mut self) -> Result<Vec<String>, Self::Err> {
        let build_env = BuildEnv::new(self.get_package_root())?;

        let dist_dir_str = self.dist_dir_str(&build_env)?;
        let defines_str = self.get_defines_str();
        let platform_str = self.get_platform_str(&build_env);
        let flavor_str = self.get_flavor_str();
        let dry_run_str = if self.dry_run {" --dry-run"} else {""};

        Ok(vec![format!(
            "pk run-recipe test{}{}{}{}{}",
            dist_dir_str, defines_str,platform_str,flavor_str,dry_run_str
        )])
    }
}

//
// Default Trait Implementation
//
impl Default for Test {
    fn default() -> Self {
        Self {
            dist_dir: None,
            dry_run: false,
            verbose: false,
            defines: None,
            platforms: None,
            flavors: None,
            package_root: None,
        }
    }
}

//
// Public Methods - primarily setters
//
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

    /// Update the package root, which is where we look for the manifest and vcs directories. By
    /// default, we look in the current working directory...
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

    /// Add a platform to the list of platforms on the Test struct. This may be called
    /// multiple times to accumulate platforms.
    ///
    /// # Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use pk_make::Test;
    /// let test = Test::default()
    ///                 .platform(Some("cent6"))?
    ///                 .platform(Some("cent7"))?
    ///                 .build();
    /// # Ok(())
    /// # }
    /// ```
    pub fn platform<I>(&mut self, value: Option<I>) -> Result<&mut Self, AnyError>
    where
        I: TryInto<Platform> + std::fmt::Debug + Clone,
    {
        match value {
            Some(val) => match self.platforms {
                Some(ref mut platforms) => {
                    let val_cpy = val.clone();
                    //platforms.insert(val.into());
                    match val.try_into() {
                        Ok(v) => platforms.insert(v),
                        Err(_) => {
                            return Err(anyhow!("Error converting {:?} into Platform", val_cpy))
                        }
                    };
                }
                None => {
                    let val_cpy = val.clone();
                    let mut hset = HashSet::new();
                    //hset.insert(val.into());
                    //
                    match val.try_into() {
                        Ok(v) => hset.insert(v),
                        Err(_) => {
                            return Err(anyhow!("Error converting {:?} into Platform", val_cpy))
                        }
                    };
                    self.platforms = Some(hset);
                }
            },
            None => self.platforms = None,
        }
        Ok(self)
    }
    /// Add a vec of platforms to the list of platforms on the Test struct. This may be called
    /// multiple times to accumulate platforms.
    ///
    /// # Example
    /// ```
    /// # fn main() -> Result<(),Box<dyn std::error::Error>> {
    /// # use pk_make::Test;
    /// let test = Test::default()
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

    /// Set a flavor in the Test struct. This method may be called multiple times,
    /// accumulating flavors.
    ///
    /// # Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use pk_make::Test;
    /// let mut test = Test::default();
    /// test.flavor(Some("^"))?.flavor(Some("maya2020"))?;
    ///
    /// // Of course we could have done this in one line:
    /// // let test = Test::default()
    /// //                       .flavor(Some("^"))?
    /// //                       .flavor(Some("maya2020"))?
    /// //                       .build();
    /// # Ok(())
    /// # }
    /// ```
    pub fn flavor<I>(&mut self, value: Option<I>) -> Result<&mut Self, AnyError>
    where
        I: TryInto<Flavor> + std::fmt::Debug + Clone,
    {
        match value {
            Some(val) => match self.flavors {
                Some(ref mut flavors) => {
                    let val_cpy = val.clone();
                    //platforms.insert(val.into());
                    match val.try_into() {
                        Ok(v) => flavors.insert(v),
                        Err(_) => {
                            return Err(anyhow!("Error converting {:?} into Flavor", val_cpy))
                        }
                    };
                }
                None => {
                    let val_cpy = val.clone();
                    let mut hset = HashSet::new();
                    //hset.insert(val.into());
                    //
                    match val.try_into() {
                        Ok(v) => hset.insert(v),
                        Err(_) => {
                            return Err(anyhow!("Error converting {:?} into Flavor", val_cpy))
                        }
                    };
                    self.flavors = Some(hset);
                }
            },
            None => self.flavors = None,
        }
        Ok(self)
    }

    /// Set a vec of flavor in the Test struct. This method may be called multiple times,
    /// accumulating flavors.
    ///
    /// # Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use pk_make::Test;
    /// let mut test = Test::default();
    /// test.flavors(Some(vec!["maya2020"]))?;
    ///
    /// // Of course we could have done this in one line:
    /// // let test = Test::default()
    /// //                    .flavors(Some(vec!["^"]))?
    /// //                    .build();
    /// # Ok(())
    /// # }
    /// ```
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

//
// Tabulate implementation
//
impl Tabulate for Test {
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
        table
    }
}

//
// Import Tests for Test
//
#[cfg(test)]
#[path = "./test_test.rs"]
mod test_test;
