use lazy_static::lazy_static;
use regex::Regex;
use std::{env, error::Error, fs, io, path::PathBuf};

lazy_static! {
    static ref FIGURE_REGEX: Regex = Regex::new(r#"<Figure\s*([^>]*)>(.+)</Figure>"#).unwrap();
    static ref ATTRIBUTES_REGEX: Regex = Regex::new(r#"([a-zA-Z_]+)="([^"]+)""#).unwrap();
}

#[derive(Default, Clone, Debug)]
struct Figure<'a> {
    src: &'a str,
    dark_src: Option<&'a str>,
    body: &'a str,
    alt: Option<&'a str>,
}

impl<'a> Figure<'a> {
    pub fn new(attrs: &'a str, body: &'a str) -> Figure<'a> {
        let mut res = Self {
            body,
            ..Default::default()
        };

        for cap in ATTRIBUTES_REGEX.captures_iter(attrs) {
            let value = cap.get(2).unwrap().as_str();
            match cap.get(1) {
                Some(field) => match field.as_str() {
                    "darkSrc" => {
                        res.dark_src = Some(value);
                    }
                    "src" => {
                        res.src = value;
                    }
                    "alt" => {
                        res.alt = Some(value);
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        return res;
    }
}

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

fn find_all_figures_in_file(path: impl Into<PathBuf>) -> io::Result<Vec<String>> {
    let file = fs::read_to_string(path.into())?;
    let matches: Vec<_> = FIGURE_REGEX
        .captures_iter(&file)
        .map(|cap| {
            let figure = Figure::new(cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str());
            return figure;
        })
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
