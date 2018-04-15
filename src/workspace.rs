//! # Workspace objects and fuctions
//!
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

extern crate toml;

/// Workspace object which represents [workspace] in .toml file.
///
///
#[derive(Deserialize, Debug)]
pub struct Workspace {
  pub packages: Option<Vec<Package>>,
}

/// Creates new formatted key pair string.
///
/// If value is empty, just return string key without trailing white space.
///
/// # Examples
///
/// ```
/// extern crate lexicon;
/// use lexicon::workspace::format_pair;
///
/// assert_eq!("key:", format_pair("key:", "".to_string()));
/// assert_eq!("key: value", format_pair("key:", "value".to_string()));
/// ```
pub fn format_pair(key: &str, value: String) -> String {
  match value.as_str() {
    "" => key.to_string(),
    _ => vec![key.to_string(), value].join(" "),
  }
}

/// Package object which represents [package] in .toml file.
///
/// workspace has multiple packages.
#[derive(Deserialize, Debug)]
pub struct Package {
  pub id: Option<String>,
  pub name: Option<String>,
  pub version: Option<String>,
  pub repository: Option<String>,
  pub description: Option<String>,
  pub keywords: Option<Vec<String>>,
}

impl Package {
  // TODO: refactor :'(
  fn id(&self) -> String {
    match self.id {
      Some(ref v) => v.to_string(),
      _ => "".to_string(),
    }
  }

  fn name(&self) -> String {
    match self.name {
      Some(ref v) => v.to_string(),
      _ => "".to_string(),
    }
  }

  fn version(&self) -> String {
    match self.version {
      Some(ref v) => v.to_string(),
      _ => "".to_string(),
    }
  }

  fn repository(&self) -> String {
    match self.repository {
      Some(ref v) => v.to_string(),
      _ => "".to_string(),
    }
  }

  fn description(&self) -> String {
    match self.description {
      Some(ref v) => v.to_string(),
      _ => "".to_string(),
    }
  }

  fn keywords(&self) -> String {
    match self.keywords {
      Some(ref v) => v.join(","),
      _ => "".to_string(),
    }
  }

  pub fn to_string(&self) -> String {
    format!(
      r#"{id}
{name}
{version}
{repository}
{description}
{keywords}
"#,
      id = format_pair("id:", self.id()),
      name = format_pair("name:", self.name()),
      version = format_pair("version:", self.version()),
      repository = format_pair("repository:", self.repository()),
      description = format_pair("description:", self.description()),
      keywords = format_pair("keywords:", self.keywords()),
    )
  }
}

pub fn load(filepath: &str) -> Workspace {
  let file: File = File::open(filepath).unwrap();

  let mut buf_reader = BufReader::new(file);
  let mut content = String::new();

  match buf_reader.read_to_string(&mut content) {
    Ok(_) => {
      match toml::from_str(&content) {
        Ok(r) => r,
        Err(e) => panic!("parse error: {}", e),
      }
    },
    Err(e) => panic!("read error: {}", e),
  }
}

impl fmt::Display for Package {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_string())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_format_pair() {
    assert_eq!("key:", format_pair("key:", "".to_string()));
    assert_eq!(
      "key: value",
      format_pair("key:", "value".to_string())
    );
  }

  #[test]
  fn test_package_id() {
    let mut package = Package {
      id: None,
      name: None,
      version: None,
      repository: None,
      description: None,
      keywords: None,
    };
    assert_eq!("", package.id());

    package.id = Some("hoi".to_string());
    assert_eq!("hoi", package.id());
  }

  #[test]
  fn test_to_string() {
    let mut package = Package {
      id: None,
      name: None,
      version: None,
      repository: None,
      description: None,
      keywords: None,
    };
    assert_eq!(
      r#"id:
name:
version:
repository:
description:
keywords:
"#,
      package.to_string()
    );

    package.name = Some("Hoi Zäme!".to_string());
    assert_eq!(
      r#"id:
name: Hoi Zäme!
version:
repository:
description:
keywords:
"#,
      package.to_string()
    )
  }

  #[test]
  fn test_display() {
    let package = Package {
      id: Some("hoi".to_string()),
      name: Some("Hoi Zäme!".to_string()),
      version: Some("0.0.1rc-2".to_string()),
      repository: Some("https://gitlab.com/grauwoelfchen/lexicon".to_string()),
      description: Some("Hoi us Züri!".to_string()),
      keywords: Some(vec![
        "Liäbi".to_string(),
        "Grüäss".to_string(),
      ]),
    };
    assert_eq!(package.to_string(), format!("{}", package));
  }
}
