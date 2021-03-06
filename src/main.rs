//! main
//! 
//! Implements pk-make cli and invokes pk_make lib
// crate imports
use anyhow::Error as AnyError;
use pk_make::{context, flavor, platform, site, OverridePair, Vcs};
use pk_make::targets::{Build, Docs, Install, Run, Test};
use pk_make::traits::Doit;
use std::path::PathBuf;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name = "pk-make", about = "Invoke pk recipes.")]
enum Opt {
    /// Build one or more flavors of a package
    #[structopt(display_order = 1)]
    Build {
        /// clean
        #[structopt(long)]
        clean: bool,
        /// Do not build the docs when building the main artifact(s)
        #[structopt(long = "skip-docs")]
        skip_docs: bool,

        /// Print out commands but do not execute them
        #[structopt(short = "n", long = "dry-run")]
        dry_run: bool,

        /// Override the default Output Distribution Directory
        #[structopt(short, long = "dist-dir")]
        dist_dir: Option<String>,

        /// Optionally specify one or more flavors. This option may be repeated multiple times
        #[structopt(short, long)]
        flavor: Option<Vec<flavor::Flavor>>,

        /// The target level's repository specified as a level-spec
        #[structopt(short = "L", long)]
        level: Option<String>,

        /// Only write out package metadata
        #[structopt(long = "metadata-only")]
        metadata_only: bool,

        /// Override version from version-lock
        #[structopt(short, long = "override")]
        overrides: Option<Vec<OverridePair>>,

        /// Provide the platform(s) to build for. This flag may be repeated.
        #[structopt(short = "P", long)]
        platform: Option<Vec<platform::Platform>>,

        /// Provide more verbose output
        #[structopt(short, long)]
        verbose: bool,

        /// Pass variable through to the recipe
        #[structopt(short = "D", long)]
        define: Option<Vec<String>>,

        /// Include packages from the user workarea
        #[structopt(long)]
        work: bool,

        /// Optionally specify a path to the package root directory
        #[structopt(short = "r", long = "package-root", parse(from_os_str))]
        package_root: Option<PathBuf>,
    },
    #[structopt(display_order = 2)]
    /// Build and install one or more flavors of a package to one or more platforms
    Install {
        /// clean
        #[structopt(long)]
        clean: bool,

        /// Print out commands but do not execute them
        #[structopt(short = "n", long = "dry-run")]
        dry_run: bool,

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
        #[structopt(short = "P", long)]
        platform: Option<Vec<platform::Platform>>,

        /// Optionally provide the flavor or flavors to build. May be vanilla, %, or a flavor name
        #[structopt(short, long)]
        flavor: Option<Vec<flavor::Flavor>>,

        /// Specify the build directory
        #[structopt(short, long = "build-dir")]
        build_dir: Option<String>,

        /// Controls  verbose output to shell
        #[structopt(short, long)]
        verbose: bool,

        /// Override the default Output Distribution Directory
        #[structopt(short, long = "dist-dir")]
        dist_dir: Option<String>,

        /// The target level's repository specified as a level-spec
        #[structopt(short = "L", long)]
        level: Option<String>,

        /// Override version from version-lock
        #[structopt(short, long = "override")]
        overrides: Option<Vec<OverridePair>>,

        /// Pass variable through to the recipe
        #[structopt(short = "D", long)]
        define: Option<Vec<String>>,

        /// Include packages from the user workarea
        #[structopt(long)]
        work: bool,

        /// choose a vcs system manually (sometimes necessary)
        #[structopt(long)]
        vcs: Option<Vcs>,

        /// Specify output log file location
        #[structopt(long, parse(from_os_str))]
        logfile: Option<PathBuf>,

        /// Specify the maximum number of workers used
        #[structopt(short = "j", long = "max-jobs")]
        max_jobs: Option<u8>,

        /// Optionally specify a path to the package root directory
        #[structopt(short = "r", long = "package-root", parse(from_os_str))]
        package_root: Option<PathBuf>,
    },
    #[structopt(display_order = 3)]
    /// Build documentation
    Docs {
        /// Specify the build directory
        #[structopt(short, long = "dist-dir")]
        dist_dir: Option<String>,

        /// Print out commands but do not execute them
        #[structopt(short = "n", long = "dry-run")]
        dry_run: bool,

        /// Controls  verbose output to shell
        #[structopt(short, long)]
        verbose: bool,

        /// Pass variable through to the recipe
        #[structopt(short = "D", long)]
        define: Option<Vec<String>>,

        /// Optionally provide the platform or platforms to build for
        #[structopt(short = "P", long)]
        platform: Option<Vec<platform::Platform>>,

        /// Optionally provide the flavor or flavors to build. May be vanilla, %, or a flavor name
        #[structopt(short, long)]
        flavor: Option<Vec<flavor::Flavor>>,

        /// Optionally specify a path to the package root directory
        #[structopt(short = "r", long = "package-root", parse(from_os_str))]
        package_root: Option<PathBuf>,
    },
    #[structopt(display_order = 4)]
    /// Run tests via the pk test target
    Test {
        /// Print out commands but do not execute them
        #[structopt(short = "n", long = "dry-run")]
        dry_run: bool,

        /// Specify the build directory
        #[structopt(short, long = "dist-dir")]
        dist_dir: Option<String>,

        /// Provide more verbose output
        #[structopt(short, long)]
        verbose: bool,

        /// Optionally provide the platform or platforms to build for
        #[structopt(short = "P", long)]
        platform: Option<Vec<platform::Platform>>,

        /// Optionally provide the flavor or flavors to build. May be vanilla, %, or a flavor name
        #[structopt(short, long)]
        flavor: Option<Vec<flavor::Flavor>>,

        /// Pass variable through to the recipe
        #[structopt(short = "D", long)]
        define: Option<Vec<String>>,

        /// Optionally specify a path to the package root directory
        #[structopt(short = "r", long = "package-root", parse(from_os_str))]
        package_root: Option<PathBuf>,
    },
    /// Execute an arbitrary pk recipe via pk run-recipe.
    #[structopt(
        setting(structopt::clap::AppSettings::TrailingVarArg),
        display_order = 5
    )]
    Run {
        /// Print but do not execute Command
        #[structopt(short = "n", long = "dry-run")]
        dry_run: bool,

        /// Optionally specify a path to the package root directory
        #[structopt(short = "r", long = "package-root", parse(from_os_str))]
        package_root: Option<PathBuf>,

        /// Provide verbose output while executing command
        #[structopt(short, long)]
        verbose: bool,

        /// Optionally provide the platform or platforms to build for
        #[structopt(short = "P", long)]
        platform: Option<Vec<platform::Platform>>,

        /// Optionally provide the flavor or flavors to build. May be vanilla, %, or a flavor name
        #[structopt(short, long)]
        flavor: Option<Vec<flavor::Flavor>>,

        vars: Vec<String>,
    },
}

