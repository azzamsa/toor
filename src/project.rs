use std::path::PathBuf;

/// A list of files considered to mark the root of a project.
/// https://github.com/bbatsov/projectile/blob/71f18add5e66201c3ea7c9650b848968db3aec42/projectile.el#L326
/// A list of files considered to mark the root of a project.
/// https://github.com/bbatsov/projectile/blob/71f18add5e66201c3ea7c9650b848968db3aec42/projectile.el#L326
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
];

pub fn find_project_root(mut path: PathBuf) -> Option<PathBuf> {
    loop {
        // Check if any marks exist in the current directory
        if PROJECT_ROOT_MARKS
            .iter()
            .any(|file_name| path.join(file_name).exists())
        {
            return Some(path);
        }

        // Move to the parent directory
        if !path.pop() {
            // If pop fails, we've reached the root directory
            break;
        }
    }
    None
}
