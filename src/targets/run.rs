//! Run target handles recipes not explicitly defined within. We
//! lose the ability to typecheck arguments at the pk-make level,
//! but gain flexibility.
//!

// Internal crate imports
use crate::traits::Doit;
use crate::traits::Tabulate;
use crate::utils::exec_cmd;
use crate::Flavor;
use crate::Platform;
use crate::BuildEnv;

// External crate imports
use anyhow::anyhow;
use anyhow::Error as AnyError;
use indexmap::IndexSet as HashSet;
use prettytable::{row, Table};
use std::convert::TryInto;
use std::path::PathBuf;

/// Models the pk run target as a largely opaque vector of strings.  
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Run {
    verbose: bool,
    dry_run: bool,
    package_root: Option<PathBuf>,
    platforms: Option<HashSet<Platform>>,
    flavors: Option<HashSet<Flavor>>,
    vars: Vec<String>,
}

//
// private methods
//
impl Run {
    fn get_recipe_target_str(&self) -> &str {
        // we assume that validation has been done already
        self.vars[0].as_str()
    }
    fn get_recipe_args_str(&self) -> String {
        self.vars[1..].join(" ")
    }

    fn get_platform_str(&self, build_env: &BuildEnv) -> String {
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
            None => format!(" --platform={}", build_env.dd_os.as_str()),
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
    // retreive the package root directory
    fn get_package_root(&self) -> &std::path::Path {
        self.package_root
            .as_deref()
            .unwrap_or_else(|| std::path::Path::new(self.get_package_root()))
    }
}
impl Doit for Run {
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
        self.fix_args()?;
        let package_root = self
            .package_root
            .as_deref()
            .unwrap_or_else(|| std::path::Path::new("."));
        let build_env = BuildEnv::new(package_root)?;

        let recipe_target = self.get_recipe_target_str();
        let recipe_args_str = self.get_recipe_args_str();
        let platform_str = self.get_platform_str(&build_env);
        let flavor_str = self.get_flavor_str();

        Ok(vec![format!(
            "pk run-recipe {} {}{}{}",
            recipe_target, flavor_str, platform_str,recipe_args_str
        )])
    }
}

impl Default for Run {
    fn default() -> Self {
        Self {
            verbose: false,
            dry_run: false,
            package_root: None,
            platforms: None,
            flavors: None,
            vars: Vec::new(),
        }
    }
}

impl Run {
    pub fn verbose(&mut self, input: bool) -> &mut Self {
        self.verbose = input;
        self
    }

    pub fn dry_run(&mut self, input: bool) -> &mut Self {
        self.dry_run = input;
        self
    }

    /// Add a platform to the list of platforms on the Run struct. This may be called
    /// multiple times to accumulate platforms.
    ///
    /// # Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use pk_make::Run;
    /// let run = Run::default()
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
    /// Add a vec of platforms to the list of platforms on the Run struct. This may be called
    /// multiple times to accumulate platforms.
    ///
    /// # Example
    /// ```
    /// # fn main() -> Result<(),Box<dyn std::error::Error>> {
    /// # use pk_make::Run;
    /// let run = Run::default()
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

    /// Set a flavor in the Run struct. This method may be called multiple times,
    /// accumulating flavors.
    ///
    /// # Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use pk_make::Run;
    /// let mut run = Run::default();
    /// run.flavor(Some("^"))?.flavor(Some("maya2020"))?;
    ///
    /// // Of course we could have done this in one line:
    /// // let run = Run::default()
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

    /// Set a vec of flavor in the Run struct. This method may be called multiple times,
    /// accumulating flavors.
    ///
    /// # Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use pk_make::Run;
    /// let mut run = Run::default();
    /// run.flavors(Some(vec!["maya2020"]))?;
    ///
    /// // Of course we could have done this in one line:
    /// // let run = Run::default()
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

    pub fn vars(&mut self, input: Vec<String>) -> &mut Self {
        let mut input = input;
        self.vars.append(&mut input);
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

    pub fn build(&mut self) -> Self {
        let mut default = Self::default();
        std::mem::swap(&mut default, self);
        default
    }

    fn fix_args(&mut self) -> Result<(), AnyError> {
        if self.vars.is_empty() {
            return Err(anyhow!(
                "Run has no vars. Must supply at least one as the recipe name"
            ));
        }
        if self.vars[0].starts_with('-') {
            return Err(anyhow!(
                "first argument must be a valid target. It cannot be a flag: '{}'",
                &self.vars[0]
            ));
        }
        let mut verbose_found = false;
        let mut dry_run_found = false;
        for var in &self.vars {
            if var == "-n" || var == "--dry-run" {
                dry_run_found = true;
            }
            if var == "-v" || var == "--verbose" {
                verbose_found = true;
            }
            if verbose_found && dry_run_found {
                break;
            }
        }

        // 2 cases deal with here:
        // 1) verbose of dry_run found in args (set self.dry_run or self.verbose)
        // 2) verbose or dry_run specified in self, but not set in args (append to args)
        if self.verbose && !verbose_found {
            self.vars.push("--verbose".into());
        } else if verbose_found && !self.verbose {
            self.verbose = true;
        }

        if self.dry_run && !dry_run_found {
            self.vars.push("--dry-run".into());
        } else if dry_run_found && !self.dry_run {
            self.dry_run = true;
        }

        Ok(())
    }
}

//
// Tabulate implementation
//
impl Tabulate for Run {
    fn create_table(&self) -> Table {
        let mut table = Table::new();
        table.add_row(row!["Field", "Value"]);
        table.add_row(row!["verbose", self.verbose]);
        table.add_row(row!["dry_run", self.dry_run]);
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
        table.add_row(row!["vars", self.vars.join("\n")]);

        table
    }
}

#[cfg(test)]
#[path = "./run_test.rs"]
mod run_test;
