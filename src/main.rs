#[macro_use]
extern crate clap;

use std::process;

use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    let detail_subcommand = SubCommand::with_name("detail")
        .about("パッケージの詳細を表示")
        .arg(Arg::with_name("PACKAGE"))
        .setting(AppSettings::ArgRequiredElseHelp);

    let install_subcommand = SubCommand::with_name("install")
        .about("パッケージをインストール")
        .arg(Arg::with_name("PACKAGE"))
        .setting(AppSettings::ArgRequiredElseHelp);

    let uninstall_subcommand = SubCommand::with_name("uninstall")
        .about("パッケージをアンインストール")
        .arg(Arg::with_name("PACKAGE"))
        .setting(AppSettings::ArgRequiredElseHelp);

    let create_subcommand = SubCommand::with_name("create")
        .about("パッケージの雛形を作成する")
        .arg(Arg::with_name("PACKAGE"));

    let app = App::new("Pear-rs")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .settings(&[
            AppSettings::ColoredHelp,
            AppSettings::VersionlessSubcommands,
            AppSettings::SubcommandRequiredElseHelp,
        ])
        .subcommand(detail_subcommand)
        .subcommand(install_subcommand)
        .subcommand(uninstall_subcommand)
        .subcommand(create_subcommand);

    let matches = app.get_matches();

    if let Some(matches) = matches.subcommand_matches("detail") {
        if let Some(package) = matches.value_of("PACKAGE") {
            let exitcode = pear::detail(package);
            process::exit(exitcode);
        }
    }

    if let Some(matches) = matches.subcommand_matches("install") {
        if let Some(package) = matches.value_of("PACKAGE") {
            let exitcode = pear::install(package);
            process::exit(exitcode);
        }
    }

    if let Some(matches) = matches.subcommand_matches("uninstall") {
        if let Some(package) = matches.value_of("PACKAGE") {
            let exitcode = pear::uninstall(package);
            process::exit(exitcode);
        }
    }

    if let Some(matches) = matches.subcommand_matches("create") {
        let exitcode = pear::create(matches.value_of("PACKAGE"));
        process::exit(exitcode);
    }
}
