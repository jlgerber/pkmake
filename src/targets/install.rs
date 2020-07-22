use std::collections::HashSet;
use crate::flavor::Flavor;
use crate::site::Site;
use crate::context::Context;
use crate::platform::Platform;

#[derive(Debug, PartialEq, Eq)]
/// Install target.
pub struct Install {
    pub dry_run: bool,
    pub with_docs: bool,
    pub context: Option<Context>,
    pub show: Option<String>,
    pub sites: Option<HashSet<Site>>,
    pub platforms: Option<HashSet<Platform>>,
    pub flavors: Option<HashSet<Flavor>>,
    pub verbose: bool,
}

impl Default for Install {
    fn default() -> Self {
        Self {
            dry_run: false,
            with_docs: true,
            context:None, 
            show: None,
            sites:None, 
            platforms: None,
            flavors: None,
            verbose: false
        }
    }
}

impl Install {
    
    /// Set the dry_run field.
    ///
    /// # Example 
    /// ```
    /// #fn main() {
    /// let install = Install::default().dry_run(true).build();
    /// #}
    /// ```
    pub fn dry_run(&mut self, input: bool) -> &mut Self {
        self.dry_run = input;
        self
    }
    /// set with_docs  field on Install struct. 
    ///
    /// # Example 
    /// ```
    /// #fn main() {
    /// let install = Install::default().with_docs(false).build();
    /// #}
    /// ```
    pub fn with_docs(&mut self, value: bool) -> &mut Self {
        self.with_docs = value;
        self
    }
    
    /// Set the context on the Install struct. 
    ///
    /// # Example 
    /// ```
    /// #fn main() {
    /// let install = Install::default().context(Some("facility")).build();
    /// #}
    /// ```
    pub fn context<I>(&mut self, value: Option<I> ) 
    -> &mut Self 
        where I: Into<Context> 
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
    /// #fn main() {
    /// let install = Install::default().show(Some("DEV01")).build();
    /// #}
    /// ```
    pub fn show<I>(&mut self,value: Option<I> ) -> &mut Self where I: Into<String> {
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
    /// #fn main() {
    /// let install = Install::default()
    ///                 .site(Some("vancouver"))
    ///                 .site(Some("playa"))
    ///                 .build();
    /// #}
    /// ```
    pub fn site<I>(&mut self, value: Option<I>) -> &mut Self where I: Into<Site> {
        match value {
            Some(val) => match self.sites {
                Some(ref mut sites) => {sites.insert(val.into());},
                None => {
                    let mut hset = HashSet::new();
                    hset.insert(val.into());
                    self.sites = Some(hset);
                }
            }
            None => self.sites = None,
        }
        self
    }
    
    /// Add a platform to the list of platforms on the Install struct. This may be called
    /// multiple times to accumulate platforms. 
    ///
    /// # Example 
    /// ```
    /// #fn main() {
    /// let install = Install::default()
    ///                 .platform(Some("cent6"))
    ///                 .platform(Some("cent7"))
    ///                 .build();
    /// #}
    /// ```
    pub fn platform<I>(&mut self, value: Option<I>) -> &mut Self where I: Into<Platform> {
        match value {
            Some(val) => match self.platforms{
                Some(ref mut platforms) => {platforms.insert(val.into());},
                None => {
                    let mut hset = HashSet::new();
                    hset.insert(val.into());
                    self.platforms= Some(hset);
                }
            }
            None => self.platforms= None,
        }
        self
    }

    /// Set a flavor in the Install struct. This method may be called multiple times, 
    /// accumulating flavors.
    ///
    /// # Example 
    /// ```
    /// #fn main() {
    /// let mut install = Install::default();
    /// install.flavor(Some("^")).flavor(Some("maya2020"));
    /// 
    /// // Of course we could have done this in one line: 
    /// // let install = Install::default().flavor(Some("^")).flavor(Some("maya2020")).build();
    /// #}
    /// ```
    pub fn flavor<I>(&mut self, value: Option<I>) -> &mut Self where I: Into<Flavor> {
        match value {
            Some(val) => match self.flavors{
                Some(ref mut flavors) => {flavors.insert(val.into());},
                None => {
                    let mut hset = HashSet::new();
                    hset.insert(val.into());
                    self.flavors= Some(hset);
                }
            }
            None => self.flavors= None,
        }
        self
    }
    
    /// Set the verbose field in the Install struct 
    pub fn verbose(&mut self, input: bool) -> &mut Self {
        self.verbose = input;
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
            context: None,
            show: None, 
            sites: None,
            platforms: None,
            flavors: None,
            verbose: false
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn can_build() {
        let result = Install::default()
                        .dry_run(true)
                        .with_docs(false)
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
            context: Some(Context::Facility),
            show: Some("dev01".to_string()),
            sites: Some(site_hs),
            platforms: Some(platforms_hs),
            flavors: Some(flavors_hs),
            verbose: true
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
