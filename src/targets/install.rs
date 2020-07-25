use crate::context::Context;
use crate::flavor::Flavor;
use crate::platform::Platform;
use crate::site::Site;
use crate::traits::Doit;
use crate::BuildEnv;
use crate::ManifestInfo;
use crate::OverridePair;
use crate::Vcs;
use anyhow::anyhow;
use anyhow::Error as AnyError;
use std::collections::HashSet;

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
}

/***************************
  Doit trait implementation
****************************/
impl Doit for Install {
    type Err = AnyError;
    /// doit executes the install target command
    fn doit(&mut self) -> Result<(), Self::Err> {
        if self.verbose {
            println!("{:#?}", self);
        }
        let cmd = self.construct_command()?;
        if self.dry_run || self.verbose {
            for c in cmd {
                println!("{}", c);
            }
        }
        Ok(())
    }
    /// construct the command which will be executed
    fn construct_command(&mut self) -> Result<Vec<String>, Self::Err> {
        let build_env = BuildEnv::new(".")?;

        self.reconcile_context_and_level(&build_env)?;
        // bail out early if we are installing to facility, as we are simply calling
        if self.get_context() == &Context::Facility {
            return match build_env.vcs {
                Vcs::Git => return Ok(vec!["git-tag create".into()]),
                Vcs::Svn => return Ok(vec!["svn-tag create".into()]),
                Vcs::Both => {
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

        let clean_str = if self.clean { "--clean" } else { "" };

        let defines_str = self.get_defines_str();

        // if the use supplied the dist_dir, great. Otherwise, grab it from the env
        let dist_dir_str = self.get_dist_dir_str();

        let docs_str = self.get_docs_str();

        let flavor_str = self.get_flavor_str();

        let level_str = self.get_level_str();

        let overrides_str = self.get_overrides_str();

        let platform_str = self.get_platform_str();

        let work_str = if self.work { "--work" } else { "" };
        let build_dir_str = self.get_build_dir_str(&build_env)?;

        // we have to build an install command for every target
        let mut result = vec![format!(
            "pk audit && pk build {} {} {} {} {} {} {} {} {}",
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
// Helper Methods for Install::construct_command(...)
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
        // There is nothing more that needs to be done, so we exit.
        match self.level.as_ref() {
            Some(level) => {
                if level.to_lowercase().as_str() == "facility" {
                    self.context = Some(Context::Facility);
                }
                return Ok(());
            }
            None => (),
        }
        // if some_level {
        //     return Ok(());
        // }
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
        // TODO: I believe that we still have to set level to either the showname or <show>.work depending upon
        // the context. DONE
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
            Some(dist_dir) => format!("--dist-dir={}", dist_dir),
            None => "".to_string(),
        }
    }

    fn get_docs_str(&self) -> &str {
        if self.with_docs {
            "--with-docs"
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
            format!("--flavor={}", &flavors)
        } else {
            "".to_string()
        };
        flavor_str
    }

    fn get_level_str(&self) -> String {
        match self.level.as_ref() {
            Some(level) => format!("--level={}", level),
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
    fn get_platform_str(&self) -> String {
        // wow this one is fun. we need to convert Option<T> -> Option<&T> then unwrap,
        // get a vector of Flavors, them convert them to strs, and join them into a string
        match self.platforms {
            Some(ref platforms) => format!(
                "--platform={}",
                platforms
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

    fn get_site_str(&self) -> String {
        // wow this one is fun. we need to convert Option<T> -> Option<&T> then unwrap,
        // get a vector of Flavors, them convert them to strs, and join them into a string
        match self.sites {
            Some(ref sites) => format!(
                "--site={}",
                sites
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

    fn get_overrides_str(&self) -> String {
        // wow this one is fun. we need to convert Option<T> -> Option<&T> then unwrap,
        // get a vector of Flavors, them convert them to strs, and join them into a string
        match self.overrides {
            Some(ref overrides) => format!(
                "--override={}",
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

    fn get_build_dir_str(&self, build_env: &BuildEnv) -> Result<String, AnyError> {
        // dont see the use in the env_build_dir here.
        // TODO: remove build_env from arg list as it appears to be irrelevant
        /*
        let env_build_dir = build_env
            .build_dir
            .to_str()
            .ok_or(anyhow!("unable to fetch build_dir from env"))?
            .into();

        // if the use supplied the build_dir, great. Otherwise, grab it from the env
        let build_dir = self.build_dir.as_ref().unwrap_or(&env_build_dir);
        let build_dir_str = if self.build_dir.is_some() {
            format!("--build-dir={}", build_dir)
        } else {
            "".to_string()
        };
        //Ok(build_dir_str)
        */
        match self.build_dir.as_ref() {
            Some(build_dir) => Ok(format!("--build-dir={}", build_dir)),
            None => Ok("".to_string()),
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
            .ok_or(anyhow!("unable to fetch dist_dir from env"))?;

        let dist_dir = self
            .dist_dir
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or(env_dist_dir);

        let site_str = self.get_site_str();

        let platform_str = self.get_platform_str();

        let level_str = self.get_level_str();

        for flavor in flavors_ref {
            if flavor == &Flavor::Vanilla {
                result.push(format!(
                    "pk install {} {} {} {}/{}-{}",
                    level_str,
                    site_str,
                    platform_str,
                    dist_dir,
                    manifest_info.name(),
                    manifest_info.version()
                ));
            } else {
                result.push(format!(
                    "pk install {} {} {} {}/{}-{}_{}",
                    level_str,
                    site_str,
                    platform_str,
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
        }
    }
}

//
// Public Methods
//
impl Install {
    /// Set the dry_run field.
    ///
    /// # Example
    /// ```
    /// # fn main() {
    /// # use pk_make::Install;
    /// let install = Install::default().dry_run(true).build();
    /// # }
    /// ```
    pub fn dry_run(&mut self, input: bool) -> &mut Self {
        self.dry_run = input;
        self
    }
    /// set with_docs  field on Install struct.
    ///
    /// # Example
    /// ```
    /// # fn main() {
    /// # use pk_make::Install;
    /// let install = Install::default().with_docs(false).build();
    /// # }
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
    /// # fn main() {
    /// # use pk_make::Install;
    /// let install = Install::default().context(Some("facility")).build();
    /// # }
    /// ```
    pub fn context<I>(&mut self, value: Option<I>) -> &mut Self
    where
        I: Into<Context>,
    {
        match value {
            Some(val) => self.context = Some(val.into()),
            None => self.context = None,
        }
        self
    }

    /// Set an optional, explicit show on the Install struct.
    ///
    /// # Example
    /// ```
    /// # fn main() {
    /// # use pk_make::Install;
    /// let install = Install::default().show(Some("DEV01")).build();
    /// # }
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
    /// method may be called multiple times.
    ///
    /// # Example
    /// ```
    /// # fn main() {
    /// # use pk_make::Install;
    /// let install = Install::default()
    ///                 .site(Some("vancouver"))
    ///                 .site(Some("playa"))
    ///                 .build();
    /// # }
    /// ```
    pub fn site<I>(&mut self, value: Option<I>) -> &mut Self
    where
        I: Into<Site>,
    {
        match value {
            Some(val) => match self.sites {
                Some(ref mut sites) => {
                    sites.insert(val.into());
                }
                None => {
                    let mut hset = HashSet::new();
                    hset.insert(val.into());
                    self.sites = Some(hset);
                }
            },
            None => self.sites = None,
        }
        self
    }

    /// Add a vec of sites to the list of sites maintianed by the Install struct. This
    /// method may be called multiple times.
    ///
    /// # Example
    /// ```
    /// # fn main() {
    /// # use pk_make::Install;
    /// let install = Install::default()
    ///                 .sites(Some(vec!["vancouver","portland"]))
    ///                 .build();
    /// # }
    /// ```
    pub fn sites<I>(&mut self, value: Option<Vec<I>>) -> &mut Self
    where
        I: Into<Site>,
    {
        match value {
            Some(vals) => match self.sites {
                Some(ref mut sites) => {
                    for val in vals {
                        sites.insert(val.into());
                    }
                }
                None => {
                    let mut hset = HashSet::new();
                    for val in vals {
                        hset.insert(val.into());
                    }
                    self.sites = Some(hset);
                }
            },
            None => self.sites = None,
        }
        self
    }

    /// Add a platform to the list of platforms on the Install struct. This may be called
    /// multiple times to accumulate platforms.
    ///
    /// # Example
    /// ```
    /// # fn main() {
    /// # use pk_make::Install;
    /// let install = Install::default()
    ///                 .platform(Some("cent6"))
    ///                 .platform(Some("cent7"))
    ///                 .build();
    /// # }
    /// ```
    pub fn platform<I>(&mut self, value: Option<I>) -> &mut Self
    where
        I: Into<Platform>,
    {
        match value {
            Some(val) => match self.platforms {
                Some(ref mut platforms) => {
                    platforms.insert(val.into());
                }
                None => {
                    let mut hset = HashSet::new();
                    hset.insert(val.into());
                    self.platforms = Some(hset);
                }
            },
            None => self.platforms = None,
        }
        self
    }
    /// Add a vec of platforms to the list of platforms on the Install struct. This may be called
    /// multiple times to accumulate platforms.
    ///
    /// # Example
    /// ```
    /// # fn main() {
    /// # use pk_make::Install;
    /// let install = Install::default()
    ///                 .platforms(Some(vec!["cent7", "cent6"]))
    ///                 .build();
    /// # }
    /// ```
    pub fn platforms<I>(&mut self, value: Option<Vec<I>>) -> &mut Self
    where
        I: Into<Platform>,
    {
        match value {
            Some(vals) => match self.platforms {
                Some(ref mut platforms) => {
                    for val in vals {
                        platforms.insert(val.into());
                    }
                }
                None => {
                    let mut hset = HashSet::new();
                    for val in vals {
                        hset.insert(val.into());
                    }
                    self.platforms = Some(hset);
                }
            },
            None => self.platforms = None,
        }
        self
    }

    /// Set a flavor in the Install struct. This method may be called multiple times,
    /// accumulating flavors.
    ///
    /// # Example
    /// ```
    /// # fn main() {
    /// let mut install = Install::default();
    /// # use pk_make::Install;
    /// install.flavor(Some("^")).flavor(Some("maya2020"));
    ///
    /// // Of course we could have done this in one line:
    /// // let install = Install::default().flavor(Some("^")).flavor(Some("maya2020")).build();
    /// # }
    /// ```
    pub fn flavor<I>(&mut self, value: Option<I>) -> &mut Self
    where
        I: Into<Flavor>,
    {
        match value {
            Some(val) => match self.flavors {
                Some(ref mut flavors) => {
                    flavors.insert(val.into());
                }
                None => {
                    let mut hset = HashSet::new();
                    hset.insert(val.into());
                    self.flavors = Some(hset);
                }
            },
            None => self.flavors = None,
        }
        self
    }

    /// Set a vec of flavor in the Install struct. This method may be called multiple times,
    /// accumulating flavors.
    ///
    /// # Example
    /// ```
    /// # fn main() {
    /// let mut install = Install::default();
    /// # use pk_make::Install;
    /// install.flavor(Some("^")).flavors(Some(vec!["maya2020"]));
    ///
    /// // Of course we could have done this in one line:
    /// // let install = Install::default().flavor(Some("^")).flavor(Some("maya2020")).build();
    /// # }
    /// ```
    pub fn flavors<I>(&mut self, value: Option<Vec<I>>) -> &mut Self
    where
        I: Into<Flavor>,
    {
        match value {
            Some(vals) => match self.flavors {
                Some(ref mut flavors) => {
                    for val in vals {
                        flavors.insert(val.into());
                    }
                }
                None => {
                    let mut hset = HashSet::new();
                    for val in vals {
                        hset.insert(val.into());
                    }
                    self.flavors = Some(hset);
                }
            },
            None => self.flavors = None,
        }
        self
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

    /// Set the overrides value and return a mutable reference to self
    pub fn overrides(&mut self, value: Option<Vec<OverridePair>>) -> &mut Self {
        self.overrides = value;
        self
    }

    /// Set the defines and return a mutable reference to self per the
    /// builder pattern.
    pub fn defines(&mut self, input: Option<Vec<String>>) -> &mut Self {
        self.defines = input;
        self
    }
    pub fn work(&mut self, input: bool) -> &mut Self {
        self.work = input;
        self
    }
    pub fn vcs(&mut self, input: Option<Vcs>) -> &mut Self {
        self.vcs = input;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_construct_default() {
        let result = Install::default();
        let expected = Install {
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
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn can_build() {
        let result = Install::default()
            .dry_run(true)
            .with_docs(false)
            .build_dir(Some("foo/bar"))
            .context(Some(Context::Facility))
            .show(Some("dev01"))
            .site(Some("all"))
            .platform(Some("cent7"))
            .flavor(Some("^"))
            .verbose(true)
            .build();
        let mut site_hs = HashSet::new();
        site_hs.insert(Site::All);
        let mut platforms_hs = HashSet::new();
        platforms_hs.insert(Platform::Cent7_64);
        let mut flavors_hs = HashSet::new();
        flavors_hs.insert(Flavor::Vanilla);

        let expected = Install {
            dry_run: true,
            with_docs: false,
            build_dir: Some("foo/bar".to_string()),
            context: Some(Context::Facility),
            show: Some("dev01".to_string()),
            sites: Some(site_hs),
            platforms: Some(platforms_hs),
            flavors: Some(flavors_hs),
            verbose: true,

            clean: false,
            dist_dir: None,
            level: None,
            overrides: None,
            defines: None,
            work: false,
            vcs: None,
        };
        assert_eq!(result, expected);
    }
}
/*
    dry_run: false,
 with_docs: true,
 context:None,
 show: None,
 sites:None,
 platforms: None,
 flavors: None,
 verbose: false
*/
