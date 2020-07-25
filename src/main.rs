//use anyhow::anyhow;
use anyhow::Error as AnyError;
//use pk_make::build_env::BuildEnv;
use pk_make::targets::{Build, Docs, Install, Run, Test};
use pk_make::traits::Doit;
use pk_make::{context, flavor, platform, site, OverridePair, Vcs};
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
        #[structopt(short, long)]
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
        } => {
            let mut build = Build::default()
                .clean(clean)
                .with_docs(!skip_docs)
                .dry_run(dry_run)
                .dist_dir(dist_dir)
                .flavors(flavor)
                .level(level)
                .metadata_only(metadata_only)
                .overrides(overrides)
                .platforms(platform)
                .verbose(verbose)
                .defines(define)
                .work(work)
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
        } => {
            let mut install = Install::default()
                .clean(clean)
                .dry_run(dry_run)
                .with_docs(!skip_docs)
                .context(context)
                .show(show)
                .sites(site)?
                .platforms(platform)?
                .flavors(flavor)
                .build_dir(build_dir)
                .verbose(verbose)
                .dist_dir(dist_dir)
                .level(level)
                .overrides(overrides)?
                .defines(define)
                .work(work)
                .vcs(vcs)
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
