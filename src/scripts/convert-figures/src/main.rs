use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::{borrow::Cow, env, error::Error, fs, io, path::PathBuf};

lazy_static! {
    static ref FIGURE_REGEX: Regex = Regex::new(r#"(?s)<Figure\s*([^>]*)>(.+)</Figure>"#).unwrap();
    static ref ATTRIBUTES_REGEX: Regex = Regex::new(r#"([a-zA-Z_]+)="([^"]+)""#).unwrap();
    static ref IMPORT_REGEX: Regex = Regex::new(r#"import Figure from "[^"]+";?\n*"#).unwrap();
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

impl Display for Figure<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        // <figure>
        // <a href={src} title="Click to enlarge" target="_blank">
        //   <picture>
        //     <source
        //       srcset={src}
        //       media={darkSrc ? "(prefers-color-scheme: light)" : undefined}
        //     />
        //     {
        //       darkSrc ? (
        //         <source srcset={darkSrc} media={"(prefers-color-scheme: dark)"} />
        //       ) : null
        //     }
        //     <img src={src} alt={alt} />
        //   </picture>
        // </a>
        // <figcaption>
        //   <slot />
        // </figcaption>
        // </figure>

        writeln!(f, "<figure>")?;
        writeln!(
            f,
            r#"  <a href="{}" title="Click to enlarge" target="_blank">"#,
            self.src
        )?;
        writeln!(f, "    <picture>")?;

        if self.dark_src.is_some() {
            writeln!(
                f,
                r#"      <source srcset="{}" media="(prefers-color-scheme: light)" />"#,
                self.src
            )?;
            writeln!(
                f,
                r#"      <source srcset="{}" media="(prefers-color-scheme: dark)" />"#,
                self.dark_src.unwrap()
            )?;
        }

        // <img tag>
        write!(f, r#"      <img src="{}""#, self.src)?;
        match self.alt {
            Some(alt) => {
                writeln!(f, " alt=\"{}\" />", alt)?;
            }
            _ => {
                writeln!(f, " />")?;
            }
        }

        writeln!(f, "    </picture>\n  </a>")?;
        writeln!(f, "  <figcaption>{}</figcaption>", self.body)?;
        write!(f, "</figure>")?;
        Ok(())
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

fn build_out_filename(original_path: &PathBuf) -> PathBuf {
    let file_name = original_path.file_name().unwrap().to_str().unwrap();
    let mut res = original_path.clone();
    let file_name = file_name.replace(".mdx", ".md");
    res.set_file_name(file_name);

    return res;
}

fn replace_figures<'a>(string: impl Into<&'a str>) -> Cow<'a, str> {
    FIGURE_REGEX.replace_all(string.into(), |caps: &regex::Captures| {
        let figure = Figure::new(&caps[1], &caps[2]);
        format!("{}", figure)
    })
}

fn remove_import<'a>(string: impl Into<&'a str>) -> Cow<'a, str> {
    IMPORT_REGEX.replace_all(string.into(), "")
}

fn find_all_figures_in_string(string: &str) -> io::Result<Vec<String>> {
    let matches: Vec<_> = FIGURE_REGEX
        .captures_iter(string)
        .map(|caps: regex::Captures| {
            let figure = Figure::new(
                &caps.get(1).unwrap().as_str(),
                &caps.get(2).unwrap().as_str(),
            );
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
        let outfile = build_out_filename(&file);
        let string: String = fs::read_to_string(file)?;
        let no_figures = replace_figures(&*string).to_string();
        let replaced = remove_import(&*no_figures);
        fs::write(outfile, replaced.to_string())?;
    }
    Ok(())
}
