//! Run target handles recipes not explicitly defined within. We
//! lose the ability to typecheck arguments at the pk-make level,
//! but gain flexibility.
//!
use crate::traits::Doit;
use anyhow::anyhow;
use anyhow::Error as AnyError;

/// Models the pk run target as a largely opaque vector of strings.  
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Run {
    verbose: bool,
    dry_run: bool,
    vars: Vec<String>,
}

impl Doit for Run {
    type Err = AnyError;

    fn doit(&mut self) -> Result<(), Self::Err> {
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
        self.fix_args();
        if self.verbose {
            println!("{:#?}", self);
        }
        Ok(())
    }
}

impl Default for Run {
    fn default() -> Self {
        Self {
            verbose: false,
            dry_run: false,
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

    pub fn vars(&mut self, input: Vec<String>) -> &mut Self {
        let mut input = input;
        self.vars.append(&mut input);
        self
    }
    pub fn build(&mut self) -> Self {
        let mut default = Self::default();
        std::mem::swap(&mut default, self);
        default
    }

    fn fix_args(&mut self) {
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
    }
}

#[cfg(test)]
#[path = "./run_test.rs"]
mod run_test;
