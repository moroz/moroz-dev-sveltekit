use lazy_static::lazy_static;
use regex::Regex;
use std::{
    env,
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};

/// Returns the first parent directory of the present working directory that contains a `.git`
/// directory, or `None`, if none has been found while traversing the directory tree.
fn find_git_root() -> Option<PathBuf> {
    let mut pwd = env::current_dir().unwrap();
    loop {
        let git_dir = pwd.join(".git");
        if git_dir.exists() && git_dir.is_dir() {
            return Some(pwd);
        }
        if !pwd.pop() {
            return None;
        }
    }
}

/// Run a glob pattern starting from the given base directory, searching for all .mdx files.
fn find_all_mdx_files(base_dir: &PathBuf) -> Vec<PathBuf> {
    let pattern = base_dir.to_str().unwrap();
    let pattern = format!("{}/**/*.mdx", pattern);
    match glob::glob(&pattern) {
        Ok(paths) => {
            return paths.filter_map(Result::ok).collect();
        }
        _ => {
            return vec![];
        }
    }
}

lazy_static! {
    static ref FIGURE_REGEX: Regex = Regex::new(r#"<Figure\s*([^>]*)>(.+)</Figure>"#).unwrap();
}

fn find_all_figures_in_file(path: impl Into<PathBuf>) -> io::Result<Vec<String>> {
    let file = fs::read_to_string(path.into())?;
    let matches: Vec<_> = FIGURE_REGEX
        .find_iter(&file)
        .map(|mat| mat.as_str())
        .collect();
    dbg!(matches);
    Ok(vec![])
}

fn main() -> Result<(), Box<dyn Error>> {
    let git_root = find_git_root().unwrap();
    let files = find_all_mdx_files(&git_root);
    for file in files {
        find_all_figures_in_file(&file)?;
    }
    Ok(())
}
