pub mod modules;
mod yafetch;

use std::path::PathBuf;
use std::process::ExitCode;

use mlua::Lua;

const USAGE: &str = "\
usage:
  yafetch [<config>]     render a configuration, ~/.config/yafetch/init.lua by default
  yafetch -V, --version  print the version and exit
  yafetch -h, --help     print this message and exit
";

const SAMPLE: &str =
    "https://raw.githubusercontent.com/pkarpovich/yafetch/main/examples/sample.lua";

/// What the command line asks for.
#[derive(Debug, PartialEq, Eq)]
enum Command {
    /// Render a configuration: the one named, or the one in the config directory.
    Render(Option<PathBuf>),
    Version,
    Help,
    Unknown(String),
    Surplus,
}

/// Reads what was asked for from the arguments after the program's own name.
fn command(args: &[String]) -> Command {
    match args {
        [] => Command::Render(None),
        [only] => match only.as_str() {
            "-V" | "--version" => Command::Version,
            "-h" | "--help" => Command::Help,
            flag if flag.starts_with('-') => Command::Unknown(flag.to_string()),
            path => Command::Render(Some(PathBuf::from(path))),
        },
        _ => Command::Surplus,
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match command(&args) {
        Command::Version => {
            println!("yafetch {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Command::Help => {
            print!("{USAGE}");
            ExitCode::SUCCESS
        }
        Command::Unknown(flag) => misuse(&format!("unknown option {flag}")),
        Command::Surplus => misuse("one configuration file at a time"),
        Command::Render(path) => render(path),
    }
}

fn render(path: Option<PathBuf>) -> ExitCode {
    let path = match path {
        Some(path) => path,
        None => match configured() {
            Ok(path) => path,
            Err(reason) => return refuse(&reason),
        },
    };

    let yafetch = yafetch::Yafetch { lua: Lua::new() };
    yafetch.register();
    let Err(reason) = yafetch.run(&path) else {
        return ExitCode::SUCCESS;
    };
    refuse(&reason)
}

/// The configuration in the user's config directory, or why there is none to read.
fn configured() -> Result<PathBuf, String> {
    let directories = xdg::BaseDirectories::with_prefix("yafetch");
    let Some(path) = directories.find_config_file("init.lua") else {
        return Err(format!(
            "no configuration: ~/.config/yafetch/init.lua does not exist\n\
             start from the sample: curl --create-dirs -o ~/.config/yafetch/init.lua {SAMPLE}"
        ));
    };
    Ok(path)
}

fn refuse(reason: &str) -> ExitCode {
    eprintln!("yafetch: {reason}");
    ExitCode::FAILURE
}

fn misuse(reason: &str) -> ExitCode {
    eprintln!("yafetch: {reason}");
    eprint!("{USAGE}");
    ExitCode::FAILURE
}

#[cfg(test)]
mod tests {
    use super::*;

    fn asked(args: &[&str]) -> Command {
        let args: Vec<String> = args.iter().map(|argument| argument.to_string()).collect();
        command(&args)
    }

    #[test]
    fn no_argument_renders_the_configured_file() {
        assert_eq!(asked(&[]), Command::Render(None));
    }

    #[test]
    fn a_path_renders_that_file() {
        assert_eq!(
            asked(&["other.lua"]),
            Command::Render(Some(PathBuf::from("other.lua")))
        );
    }

    #[test]
    fn the_version_is_asked_for_by_either_spelling() {
        assert_eq!(asked(&["-V"]), Command::Version);
        assert_eq!(asked(&["--version"]), Command::Version);
    }

    #[test]
    fn the_help_is_asked_for_by_either_spelling() {
        assert_eq!(asked(&["-h"]), Command::Help);
        assert_eq!(asked(&["--help"]), Command::Help);
    }

    #[test]
    fn an_option_that_is_not_understood_is_not_a_path() {
        assert_eq!(
            asked(&["--wat"]),
            Command::Unknown("--wat".to_string()),
            "an option read as a configuration path panics on a file that was never meant"
        );
        assert_eq!(asked(&["-x"]), Command::Unknown("-x".to_string()));
    }

    #[test]
    fn more_than_one_argument_is_a_misuse_rather_than_silence() {
        assert_eq!(asked(&["one.lua", "two.lua"]), Command::Surplus);
    }
}
