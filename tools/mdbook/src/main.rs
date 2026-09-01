//! Workspace-local wrapper around the `mdbook` binary.
//!
//! Adds a thin Rust-native entry point so the book can be built and served
//! via `cargo` without `mdbook` being in PATH *or* each contributor
//! remembering the exact CLI incantation. Run it as:
//!
//! ```text
//! cargo book build       # mdbook build book
//! cargo book serve       # mdbook serve book
//! cargo book test        # cargo test --workspace --doc (the doctests)
//! cargo book all         # build the book AND run doctests
//! cargo book --help      # list subcommands
//! ```
//!
//! If `mdbook` isn't installed, prints an install hint and exits with
//! code 1. No network, no sandbox setup, no plugin discovery — mdbook's
//! own binary handles all of that.
//!
//! ## Why a Rust wrapper for a Rust binary?
//!
//! `mdbook` is published on crates.io but its API surface is meant for
//! `cargo install`-ed binaries, not for direct invocation. Wrapping the
//! CLI is simpler than depending on the `mdbook` crate, which has a much
//! wider API surface than we need.

use std::ffi::OsString;
use std::path::PathBuf;
use std::process::{Command, ExitCode};

/// Help text used when `cargo book` is run without a subcommand.
const HELP: &str = "\
Usage: cargo book <command>

Commands:
  build   Render the book to ./book/book (mdbook build book)
  serve   Serve the book at http://localhost:3000 with live reload
  test    Run the workspace doctests (`cargo test --workspace --doc`)
  all     build, then test (CI entry point)
  help    Show this message

Install prerequisite:
  cargo install mdbook --locked

Book source: ./book/src
Book output: ./book/book
";

/// Subcommands this wrapper knows about.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Sub {
    Build,
    Serve,
    Test,
    All,
    Help,
}

fn main() -> ExitCode {
    let sub = match parse_args().as_deref() {
        Some("build") => Sub::Build,
        Some("serve") => Sub::Serve,
        Some("test") => Sub::Test,
        Some("all") => Sub::All,
        // Default to `Help` for `--help` / `-h` *and* for the no-arg case.
        Some("help" | "--help" | "-h") | None => Sub::Help,
        Some(other) => {
            eprintln!("Unknown subcommand: {other}");
            eprintln!("{HELP}");
            return ExitCode::from(2u8);
        }
    };

    match sub {
        Sub::Help => {
            println!("{HELP}");
            ExitCode::SUCCESS
        }
        Sub::Build => run_mdbook("build"),
        Sub::Serve => run_mdbook("serve"),
        Sub::Test => run_cargo_doc(),
        Sub::All => match run_mdbook("build") {
            ExitCode::SUCCESS => run_cargo_doc(),
            other => other,
        },
    }
}

/// Pulls the first positional argument out of `std::env::args`. Returns
/// `None` if there isn't one.
fn parse_args() -> Option<String> {
    let mut iter = std::env::args().skip(1);
    iter.next()
}

/// Where the book source lives, anchored at the workspace root.
fn book_dir() -> PathBuf {
    // The binary lives at <root>/target/debug/<binname> (during `cargo run`)
    // or <root>/tools/mdbook/target/<config>/<binname> (during `cargo book`).
    // We need to find <root> reliably.
    //
    // Strategy: walk up from CARGO_MANIFEST_DIR (which is set when cargo
    // invokes us) until we find a `book/src` directory.
    let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let manifest_path = PathBuf::from(&manifest);
    let mut dir = manifest_path.as_path();
    for _ in 0..6 {
        let candidate = dir.join("book");
        if candidate.join("src").is_dir() {
            return candidate;
        }
        if let Some(parent) = dir.parent() {
            dir = parent;
        } else {
            break;
        }
    }
    // Fall back to ./book relative to CWD.
    PathBuf::from("book")
}

/// Run `mdbook <subcommand> book` and propagate its exit code.
fn run_mdbook(subcommand: &str) -> ExitCode {
    let book = book_dir();
    if !book.join("book.toml").is_file() {
        eprintln!("Could not find a book.toml at {}.", book.display());
        eprintln!(
            "Are you running from the workspace root? Found manifest: {:?}",
            std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default()
        );
        return ExitCode::from(2u8);
    }

    let mut cmd = Command::new("mdbook");
    cmd.arg(subcommand).arg(&book);
    // mdbook prints status to stderr; pipe through.
    cmd.stdout(std::io::stdout());
    cmd.stderr(std::io::stderr());

    let status = match cmd.status() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to spawn `mdbook` ({e}).");
            eprintln!("Install it once with:");
            eprintln!("    cargo install mdbook --locked");
            eprintln!("Then re-run `cargo book {subcommand}`.");
            return ExitCode::from(127u8);
        }
    };

    exit_from_status(status)
}

/// Run `cargo test --workspace --doc`.
fn run_cargo_doc() -> ExitCode {
    let mut cmd = Command::new("cargo");
    cmd.args(["test", "--workspace", "--doc", "--quiet"]);
    cmd.stdout(std::io::stdout());
    cmd.stderr(std::io::stderr());

    let status = match cmd.status() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to spawn `cargo test --doc` ({e}).");
            return ExitCode::from(127u8);
        }
    };

    exit_from_status(status)
}

/// Map a child-process [`ExitStatus`] to an [`ExitCode`].
///
/// `ExitCode::from` only accepts `u8`, but [`ExitStatus::code`] returns
/// `Option<i32>`. This helper collapses both branches of `Option` and
/// the full i32 range down to the binary result we actually care about
/// for a CLI wrapper: success (`ExitCode::SUCCESS`) or failure
/// (`ExitCode::FAILURE`).
///
/// `ExitStatus` is `Copy` and only 4 bytes, so we take it by value rather
/// than by reference (`clippy::trivially_copy_pass_by_ref`).
fn exit_from_status(status: std::process::ExitStatus) -> ExitCode {
    if status.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

// _use OsString so the build doesn't warn about unused imports if we
// later add path-aware flags.
#[allow(dead_code)]
const _OS: fn() -> OsString = OsString::new;
