use crate::context::Context;
use crate::flavor::Flavor;
use crate::platform::Platform;
use crate::site::Site;
use crate::traits::{Doit, Tabulate};
use crate::BuildEnv;
use crate::ManifestInfo;
use crate::OverridePair;
use prettytable::{row, Table};
//use crate::PkMakeError;
use crate::Vcs;
use anyhow::anyhow;
use anyhow::Error as AnyError;
use std::path::PathBuf;

//use std::collections::HashSet;
// IndexSet provides consistent ordering of keys based on insertion
// order
use indexmap::IndexSet as HashSet;
use std::convert::TryInto;

const DEFAULT_CONTEXT: Context = Context::User;

#[derive(Debug, PartialEq, Eq)]
/// struct implementing Install target.
pub struct Install {
    pub clean: bool,
    pub dry_run: bool,
    pub with_docs: bool,
    pub build_dir: Option<String>,
    pub context: Option<Context>,
    pub show: Option<String>,
    pub sites: Option<HashSet<Site>>,
    pub platforms: Option<HashSet<Platform>>,
    pub flavors: Option<HashSet<Flavor>>,
    pub verbose: bool,
    pub dist_dir: Option<String>,
    pub level: Option<String>,
    pub overrides: Option<Vec<OverridePair>>,
    pub defines: Option<Vec<String>>,
    pub work: bool,
    pub vcs: Option<Vcs>,
    pub logfile: Option<PathBuf>,
    pub max_jobs: Option<u8>,
}

/***************************
  Doit trait implementation
****************************/
impl Doit for Install {
    type Err = AnyError;
    /// doit executes the install target command
    fn doit(&mut self) -> Result<(), Self::Err> {
        if self.verbose {
            //println!("{:#?}", self);
            self.tabulate();
        }
        let cmd = self.build_cmd()?;
        if self.dry_run || self.verbose {
            for c in cmd {
                println!("{}", c);
            }
        }
        Ok(())
    }
    /// construct the command which will be executed
    fn build_cmd(&mut self) -> Result<Vec<String>, Self::Err> {
        let build_env = BuildEnv::new(".")?;

        self.reconcile_context_and_level(&build_env)?;
        // bail out early if we are installing to facility, as we are simply calling
        if self.get_context() == &Context::Facility {
            return match build_env.vcs {
                Some(Vcs::Git) => return Ok(vec!["git-tag create".into()]),
                Some(Vcs::Svn) => return Ok(vec!["svn-tag create".into()]),
                Some(Vcs::Both) => {
                    if let Some(ref vcs) = self.vcs {
                        match vcs {
                            Vcs::Git => Ok(vec!["git-tag create".into()]),
                            Vcs::Svn => Ok(vec!["svn-tag create".into()]),
                            _ => Err(anyhow!(
                                "Vcs system provided by user unrecognized: '{}'",
                                vcs
                            )),
                        }
                    } else {
                        Err(anyhow!(
                            "Auto detected Git and Svn. User must supply vcs explicitly"
                        ))
                    }
                }
                _ => Err(anyhow!("Unrecognized vcs")),
            };
        }

        let clean_str = if self.clean { " --clean" } else { "" };

        let defines_str = self.get_defines_str();

        // if the use supplied the dist_dir, great. Otherwise, grab it from the env
        let dist_dir_str = self.get_dist_dir_str();

        let docs_str = self.get_docs_str();

        let flavor_str = self.get_flavor_str();

        //let level_str = self.get_level_str();

        let overrides_str = self.get_overrides_str();

        let platform_str = self.get_platform_str(&build_env);

        let work_str = if self.work { " --work" } else { "" };

        let build_dir_str = self.get_build_dir_str()?;

        // we have to build an install command for every target
        let mut result = vec![format!(
            "pk audit && pk build{}{}{}{}{}{}{}{}{}",
            clean_str,
            dist_dir_str,
            docs_str,
            flavor_str,
            overrides_str,
            platform_str,
            defines_str,
            work_str,
            build_dir_str,
        )];
        self.update_results_with_install(&mut result, &build_env)?;
        Ok(result)
    }
}

