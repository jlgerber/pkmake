use anyhow::anyhow;
use anyhow::Error as AnyError;
use pk_make::build_env::BuildEnv;
use pk_make::targets::{Build, Docs, Install, Run, Test};
use pk_make::traits::Doit;
use pk_make::{context, flavor, platform, site};
use structopt::StructOpt;

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
        /// Override the default Output Distribution Directory
        #[structopt(short, long = "dist-dir")]
        dist_dir: Option<String>,
        /// Optionally specify one or more flavors. This option may be repeated multiple times
        #[structopt(short, long)]
        flavor: Option<Vec<flavor::Flavor>>,
        /// Provide the platform(s) to build for. This flag may be repeated.
        #[structopt(short, long)]
        platform: Option<Vec<platform::Platform>>,
        /// Provide more verbose output
        #[structopt(short, long)]
        verbose: bool,
        #[structopt(short = "D", long)]
        define: Option<Vec<String>>,
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
        #[structopt(short, long = "build-dir")]
        build_dir: Option<String>,
        /// Controls  verbose output to shell
        #[structopt(short, long)]
        verbose: bool,
    },
    #[structopt(display_order = 3)]
    /// Build documentation
    Docs {
        /// Specify the build directory
        #[structopt(short, long = "build-dir")]
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
        #[structopt(short, long = "build-dir")]
        build_dir: Option<String>,
        /// Provide more verbose output
        #[structopt(short, long)]
        verbose: bool,
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

        /// Provide verbose output while executing command
        #[structopt(short, long)]
        verbose: bool,
        vars: Vec<String>,
    },
}

fn main() -> Result<(), AnyError> {
    let opt = Opt::from_args();
    match opt {
        Opt::Build {
            skip_docs,
            dry_run,
            dist_dir,
            flavor,
            platform,
            verbose,
            define,
        } => {
            let mut build = Build::default()
                .with_docs(!skip_docs)
                .dry_run(dry_run)
                .dist_dir(dist_dir)
                .flavors(flavor)
                .platforms(platform)
                .verbose(verbose)
                .defines(define)
                .build();
            build.doit()
        }
        Opt::Install {
            skip_docs,
            context,
            show,
            site,
            platform,
            flavor,
            build_dir,
            verbose,
        } => {
            let mut install = Install::default()
                .with_docs(!skip_docs)
                .context(context)
                .show(show)
                .sites(site)
                .platforms(platform)
                .flavors(flavor)
                .build_dir(build_dir)
                .verbose(verbose)
                .build();
            install.doit()
        }
        Opt::Docs {
            build_dir,
            dry_run,
            verbose,
        } => {
            let mut docs = Docs::default()
                .dry_run(dry_run)
                .build_dir(build_dir)
                .verbose(verbose)
                .build();
            docs.doit()
        }
        Opt::Test {
            dry_run,
            build_dir,
            verbose,
        } => {
            let mut test = Test::default()
                .dry_run(dry_run)
                .build_dir(build_dir)
                .verbose(verbose)
                .build();
            test.doit()
        }
        Opt::Run {
            dry_run,
            verbose,
            vars,
        } => {
            let mut run = Run::default()
                .dry_run(dry_run)
                .verbose(verbose)
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
