//! Run target handles recipes not explicitly defined within. We 
//! lose the ability to typecheck arguments at the pk-make level,
//! but gain flexibility.
//!
use crate::traits::Doit;
use anyhow::Error as AnyError;
use anyhow::anyhow;

/// Models the pk run target as a largely opaque vector of strings.  
#[derive(Debug,PartialEq,Eq,Hash)]
pub struct Run {
    verbose: bool,
    dry_run: bool,
    vars: Vec<String>
}

impl Doit for Run {
    type Err = AnyError;

    fn doit(&mut self) -> Result<(),Self::Err> {
        if self.vars.len() == 0 {
            return Err(anyhow!("Run has no vars. Must supply at least one as the recipe name"))
        }
        if self.vars[0].starts_with('-') {
            return Err(anyhow!("first argument must be a valid target. It cannot be a flag: '{}'", &self.vars[0]))
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
            vars: Vec::new() 
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
mod tests {
    use super::*;
    
    #[test]
    fn can_build_default() {
        let result = Run::default();
        let expected = Run {
            dry_run: false,
            verbose: false,
            vars: Vec::new()
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn can_update_build() {
        let result = Run::default()
                        .dry_run(true)
                        .verbose(true)
                        .vars(vec!["foo".into(), "bar".into()]) 
                        .build();
        let expected = Run{
            dry_run: true,
            verbose: true,
            vars: vec!["foo".into(),"bar".into()]
        };
        assert_eq!(result, expected);
    }
    
    #[test]
    fn propagates_dry_run_and_verbose_to_vars_if_set() {
        let mut result = Run::default()
                        .dry_run(true)
                        .verbose(true)
                        .vars(vec!["foo".into(), "bar".into()]) 
                        .build();
        result.fix_args(); 
        let expected = Run{
            dry_run: true,
            verbose: true,
            vars: vec!["foo".into(),"bar".into(), "--verbose".into(), "--dry-run".into()]
        };
        assert_eq!(result, expected);
    }
 
    #[test]
    fn propagates_dry_run_and_verbose_to_struct_if_set_in_vars() {
        let mut result = Run::default()
                        .dry_run(false)
                        .verbose(false)
                        .vars(vec!["foo".into(), "bar".into(), "--verbose".into(), "--dry-run".into()]) 
                        .build();
        result.fix_args(); 
        let expected = Run{
            dry_run: true,
            verbose: true,
            vars: vec!["foo".into(),"bar".into(), "--verbose".into(), "--dry-run".into()]
        };
        assert_eq!(result, expected);
    }
 
    #[test]
    fn propagates_dry_run_and_verbose_to_struct_if_set_in_vars_as_short_flags() {
        let mut result = Run::default()
                        .dry_run(false)
                        .verbose(false)
                        .vars(vec!["foo".into(), "-n".into(),"-v".into(),"bar".into()])
                        .build();
        result.fix_args(); 
        let expected = Run{
            dry_run: true,
            verbose: true,
            vars: vec!["foo".into(),"-n".into(), "-v".into(), "bar".into()]
        };
        assert_eq!(result, expected);
    }
  
    #[test]
    fn propagates_verbose_to_struct_if_set_in_vars_as_short_flags() {
        let mut result = Run::default()
                        .dry_run(false)
                        .verbose(false)
                        .vars(vec!["foo".into(),"-v".into(),"bar".into()])
                        .build();
        result.fix_args(); 
        let expected = Run{
            dry_run: false,
            verbose: true,
            vars: vec!["foo".into(),"-v".into(), "bar".into()]
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn propagates_dry_run_to_struct_if_set_in_vars_as_short_flags() {
        let mut result = Run::default()
                        .dry_run(false)
                        .verbose(false)
                        .vars(vec!["foo".into(), "-n".into(),"bar".into()])
                        .build();
        result.fix_args(); 
        let expected = Run{
            dry_run: true,
            verbose: false,
            vars: vec!["foo".into(),"-n".into(), "bar".into()]
        };
        assert_eq!(result, expected);
    }
    
    #[test]
    fn doit_errors_if_first_vars_is_a_flag() {
        let mut result = Run::default()
            .vars(vec!["-v".into(), "bla".into()])
            .build();
        let result = result.doit();
         assert!(result.is_err());  
    }
 
    #[test]
    fn doit_errors_if_vars_is_empty() {
        let mut result = Run::default()
            .build();
        let result = result.doit();
         assert!(result.is_err());  
    }



}
