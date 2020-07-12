extern crate getopts;
use getopts::Options;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::iter::Iterator;
use std::path::{Path, PathBuf};
use std::process::Command;

type DynError = Box<dyn std::error::Error>;

#[derive(Clone, Copy)]
pub struct PackageConf<'a> {
    pub name: &'a str,
    pub post_build: fn(conf: &Config) -> Result<(), DynError>,
    pub install: fn(conf: &Config) -> Result<(), DynError>,
}

pub struct Config<'a> {
    pub subcommand: String,
    target: String,      // target-triple
    target_dir: String,  // directory for all generated artifact
    install_dir: String, // directory for installation
    release: bool,
    packages_conf: Vec<PackageConf<'a>>,
    opts: Options,
}

impl<'a> Config<'a> {
    ///build config from environment variable and passed argument
    pub fn from_env(all_packages_conf: &'a [PackageConf]) -> Result<Self, DynError> {
        let mut args = env::args();
        //get subcommand
        let subcommand = if let Some(arg) = args.nth(1) {
            arg
        } else {
            String::from("")
        };

        //get build options
        let mut opts_args = Vec::<String>::new();
        for e in args {
            if e == "--" {
                break;
            }
            opts_args.push(e);
        }
        let mut opts = Options::new();
        opts.optmulti("p", "project", "project to build", "NAME");
        opts.optflag("", "all", "build all projects");
        opts.optflag("", "release", "build in release mode, with optimization");
        opts.optopt("", "target", "build for the target triple", "TRIPLE");
        opts.optopt(
            "",
            "target-dir",
            "directory for all generated artifacts",
            "DIRECTORY",
        );
        opts.optopt("", "install-dir", "installation directory", "DIRECTORY");
        opts.optflag("h", "help", "print this help menu");
        let matches = opts.parse(&opts_args)?;

        //build config data from option and/or environment variable
        let target = if let Some(s) = matches.opt_str("target") {
            s
        } else if let Some(var) = env::var_os("CARGO_BUILD_TARGET") {
            var.into_string().unwrap()
        } else {
            String::from("")
        };

        let target_dir = if let Some(s) = matches.opt_str("target-dir") {
            s
        } else if let Some(var) = env::var_os("CARGO_TARGET_DIR") {
            var.into_string().unwrap()
        } else if let Some(var) = env::var_os("CARGO_BUILD_TARGET_DIR") {
            var.into_string().unwrap()
        } else {
            String::from("target")
        };

        let install_dir = if let Some(s) = matches.opt_str("install-dir") {
            s
        } else if target.contains("apple") {
            env::var("HOME").unwrap() + "/Library/Audio/Plug-Ins/LV2"
        } else if target.contains("windows") {
            env::var("APPDATA").unwrap() + "/LV2"
        } else if cfg!(target_vendor = "apple") {
            env::var("HOME").unwrap() + "/Library/Audio/Plug-Ins/LV2"
        } else if cfg!(target_os = "windows") {
            env::var("APPDATA").unwrap() + "/LV2"
        } else {
            env::var("HOME").unwrap() + "/.lv2"
        };

        let release = matches.opt_present("release");

        //list of package to build
        let packages_conf = if matches.opt_present("all") || !matches.opt_present("project") {
            all_packages_conf
                .iter()
                .copied()
                .collect::<Vec<PackageConf>>()
        } else {
            let mut tmp = Vec::<PackageConf>::new();
            let project = matches.opt_strs("p");
            'proj_loop: for proj in project {
                for pkg_conf in all_packages_conf {
                    if proj == pkg_conf.name {
                        tmp.push(*pkg_conf);
                        continue 'proj_loop;
                    }
                }
                return Err(format!("No project named `{}`", proj).into());
            }
            tmp
        };

        Ok(Self {
            subcommand,
            target,
            target_dir,
            install_dir,
            release,
            packages_conf,
            opts,
        })
    }

    ///Display help
    pub fn print_help(&self) {
        let brief = "Usage: cargo xtask SUBCOMMAND [options]";
        let mut usage = self.opts.usage(&brief);
        let more_help = "
    Subcommands are:
        build   build lv2 project(s)

    Handled environment variables:
        CARGO_BUILD_TARGET
        CARGO_TARGET_DIR
        CARGO_BUILD_TARGET_DIR

";
        usage.push_str(&more_help);
        print!("{}", usage);
    }

    ///Absolute path to the Cargo build directory
    pub fn build_dir(&self) -> PathBuf {
        let profile_dir = if self.release { "release" } else { "debug" };
        workspace_root()
            .join(&self.target_dir)
            .join(&self.target)
            .join(profile_dir)
    }

    pub fn install_dir(&self) -> PathBuf {
        PathBuf::from(&self.install_dir)
    }

    fn packages_conf(&self) -> Vec<PackageConf> {
        self.packages_conf.clone()
    }

    ///Guess the prefix used by Cargo when building a dynamic library with Cargo
    pub fn lib_prefix(&self) -> String {
        let prefix = if self.target.contains("apple") {
            "lib"
        } else if self.target.contains("windows") {
            ""
        } else if cfg!(target_vendor = "apple") {
            "lib"
        } else if cfg!(target_os = "windows") {
            ""
        } else {
            "lib"
        };
        String::from(prefix)
    }

    ///Guess the suffix (i.e. extension) used by Cargo when building a dynamic library with Cargo
    pub fn lib_suffix(&self) -> String {
        let suffix = if self.target.contains("apple") {
            ".dylib"
        } else if self.target.contains("windows") {
            ".dll"
        } else if cfg!(target_vendor = "apple") {
            ".dylib"
        } else if cfg!(target_os = "windows") {
            ".dll"
        } else {
            ".so"
        };
        String::from(suffix)
    }
}