fn main() -> Result<(), AnyError> {
    let opt = Opt::from_args();
    match opt {
        Opt::Build {
            clean,
            skip_docs,
            dry_run,
            dist_dir,
            flavor,
            level,
            metadata_only,
            overrides,
            platform,
            verbose,
            define,
            work,
            package_root,
        } => {
            let mut build = Build::default()
                .clean(clean)
                .with_docs(!skip_docs)
                .dry_run(dry_run)
                .dist_dir(dist_dir)
                .flavors(flavor)?
                .level(level)
                .metadata_only(metadata_only)
                .overrides(overrides)?
                .platforms(platform)?
                .verbose(verbose)
                .defines(define)
                .work(work)
                .package_root(package_root)
                .build();
            build.doit()
        }
        Opt::Install {
            skip_docs,
            dry_run,
            context,
            show,
            site,
            platform,
            flavor,
            build_dir,
            verbose,
            clean,
            dist_dir,
            level,
            overrides,
            define,
            work,
            vcs,
            logfile,
            max_jobs,
            package_root,
        } => {
            let mut install = Install::default()
                .clean(clean)
                .dry_run(dry_run)
                .with_docs(!skip_docs)
                .context(context)?
                .show(show)
                .sites(site)?
                .platforms(platform)?
                .flavors(flavor)?
                .build_dir(build_dir)
                .verbose(verbose)
                .dist_dir(dist_dir)
                .level(level)?
                .overrides(overrides)?
                .defines(define)
                .work(work)
                .vcs(vcs)?
                .logfile(logfile)
                .max_jobs(max_jobs)
                .package_root(package_root)
                .build();
            install.doit()
        }
        Opt::Docs {
            dist_dir,
            dry_run,
            verbose,
            define,
            flavor,
            platform,
            package_root,
        } => {
            let mut docs = Docs::default()
                .dry_run(dry_run)
                .dist_dir(dist_dir)
                .defines(define)
                .verbose(verbose)
                .flavors(flavor)?
                .platforms(platform)?
                .package_root(package_root)
                .build();
            docs.doit()
        }
        Opt::Test {
            dry_run,
            dist_dir,
            verbose,
            platform,
            flavor,
            define,
            package_root,
        } => {
            let mut test = Test::default()
                .dry_run(dry_run)
                .dist_dir(dist_dir)
                .verbose(verbose)
                .platforms(platform)?
                .flavors(flavor)?
                .defines(define)
                .package_root(package_root)
                .build();
            test.doit()
        }
        Opt::Run {
            dry_run,
            verbose,
            package_root,
            platform,
            flavor,
            vars,
        } => {
            let mut run = Run::default()
                .dry_run(dry_run)
                .verbose(verbose)
                .package_root(package_root)
                .platforms(platform)?
                .flavors(flavor)?
                .vars(vars)
                .build();
            run.doit()
        }
    }
}

/*
 * The following is a list of flags from Makebridge
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
