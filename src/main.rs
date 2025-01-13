use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::Write;
use std::os::unix::io::FromRawFd;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Setup command that prints shell function
    Setup {
        /// Name of the command to wrap
        name: String,
    },
    /// CD command that writes the directory change
    Cd {
        /// Directory to change to
        dir: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Setup { name } => {
            print!(r#"
{name}() {{
  local tmpfile
  tmpfile="$(\mktemp -u)"

  exec 9>"${{tmpfile}}"
  exec 8<"${{tmpfile}}"
  \rm -f "${{tmpfile}}"

  local exit_code
  command {name} "$@"
  exit_code=$?

  local finalizers
  finalizers=()

  local fin
  while \read -r fin; do
    finalizers+=("${{fin}}")
  done <&8

  exec 8<&- # close FD 8.
  exec 9<&- # close FD 9.

  for fin in "${{finalizers[@]}}"; do
    case "${{fin}}" in
      cd:*)
        cd "${{fin//cd:/}}"
        ;;
      *)
        >&2 echo "cdwrap: unknown finalizer: $fin"
        ;;
    esac
  done

  \return ${{exit_code}}
}}
"#, name = name);
        }
        Commands::Cd { dir } => {
            // Get file descriptor 9 which should be opened by the shell function
            let mut file = unsafe { File::from_raw_fd(9) };
            if writeln!(file, "cd:{}", dir).is_err() {
                eprintln!("Failed to write to file descriptor 9. This command must be run from within a function created by 'cdwrap setup'.");
                eprintln!("First run 'cdwrap setup NAME' and source its output, then run your command through the generated function.");
                std::process::exit(1);
            }
            // Don't close the file descriptor - the shell script will do that
            std::mem::forget(file);
        }
    }
}