///Do the job
pub fn do_job(packages: &[PackageConf]) -> Result<(), DynError> {
    let mut conf = Config::from_env(packages)?;
    match conf.subcommand.as_ref() {
        "build" => build(&mut conf)?,
        "install" => install(&mut conf)?,
        "debug" => debug(&mut conf)?,
        _ => conf.print_help(),
    }
    Ok(())
}

///Build a full lv2 plugin
pub fn build(conf: &mut Config) -> Result<(), DynError> {
    let mut cargo_args = Vec::<String>::new();
    if conf.release {
        cargo_args.push(String::from("--release"));
    }
    if conf.target != "" {
        cargo_args.push(String::from("--target"));
        cargo_args.push(conf.target.clone());
    }
    cargo_args.push(String::from("--target-dir"));
    cargo_args.push(conf.target_dir.clone());

    for p in conf.packages_conf() {
        cargo_args.push(String::from("-p"));
        cargo_args.push(String::from(p.name));
    }
    println!("Building binarie(s)");
    cargo("build", &cargo_args)?;
    println!("Post build step(s)");
    for p in conf.packages_conf() {
        (p.post_build)(conf)?;
    }
    println!("Finished");
    println!();
    Ok(())
}

///Build and install lv2 plugin
pub fn install(conf: &mut Config) -> Result<(), DynError> {
    build(conf)?;
    println!("Installing plugin(s)");
    for p in conf.packages_conf() {
        (p.install)(conf)?;
    }
    println!("Finished");
    println!();
    Ok(())
}

///Create a new file using a template and a substitution list
pub fn subst<P: AsRef<Path>, Q: AsRef<Path>>(
    in_path: P,
    out_path: Q,
    subs: &[(&str, &str)],
) -> Result<(), DynError> {
    let mut in_file = BufReader::new(File::open(in_path)?);
    let mut out_file = BufWriter::new(File::create(out_path)?);
    let mut buf = String::new();
    while in_file.read_line(&mut buf).unwrap() != 0 {
        for (token, value) in subs {
            buf = buf.replace(token, value);
        }
        write!(out_file, "{}", buf)?;
        buf.clear();
    }
    Ok(())
}

///Return a BTreeSet that represent the tree of a directory
fn dir_tree<P: AsRef<Path>>(path: P) -> BTreeSet<PathBuf> {
    let mut paths: BTreeSet<PathBuf> = BTreeSet::new();
    for entry in fs::read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            paths.append(&mut dir_tree(&path));
        }
        paths.insert(path);
    }
    paths
}

///Copy recursively a directory
pub fn copy_dir<P: AsRef<Path>, Q: AsRef<Path>>(in_path: P, out_path: Q) -> Result<(), DynError> {
    let in_path = in_path.as_ref();
    let out_path = out_path.as_ref();
    if !in_path.is_dir() {
        return Err(format!("'{:?}' is not a directory ", &in_path).into());
    }
    fs::create_dir_all(&out_path).unwrap();
    for src_path in dir_tree(in_path) {
        let dest_path = out_path.join(src_path.strip_prefix(in_path).unwrap());
        if src_path.is_dir() {
            fs::create_dir_all(&dest_path).unwrap();
        } else if src_path.is_file() {
            fs::copy(src_path, dest_path).unwrap();
        }
    }
    Ok(())
}

macro_rules! print_env {
    ( $x:expr) => {{
        println!(
            stringify!($x {}),
            env::var(stringify!($x)).unwrap_or_else(|e| format!("{}", e))
        );
    }};
}

pub fn debug(_conf: &mut Config) -> Result<(), DynError> {
    print_env!(CARGO);
    print_env!(CARGO_MANIFEST_DIR);
    print_env!(CARGO_PKG_VERSION);
    print_env!(CARGO_PKG_VERSION_MAJOR);
    print_env!(CARGO_PKG_VERSION_MINOR);
    print_env!(CARGO_PKG_VERSION_PATCH);
    print_env!(CARGO_PKG_VERSION_PRE);
    print_env!(CARGO_PKG_AUTHORS);
    print_env!(CARGO_PKG_NAME);
    print_env!(CARGO_PKG_DESCRIPTION);
    print_env!(CARGO_PKG_HOMEPAGE);
    print_env!(CARGO_PKG_REPOSITORY);
    print_env!(OUT_DIR);
    print_env!(TARGET);
    print_env!(CARGO_CFG_TARGET_OS);
    Ok(())
}

///Invoke a Cargo subcommand
pub fn cargo(cmd: &str, args: &[String]) -> Result<(), DynError> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(workspace_root())
        .arg(cmd)
        .args(args)
        .status()?;

    if !status.success() {
        return Err(format!("cargo {} failed", cmd).into());
    }
    Ok(())
}

///Get the root path of the current workspace.
///
///This require `CARGO_MANIFEST_DIR` environment variable to be set at compile time which is
///normally the case when it's compiled with Cargo
pub fn workspace_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
