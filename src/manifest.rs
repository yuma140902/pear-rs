use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    uuid: String,
    name: String,
    description: Option<String>,
    authors: Option<Vec<String>>,
    version: Option<String>,
    platforms: Vec<String>,
    installer: String,
    uninstaller: Option<String>,
}

impl Display for Manifest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} (UUID: {})", self.name, self.uuid)?;
        if let Some(version) = &self.version {
            writeln!(f, "v{}", version)?;
        }
        if let Some(authors) = &self.authors {
            if authors.len() == 1 {
                writeln!(f, "by {}", authors[0])?;
            } else if authors.len() >= 2 {
                writeln!(f, "by {:?}", authors)?;
            } else {
                // Do nothing when authros.len() == 0
            }
        }
        writeln!(f, "made for {:?}", self.platforms)?;
        writeln!(f, "installer: {}", self.installer)?;
        if let Some(uninstaller) = &self.uninstaller {
            writeln!(f, "uninstaller: {}", uninstaller)?;
        } else {
            writeln!(f, "uninstaller is not provided")?;
        }

        Ok(())
    }
}
