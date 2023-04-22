// FIXME: place this in "utils" folder, probably organize as a crate

//!
//! 1. Configure hooks folder
//!
//! ```sh
//! git config core.hooksPath utils/git/hooks
//! ```
//!
//! 2. grant exec permission for git hooks on Unix-like platforms
//!
//! ```sh
//! chmod +x ${HOOKS_FOLDER}/pre-commit
//! chmod +x ${HOOKS_FOLDER}/...
//! ```
//!

use ergo_traits::*;
use std::{env, error::Error, os::unix::fs::PermissionsExt, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    // 1. Configure hooks folder
    // `git config core.hooksPath utils/git/hooks`
    let cwd = env::current_dir()?; // TODO: make this customizable
    dbg!(&cwd);
    let repo = git2::Repository::open(cwd)?;
    let hooks_path = Path::new("utils/git/hooks"); // FIXME: evaluate accurate path to resource files of this crate
    repo.config()?.set_str(
        "core.hooksPath",
        hooks_path.to_str().ok_or("path to hooks folder")?,
    )?;

    // 2. grant exec permission for git hooks on Unix-like platforms
    let hooks = ""; // TODO
    std::fs::set_permissions(hooks, PermissionsExt::from_mode(0o700))?;
    ().into_ok()
}
