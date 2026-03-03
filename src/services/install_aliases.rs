use std::io::Write;
use std::fmt::{Display, Formatter};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};
use std::path::Path;
use regex::Regex;
use crate::services::shell::get_terminal_cfg_path;

pub struct Alias {
    pub title: String,
    pub execution: String,
}

impl Display for Alias {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "alias {}='{}'", self.title, self.execution)
    }
}

pub fn install_aliases(alias: Alias) -> Result<bool, std::io::Error> {
    let terminal_root_path = get_terminal_cfg_path();

    let path = Path::new(terminal_root_path.as_str());
    if path.exists() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            if let Some(parsed_name) = parse_alias_name(&line) {
                if parsed_name == alias.title {
                    return Ok(false);
                }
            }
        }

        let mut file = OpenOptions::new()
            .append(true)
            .write(true)
            .open(path)?;
        writeln!(file, "{}", alias)?;

        Ok(true)
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Configuration file not found"))
    }
}

fn parse_alias_name(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }
    let re = Regex::new(r"^alias\s+([\w-]+)\s*=").unwrap();

    if let Some(caps) = re.captures(trimmed) {
        return caps.get(1).map(|m| m.as_str().to_string());
    }
    None
}