//
// Private Helper Methods for Install::build_cmd(...)
//
impl Install {
    // context/show and level are both responsible for setting execution level. Context and
    // show are vestiges of the build system everyone is familiar with. Level is the pk native
    // replacement for them. This function reconciles one with the other, and errors if they
    // contradict eachother.
    fn reconcile_context_and_level(&mut self, build_env: &BuildEnv) -> Result<(), AnyError> {
        // We need to check to see that the user didnt pass in both
        // level and show and Level, since they are intended to do the same
        // thing
        let some_level = self.level.is_some();
        let some_show = self.show.is_some();
        let some_context = self.context.is_some();

        // If the user has set both the level and either the context or show (or both),
        // rather than try and apply an arbitrary rule to determine priority, an error
        // is returned.
        if some_level && (some_context || some_show) {
            return Err(anyhow!("Hey There. Level and Show/Context arguments overlap in functionality. Either use one or the other"));
        }
        // At this point, if the level has been set, we can be certain that the show and context have not been set.
        // We check to see if the level is "faciilty" and update the context if it is so. Then we return early.
        if let Some(level) = self.level.as_ref() {
            if level.to_lowercase().as_str() == "facility" {
                self.context = Some(Context::Facility);
            }
            return Ok(());
        }
        // At this point, we know that level has not been set. We need to update the level, based on the context
        // and show values, applying defaults if the user has not supplied them.

        // Extract the context, which is wrapped in an Option,
        let context = match self.context.as_ref() {
            Some(context) => context,
            None => &Context::User,
        };
        // At this point, there is nothing more to be done if the context is Facility, since
        // this fact is used elsewhere to short circuit additional work based on the level.
        // So, we can simply return early.
        if context == &Context::Facility {
            return Ok(());
        }
        // At this point, we know that level has not been set, and the context is NOT Facility. Now
        // we have to retreive the show and figure out whether we are in the user context or
        // shared context. This information will be used to set the level.

        // get the  show. If the show has not been explicitly set, we pull the show
        // from DD_SHOW via the BuildEnv
        let show = if let Some(show) = self.show.as_ref() {
            show
        } else {
            match build_env.dd_show {
                Some(ref show) => show,
                None => {
                    return Err(anyhow!(
                        "Show has not been set explicitly, and DD_SHOW is not set"
                    ))
                }
            }
        };
        // If the show is facility, again we special case it, and set the context to facility
        if show.to_lowercase().as_str() == "facility" {
            self.context = Some(Context::Facility);
            return Ok(());
        }
        // lastly, we update the level depending upon the context. We know that
        // we are at a show at this point...
        if context == &Context::Shared {
            self.level = Some(show.clone());
        } else {
            self.level = Some(format!("{}.work", show));
        }
        Ok(())
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

    // build up the pk install dist-dir flag depending on the state of
    // self.dist_dir
    fn get_dist_dir_str(&self) -> String {
        match self.dist_dir.as_ref() {
            Some(dist_dir) => format!(" --dist-dir={}", dist_dir),
            None => "".to_string(),
        }
    }

    fn get_docs_str(&self) -> &str {
        if self.with_docs {
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
            format!(" --flavor={}", &flavors)
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

    /* replaced by more terse method below
    fn get_platform_str_(&self) -> String {
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
            format!("--platform={}", &platforms)
        } else {
            "".to_string()
        };
        platform_str
    }
    */
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

    fn get_site_str(&self) -> String {
        // wow this one is fun. we need to convert Option<T> -> Option<&T> then unwrap,
        // get a vector of Flavors, them convert them to strs, and join them into a string
        match self.sites {
            Some(ref sites) => format!(
                " --site={}",
                sites
                    .iter()
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            None => " --site=local".to_string(),
        }
    }

    fn get_overrides_str(&self) -> String {
        // wow this one is fun. we need to convert Option<T> -> Option<&T> then unwrap,
        // get a vector of Flavors, them convert them to strs, and join them into a string
        match self.overrides {
            Some(ref overrides) => format!(
                " --override={}",
                overrides
                    .iter()
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            None => "".to_string(),
        }
    }

    fn get_build_dir_str(&self) -> Result<String, AnyError> {
        match self.build_dir.as_ref() {
            Some(build_dir) => Ok(format!(" --build-dir={}", build_dir)),
            None => Ok("".to_string()),
        }
    }
    fn get_logfile_str(&self) -> String {
        match self.logfile.as_ref() {
            Some(logfile) => {
                let lf: &std::path::Path = logfile.as_ref();
                format!(" --logfile={}", lf.display())
            }
            None => "".to_string(),
        }
    }
    /// get the max-jobs string
    fn get_maxjobs_str(&self) -> String {
        match self.max_jobs {
            Some(jobs) => format!(" --max-jobs={}", jobs),
            None => String::new(),
        }
    }
    // used to update the results with the installation call
    fn update_results_with_install(
        &mut self,
        result: &mut Vec<String>,
        build_env: &BuildEnv,
    ) -> Result<(), AnyError> {
        // ManifestInfo reads the manifest and retreives package information
        // the name, version, and the list of flavors
        let manifest_info = ManifestInfo::from_path(build_env.manifest.as_ref())?;
        let flavors_ref = if self.flavors.is_none() {
            manifest_info.flavors.iter().collect::<Vec<_>>()
        } else {
            self.flavors.as_ref().unwrap().iter().collect::<Vec<_>>()
        };
        let env_dist_dir = build_env
            .dist_dir
            .to_str()
            .ok_or_else(|| anyhow!("unable to fetch dist_dir from env"))?;

        let dist_dir = self
            .dist_dir
            .as_deref()
            //.as_ref()
            //.map(|s| s.as_str())
            .unwrap_or(env_dist_dir);

        let site_str = self.get_site_str();

        let platform_str = self.get_platform_str(&build_env);

        let level_str = self.get_level_str();

        let logfile_str = self.get_logfile_str();

        let maxjobs_str = self.get_maxjobs_str();

        if self.verbose {
            println!("\n Install Formatting\n");
            for item in vec![
                ("level", level_str.as_str()),
                ("site", site_str.as_str()),
                ("platform", platform_str.as_str()),
                ("dist", dist_dir),
                ("logfile", logfile_str.as_str()),
                ("max-jobs", maxjobs_str.as_str()),
            ] {
                println!("{}: {}", item.0, item.1);
            }
        }
        for flavor in flavors_ref {
            if flavor == &Flavor::Vanilla {
                result.push(format!(
                    "pk install{}{}{}{}{} {}/{}-{}",
                    level_str,
                    site_str,
                    platform_str,
                    logfile_str,
                    maxjobs_str,
                    // last 3
                    dist_dir,
                    manifest_info.name(),
                    manifest_info.version()
                ));
            } else {
                result.push(format!(
                    "pk install{}{}{}{}{} {}/{}-{}_{}",
                    level_str,
                    site_str,
                    platform_str,
                    logfile_str,
                    maxjobs_str,
                    // last 4
                    dist_dir,
                    manifest_info.name(),
                    manifest_info.version(),
                    flavor.as_str()
                ));
            }
        }
        Ok(())
    }
    /// Retrieve a reference to the context
    pub fn get_context(&self) -> &Context {
        match self.context {
            None => &DEFAULT_CONTEXT,
            Some(ref ctx) => ctx,
        }
    }
}

//
// Implement Default trait for Install.
//
impl Default for Install {
    fn default() -> Self {
        Self {
            dry_run: false,
            with_docs: true,
            build_dir: None,
            context: None,
            show: None,
            sites: None,
            platforms: None,
            flavors: None,
            verbose: false,

            clean: false,
            dist_dir: None,
            level: None,
            overrides: None,
            defines: None,
            work: false,
            vcs: None,
            logfile: None,
            max_jobs: None,
        }
    }
}

//
// Public Methods - primarily setters
//
impl Install {
    /// Set the dry_run field.
    ///
    /// # Example
    /// ```
    // /// # fn main() {
    /// # use pk_make::Install;
    /// let install = Install::default().dry_run(true).build();
    // /// # }
    /// ```
    pub fn dry_run(&mut self, input: bool) -> &mut Self {
        self.dry_run = input;
        self
    }
    /// set with_docs  field on Install struct.
    ///
    /// # Example
    /// ```
    // /// # fn main() {
    /// # use pk_make::Install;
    /// let install = Install::default().with_docs(false).build();
    // /// # }
    /// ```
    pub fn with_docs(&mut self, value: bool) -> &mut Self {
        self.with_docs = value;
        self
    }

    /// set the build directory.
    pub fn build_dir<I>(&mut self, input: Option<I>) -> &mut Self
    where
        I: Into<String>,
    {
        match input {
            None => self.build_dir = None,
            Some(dir) => self.build_dir = Some(dir.into()),
        }
        self
    }

    /// Set the context on the Install struct.
    ///
    /// # Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use pk_make::Install;
    /// let install = Install::default().context(Some("facility"))?.build();
    /// # Ok(())
    /// # }
    /// ```
    pub fn context<I>(&mut self, value: Option<I>) -> Result<&mut Self, AnyError>
    where
        I: TryInto<Context> + Clone + std::fmt::Debug,
    {
        match value {
            //Some(val) => self.context = Some(val.into()),
            Some(val) => {
                let val_cpy = val.clone();
                match val.try_into() {
                    Ok(v) => self.context = Some(v),
                    Err(_) => return Err(anyhow!("error. bad context {:?}", val_cpy)),
                }
            }
            None => self.context = None,
        }
        Ok(self)
    }
    /// Set an optional, explicit show on the Install struct.
    ///
    /// # Example
    /// ```
    // /// # fn main() {
    /// # use pk_make::Install;
    /// let install = Install::default().show(Some("DEV01")).build();
    // /// # }
    /// ```
    pub fn show<I>(&mut self, value: Option<I>) -> &mut Self
    where
        I: Into<String>,
    {
        match value {
            Some(val) => self.show = Some(val.into()),
            None => self.show = None,
        }
        self
    }

    /// Add a site to the list of sites maintianed by the Install struct. This
    /// method may be called multiple times. The method is fallible, and must
    /// be unwrapped.
    ///
    /// # Example
    /// ```
    /// # fn main() -> Result<(),Box<dyn std::error::Error>> {
    /// # use pk_make::Install;
    /// let install = Install::default()
    ///                 .site(Some("vancouver"))?
    ///                 .site(Some("playa"))?
    ///                 .build();
    /// # Ok(())
    /// # }
    /// ```
    pub fn site<I>(&mut self, value: Option<I>) -> Result<&mut Self, AnyError>
    where
        I: std::convert::TryInto<Site> + std::fmt::Debug + Clone,
    {
        match value {
            Some(val) => match self.sites {
                Some(ref mut sites) => {
                    let val_cpy = val.clone();
                    match val.try_into() {
                        Ok(v) => sites.insert(v),
                        Err(_) => return Err(anyhow!("Error converting {:?} into Site", val_cpy)),
                    };
                }
                None => {
                    let mut hset = HashSet::new();
                    let val_cpy = val.clone();
                    match val.try_into() {
                        Ok(v) => hset.insert(v),
                        Err(_) => return Err(anyhow!("Error converting {:?} into Site", val_cpy)),
                    };
                    //hset.insert(val.into());
                    self.sites = Some(hset);
                }
            },
            None => self.sites = None,
        }
        Ok(self)
    }

    /// Add a vec of sites to the list of sites maintianed by the Install struct. This is a
    /// fallible setter, and must be unwrapped or '?'ed
    ///
    /// # Example
    /// ```
    /// # fn main() -> Result<(),Box<dyn std::error::Error>> {
    /// # use pk_make::Install;
    /// let install = Install::default()
    ///                 .sites(Some(vec!["vancouver","portland"]))?
    ///                 .build();
    /// # Ok(())
    /// # }
    /// ```
    pub fn sites<I>(&mut self, value: Option<Vec<I>>) -> Result<&mut Self, AnyError>
    where
        I: TryInto<Site>,
    {
        match value {
            None => self.sites = None,
            Some(sites) => {
                let sites: Result<Vec<_>, _> =
                    sites.into_iter().map(|i_val| i_val.try_into()).collect();
                match sites {
                    Err(_) => return Err(anyhow!("failed to convert one or more overrides")),
                    Ok(val) => match self.sites {
                        Some(ref mut sites) => {
                            for v in val {
                                sites.insert(v);
                            }
                        }
                        None => {
                            let mut hset = HashSet::new();
                            for v in val {
                                hset.insert(v);
                            }
                            self.sites = Some(hset);
                        }
                    },
                }
            }
        }
        Ok(self)
    }

    /// Add a platform to the list of platforms on the Install struct. This may be called
    /// multiple times to accumulate platforms.
    ///
    /// # Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use pk_make::Install;
    /// let install = Install::default()
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
    /// Add a vec of platforms to the list of platforms on the Install struct. This may be called
    /// multiple times to accumulate platforms.
    ///
    /// # Example
    /// ```
    /// # fn main() -> Result<(),Box<dyn std::error::Error>> {
    /// # use pk_make::Install;
    /// let install = Install::default()
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

    /// Set a flavor in the Install struct. This method may be called multiple times,
    /// accumulating flavors.
    ///
    /// # Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use pk_make::Install;
    /// let mut install = Install::default();
    /// install.flavor(Some("^"))?.flavor(Some("maya2020"))?;
    ///
    /// // Of course we could have done this in one line:
    /// // let install = Install::default()
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

    /// Set a vec of flavor in the Install struct. This method may be called multiple times,
    /// accumulating flavors.
    ///
    /// # Example
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # use pk_make::Install;
    /// let mut install = Install::default();
    /// install.flavors(Some(vec!["maya2020"]))?;
    ///
    /// // Of course we could have done this in one line:
    /// // let install = Install::default()
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

    /// Set the verbose field in the Install struct
    pub fn verbose(&mut self, input: bool) -> &mut Self {
        self.verbose = input;
        self
    }

    /// Set the clean value and return a mutable reference to self per the builder pattern.
    pub fn clean(&mut self, value: bool) -> &mut Self {
        self.clean = value;
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
    /// Set the overrides value and return a mutable reference to self. This method
    /// is fallible, and must be unwrapped to get a &mut Self
    ///
    /// # Example
    /// ```rust
    /// # fn main() -> Result<(),Box<dyn std::error::Error>> {
    /// # use pk_make::Install;
    /// let install = Install::default()
    ///                 .overrides(Some(vec!["makebridge=3.0.0"]))?
    ///                 .build();
    /// # Ok(())
    /// # }
    pub fn overrides<I>(&mut self, value: Option<Vec<I>>) -> Result<&mut Self, AnyError>
    where
        I: std::convert::TryInto<OverridePair>,
    {
        match value {
            None => {
                self.overrides = None;
                Ok(self)
            }
            Some(v) => {
                let vals: Result<Vec<_>, _> = v.into_iter().map(|i_val| i_val.try_into()).collect();
                match vals {
                    Err(_) => Err(anyhow!("failed to convert one or more overrides")),
                    Ok(val) => {
                        self.overrides = Some(val);
                        Ok(self)
                    }
                }
            }
        }
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
    pub fn work(&mut self, input: bool) -> &mut Self {
        self.work = input;
        self
    }
    // pub fn vcs(&mut self, input: Option<Vcs>) -> &mut Self {
    //     self.vcs = input;
    //     self
    // }
    /// Set the input given an option wrapped type which can be converted Into Vcs
    pub fn vcs<I>(&mut self, input: Option<I>) -> &mut Self
    where
        I: Into<Vcs>,
    {
        match input {
            None => self.vcs = None,
            Some(vcs) => self.vcs = Some(vcs.into()),
        }
        self
    }

    /// Set the input given an option wrapped type which can be converted Into logfile
    pub fn logfile<I>(&mut self, input: Option<I>) -> &mut Self
    where
        I: Into<PathBuf>,
    {
        match input {
            None => self.logfile = None,
            Some(logf) => self.logfile = Some(logf.into()),
        }
        self
    }
    /// Specify the optional max jobs to be used when installing
    pub fn max_jobs(&mut self, input: Option<u8>) -> &mut Self {
        self.max_jobs = input;
        self
    }
    /// Construct a new instance of Install from a mutable reference. Used to finalize
    /// a number of chained calls adhering to the builder pattern.
    pub fn build(&mut self) -> Self {
        let mut default = Self::default();
        std::mem::swap(self, &mut default);
        default
    }
}

//
// Tabulate implementation
//
impl Tabulate for Install {
    fn create_table(&self) -> Table {
        let mut table = Table::new();
        table.add_row(row!["Field", "Value"]);
        table.add_row(row!["dry_run", self.dry_run]);
        table.add_row(row!["with_docs", self.with_docs]);
        table.add_row(row!["verbose", self.verbose]);
        table.add_row(row!["dist_dir", self.dist_dir.as_deref().unwrap_or("None")]);
        table.add_row(row!["level", self.level.as_deref().unwrap_or("None")]);
        table.add_row(row!["work", self.work]);
        table.add_row(row![
            "Vcs",
            self.vcs.as_ref().map(|vcs| vcs.as_str()).unwrap_or("None")
        ]);
        table.add_row(row![
            "logfile",
            self.logfile
                .as_ref()
                .map(|l| l.to_str().unwrap_or("NON_UTF8_STR_USED"))
                .unwrap_or("None")
        ]);
        table.add_row(row![
            "max_jobs",
            self.max_jobs
                .map(|v| v.to_string())
                .unwrap_or(String::from("None"))
        ]);
        table.add_row(row![
            "overrides",
            self.overrides
                .as_ref()
                .map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n"))
                .unwrap_or(String::from("None"))
        ]);
        table.add_row(row![
            "defines",
            self.defines
                .as_ref()
                .map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n"))
                .unwrap_or(String::from("None"))
        ]);
        table.add_row(row![
            "build_dir",
            self.build_dir.as_deref().unwrap_or("None")
        ]);
        table.add_row(row![
            "context",
            self.context.as_ref().map(|v| v.as_str()).unwrap_or("None")
        ]);
        table.add_row(row!["show", self.show.as_deref().unwrap_or("None")]);
        table.add_row(row![
            "sites",
            self.sites
                .as_ref()
                .map(|v| v.iter().map(|x| x.as_str()).collect::<Vec<_>>().join("\n"))
                .unwrap_or_else(|| "None".to_string())
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
// thanks to Karol Kuczmarski
// http://xion.io/post/code/rust-unit-test-placement.html
#[cfg(test)]
#[path = "./install_test.rs"]
mod install_test;
