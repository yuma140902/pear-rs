use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process;

use manifest::Manifest;

mod manifest;

pub type ExitCode = i32;
pub const SUCCESS: ExitCode = 0;
pub const ERROR_IN_PROGRAM: ExitCode = 1;
pub const ERROR_IN_PACKAGE: ExitCode = 2;

fn get_package(mut package: String) -> Option<Box<PathBuf>> {
    if !package.ends_with(".ppkg") {
        package = package + ".ppkg";
    }
    let path = Path::new("./").join(package);
    if path.exists() {
        Some(Box::new(path))
    } else {
        None
    }
}

pub fn detail(package: &str) -> ExitCode {
    let package_path: Option<_> = get_package(package.to_string());
    if let Some(package_path) = package_path {
        let manifest_path: PathBuf = package_path.join("manifest.json");
        if !manifest_path.exists() {
            eprintln!("エラー: manifest.json が見つかりません。");
            return ERROR_IN_PACKAGE;
        }

        let file: File = File::open(&manifest_path).unwrap_or_else(|err| {
            eprintln!("エラー: '{:?}' を開けませんでした。", manifest_path);
            eprintln!("{}", err);
            process::exit(ERROR_IN_PROGRAM);
        });
        let reader = BufReader::new(file);
        let manifest: Manifest = serde_json::from_reader(reader).unwrap_or_else(|err|{
            eprintln!("エラー: manifest.json の解析中にエラー");
            eprintln!("{}", err);
            process::exit(ERROR_IN_PACKAGE);
        });
        println!("{}", manifest);
        0
    } else {
        eprintln!("エラー: パッケージ '{}' が見つかりません。", package);
        ERROR_IN_PROGRAM
    }
}

pub fn install(package: &str) -> ExitCode {
    0
}

pub fn uninstall(package: &str) -> ExitCode {
    0
}

pub fn create(package: Option<&str>) -> ExitCode {
    0
}
