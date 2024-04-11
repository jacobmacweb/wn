/*
 * This deals with the files in the repository. It does things like:
 * - Create a new repository
 * - Get the root directory of the repository
 * - Get the path to the .wnignore file
 * - Get all valid files in the repository
 */

use std::{io::BufRead, path::PathBuf, vec};

use globset::{Glob, GlobSetBuilder};
use log::{debug, error, info, trace};

use crate::{objects, refs};

pub fn create_empty(dir: Option<&PathBuf>, _branch: &Option<String>) {
    // Create empty directories
    let root = resolve_dir(dir);
    match std::fs::create_dir(&root) {
        Ok(_) => {
            info!("Created empty repository at {:?}", root);
            objects::create_empty(dir);
            refs::create_empty(dir);
        }
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::AlreadyExists => {
                    error!("Repository already exists at {:?}", root);
                }
                _ => {
                    error!("Failed to create directory at {:?}: {:?}", root, e);
                }
            }

            std::process::exit(1);
        }
    }
}

pub fn root_dir() -> PathBuf {
    let mut cwd = std::env::current_dir().unwrap();
    while !cwd.join(".wn").exists() {
        trace!("checking directory for .wn: {:?}", cwd);
        cwd = cwd
            .parent()
            .unwrap_or_else(|| {
                error!("This is not a wn repository");
                std::process::exit(1);
            })
            .to_path_buf();
    }
    trace!("Root directory is {:?}", cwd.join(".wn"));

    cwd
}

pub fn resolve_dir(dir: Option<&PathBuf>) -> PathBuf {
    let mut cwd = std::env::current_dir().unwrap();
    if let Some(d) = dir {
        cwd = cwd.join(d);
    }
    cwd = cwd.join(".wn");
    trace!("Resolved directory is {:?}", cwd);

    cwd
}

pub fn wnignore_path() -> PathBuf {
    let mut cwd = root_dir();
    cwd = cwd.join(".wnignore");
    trace!("wnignore file is {:?}", cwd);

    cwd
}

pub fn wnignore() -> Vec<String> {
    let path = wnignore_path();
    let mut ignore = Vec::new();
    if path.exists() {
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        for line in reader.lines() {
            ignore.push(line.unwrap());
        }
    }
    trace!("wnignore: {:?}", ignore);

    ignore
}

pub fn get_files(glob: Option<String>) -> Vec<String> {
    // build the globs from the optional glob or the wnignore file and the default globs
    let wnignore_globs = wnignore();

    let mut positive_globs = vec![];
    let mut negative_globs = vec![".wn/**/*".to_string()];
    for line in wnignore_globs.clone() {
        if line.starts_with("!") {
            positive_globs.push(line[1..].to_string());
        } else {
            negative_globs.push(line);
        }
    }
    debug!("positive globs: {:?}", positive_globs);
    debug!("negative globs: {:?}", negative_globs);

    // get the root directory
    let root = root_dir();

    // get all the files in the repository
    let mut positive_builder = GlobSetBuilder::new();
    for glob in positive_globs {
        positive_builder.add(Glob::new(&glob).unwrap());
    }
    let positive_set = positive_builder.build().unwrap();

    let mut negative_builder = GlobSetBuilder::new();
    for glob in negative_globs {
        negative_builder.add(Glob::new(&glob).unwrap());
    }
    let negative_set = negative_builder.build().unwrap();

    let pattern = match glob {
        Some(g) => g,
        None => "**/*".to_string(),
    };
    let current_glob = Glob::new(&pattern).unwrap().compile_matcher();

    let mut files = Vec::new();
    for entry in walkdir::WalkDir::new(&root)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let path = entry.path().strip_prefix(&root).unwrap();
            // if fits all requirements
            if (!negative_set.is_match(path) || positive_set.is_match(path))
                && current_glob.is_match(path)
            {
                files.push(path.to_str().unwrap().to_string());
            }
        }
    }

    files
}
