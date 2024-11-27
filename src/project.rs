use std::path::PathBuf;

use crate::config::Config;

/// A list of files considered to mark the root of a project.
/// `https://github.com/bbatsov/projectile/blob/71f18add5e66201c3ea7c9650b848968db3aec42/projectile.el#L326`
const PROJECT_ROOT_MARKS: [&str; 9] = [
    ".project",    // Default project mark for Emacs
    ".projectile", // Default project mark for Projectile
    //
    ".git",      // Git VCS root dir
    ".hg",       // Mercurial VCS root dir
    ".fslckout", // Fossil VCS root dir
    "_FOSSIL_",  // Fossil VCS root DB on Windows
    ".bzr",      // Bazaar VCS root dir
    "_darcs",    // Darcs VCS root dir
    ".pijul",    // Pijul VCS root dir
    ".jj",       // Jujutsu VCS root dir
];

#[must_use]
pub fn find_project_root(path: PathBuf, config: Config) -> Option<PathBuf> {
    let root_pattern = match config.root_pattern {
        Some(pattern) => pattern,
        None => PROJECT_ROOT_MARKS
            .iter()
            .map(|s| (*s).to_string())
            .collect(),
    };

    let mut current_path = Some(path);

    while let Some(ref path_buf) = current_path {
        // Check if any marks exist in the current directory
        if root_pattern
            .iter()
            .any(|file_name| path_buf.join(file_name).exists())
        {
            return current_path;
        }

        // Move to the parent directory
        current_path = path_buf.parent().map(std::path::Path::to_path_buf);
    }

    None
}
