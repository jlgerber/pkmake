use structopt::{StructOpt};

use pk_make::build_env::BuildEnv;
use pk_make::targets::{Build, Install, Docs, Test};
use pk_make::traits::Doit;
use pk_make::{flavor, context, site, platform};
/*
 flag translation
 CONTEXT => --context 
 LEVEL => --level 
 SHOW => --show 
 WITH_DOCS => --with-docs 
 DRY_RUN => -n --dry-run 
 FLAVOUR => --flavor --flavour 
 SITES --sites (local|all|name)
 BUILD_DIR --build-dir 
 PLATFORMS => --platforms 
 VERBOSe => --verbose 
 VCS => --vcs 
 pk-make 
*/

#[derive(Debug, StructOpt)]
#[structopt(name = "pk-make", about = "Invoke pk recipes.")]
enum Opt {
    /// Build one or more flavors of a package 
    #[structopt(display_order = 1)]
    Build {
        /// Do not build the docs when building the main artifact(s)
        #[structopt(long = "skip-docs")]
        skip_docs: bool,
        
        /// Print out commands but do not execute them
        #[structopt(short = "n", long = "dry-run")]
        dry_run: bool,
        
        /// Specify the build directory
        #[structopt(short, long="build-dir")]
        build_dir: Option<String>,
        
        /// Optionally specify one or more flavors. This option may be repeated multiple times
        #[structopt(short,long)]
        flavor: Option<Vec<flavor::Flavor>>,
        
        /// Provide more verbose output
        #[structopt(short,long)]
        verbose: bool,
    },
    #[structopt(display_order = 2)]
    /// Build and install one or more flavors of a package to one or more platforms 
    Install {
        /// Do not build the docs as part of the install  
        #[structopt(long = "skip-docs")]
        skip_docs: bool,
        
        /// Context may be either facility | shared | user. Defaults to user. 
        #[structopt(short, long)] 
        context: Option<context::Context>,
        
        /// The current show 
        #[structopt(long)]
        show: Option<String>,
        
        /// The site or sites. Site may be all | local | <site>. This may be 
        /// repeated one or more times 
        #[structopt(short, long)]
        site: Option<Vec<site::Site>>,
        
        /// Optionally provide the platform or platforms to build for
        #[structopt(short, long)]
        platform: Option<Vec<platform::Platform>>,
        
        /// Optionally provide the flavor or flavors to build. May be vanilla, %, or a flavor name 
        #[structopt(short, long)]
        flavor: Option<Vec<flavor::Flavor>>,
         
        /// Specify the build directory
        #[structopt(short, long="build-dir")]
        build_dir: Option<String>,
        
        /// Controls  verbose output to shell
        #[structopt(short, long)]
        verbose: bool,
    },
    #[structopt(display_order = 3)]
    /// Build documentation
    Docs  {
        /// Specify the build directory
        #[structopt(short, long="build-dir")]
        build_dir: Option<String>,
         
        /// Print out commands but do not execute them
        #[structopt(short = "n", long = "dry-run")]
        dry_run: bool,
         
        /// Controls  verbose output to shell
        #[structopt(short, long)]
        verbose: bool,
    },
    #[structopt(display_order = 4)]
    /// Run tests via the pk test target 
    Test {
        /// Print out commands but do not execute them
        #[structopt(short = "n", long = "dry-run")]
        dry_run: bool,
        
        /// Specify the build directory
        #[structopt(short, long="build-dir")]
        build_dir: Option<String>,
        
        /// Provide more verbose output
        #[structopt(short,long)]
        verbose: bool,
    },
    /// Execute an arbitrary pk recipe via pk run-recipe. All arguments are passed to pk run-recipe, which is
    /// responsible for validatio.
    #[structopt(setting(structopt::clap::AppSettings::TrailingVarArg), display_order=5)] 
    Run {
        vars: Vec<String>
    }
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Build{
            skip_docs, 
            dry_run, 
            build_dir, 
            flavor, 
            verbose
        } => {
            let build = Build::default()
                        .with_docs(!skip_docs)
                        .dry_run(dry_run)
                        .build_dir(build_dir)
                        .flavors(flavor)
                        .verbose(verbose)
                        .build();
           build.doit().unwrap(); 
        },
        Opt::Install{
            skip_docs, 
            context, 
            show, 
            site, 
            platform, 
            flavor, 
            build_dir, 
            verbose
        } => {
            let install = Install::default()
                            .with_docs(!skip_docs)
                            .context(context)
                            .show(show)
                            .sites(site)
                            .platforms(platform)
                            .flavors(flavor)
                            .build_dir(build_dir)
                            .verbose(verbose)
                            .build();
        }
        _ => unimplemented!()
    }
}
