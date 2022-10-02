use anyhow::Result;
use chrono::prelude::*;
use fs_extra::dir::{get_dir_content2, DirOptions};
use fs_extra::file::{read_to_string, write_all};
use std::env;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "gitsync")]
enum Opt {
    Pull,
    Push,
}

fn main() {
    match Opt::from_args() {
        Opt::Pull => {
            pull_updates();
        }
        Opt::Push => {
            save_work();
        }
    }
}

fn pull_updates() {
    let dev_folder = env::var("dev_folder").unwrap();

    println!("{}", &dev_folder);

    let mut options = DirOptions::new();
    options.depth = 1;

    let mut dir_content = get_dir_content2(PathBuf::from(&dev_folder), &options).unwrap();

    dir_content.directories.remove(0);

    println!("{:?}", dir_content.directories);

    let dirs = dir_content.directories;

    for dir in dirs {
        env::set_current_dir(&dir).unwrap();

        println!("Current Directory: {}", &dir);

        if !Path::new(".git/").is_dir() {
            continue;
        }

        let fetch_output = Command::new("git").arg("fetch").output().unwrap();

        io::stdout().write_all(&fetch_output.stdout).unwrap();

        let status_output = Command::new("git").arg("status").output().unwrap();

        let status_output_string = String::from_utf8(status_output.stdout).unwrap();

        println!("{}", status_output_string);

        if status_output_string.contains("Your branch is behind") {
            println!("running git pull at {}", &dir);

            let output = Command::new("git").arg("pull").output().unwrap();

            let output_string = String::from_utf8(output.stdout).unwrap();

            log(&format!("gitsync pull - {}\n{}", &dir, output_string));
        }
    }
}

fn save_work() {
    let dev_folder = env::var("dev_folder").unwrap();

    println!("{}", &dev_folder);

    let mut options = DirOptions::new();
    options.depth = 1;

    let mut dir_content =
        get_dir_content2(std::path::PathBuf::from(&dev_folder), &options).unwrap();

    dir_content.directories.remove(0);

    println!("{:?}", dir_content.directories);

    let dirs = dir_content.directories;

    for dir in dirs {
        env::set_current_dir(&dir).unwrap();

        println!("Current Directory: {}", &dir);

        if !Path::new(".git/").is_dir() {
            continue;
        }

        Command::new("git").arg("fetch").spawn().unwrap();

        let status_output = Command::new("git").arg("status").output().unwrap();

        let status_output_string = String::from_utf8(status_output.stdout).unwrap();

        if status_output_string.contains("Your branch is up to date") {
            continue;
        } else if status_output_string.contains("No commits yet") {
            let output = Command::new("git").arg("add").arg("-A").output().unwrap();

            let output_string = String::from_utf8(output.stdout).unwrap();

            log(&format!("gitsync push - {}\n{}", &dir, output_string));

            let output = Command::new("git")
                .arg("commit")
                .arg("-m")
                .arg("\"initial commit\"")
                .output()
                .unwrap();

            let output_string = String::from_utf8(output.stdout).unwrap();

            log(&format!("gitsync push - {}\n{}", &dir, output_string));

            let output = Command::new("git").arg("push").output().unwrap();

            let output_string = String::from_utf8(output.stdout).unwrap();

            log(&format!("gitsync push - {}\n{}", &dir, output_string));
        } else if status_output_string.contains("Your branch is behind")
            && !status_output_string.contains("Changes not staged for commit")
            && !status_output_string.contains("Untracked files")
        {
            let output = Command::new("git").arg("pull").output().unwrap();

            let output_string = String::from_utf8(output.stdout).unwrap();

            log(&format!("gitsync push - {}\n{}", &dir, output_string));
        } else {
            let output = Command::new("git").arg("pull").output().unwrap();

            let output_string = String::from_utf8(output.stdout).unwrap();

            log(&format!("gitsync push - {}\n{}", &dir, output_string));

            let output = Command::new("git").arg("add").arg("-A").output().unwrap();

            let output_string = String::from_utf8(output.stdout).unwrap();

            log(&format!("gitsync push - {}\n{}", &dir, output_string));

            let output = Command::new("git")
                .arg("commit")
                .arg("-m")
                .arg("\"initial commit\"")
                .output()
                .unwrap();

            let output_string = String::from_utf8(output.stdout).unwrap();

            log(&format!("gitsync push - {}\n{}", &dir, output_string));

            let output = Command::new("git").arg("push").output().unwrap();

            let output_string = String::from_utf8(output.stdout).unwrap();

            log(&format!("gitsync push - {}\n{}", &dir, output_string));
        }
    }
}

fn log(message: &str) {
    let tmp_dir = env::var("tmp_folder").unwrap();
    let tmp_log = format!("{}/gitsync_log.txt", tmp_dir);
    let tmp_path = PathBuf::from(&tmp_log);

    let time = Utc::now();

    let content = format!("{} - {}\n", time, message);

    if Path::new(&tmp_log).is_file() {
        let mut file_content = read_to_string(&tmp_path).unwrap();
        file_content.push_str(&content);
        write_all(&tmp_path, &file_content).unwrap();
    } else {
        write_all(tmp_path, &content).unwrap();
    }
}
