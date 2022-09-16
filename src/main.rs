use std::process::Command;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::env;
use fs_extra::dir::{get_dir_content2, DirOptions};
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
        },
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

        let fetch_output = Command::new("git")
                .arg("fetch")
                .output()
                .unwrap();

        io::stdout().write_all(&fetch_output.stdout).unwrap();

        let status_output = Command::new("git")
                .arg("status")
                .output()
                .unwrap();

        //let status_output_string = format!("{:?}", status_output.stdout);
        let status_output_string = String::from_utf8(status_output.stdout).unwrap();

        println!("{}", status_output_string);

        if status_output_string.contains("Your branch is behind") {
            println!("running git pull at {}", &dir);

            Command::new("git")
                    .arg("pull")
                    .spawn()
                    .unwrap();
        }
    }
}

fn save_work() {
    let dev_folder = env::var("dev_folder").unwrap();

    println!("{}", &dev_folder);

    let mut options = DirOptions::new();
    options.depth = 1;

    let mut dir_content = get_dir_content2(std::path::PathBuf::from(&dev_folder), &options).unwrap();

    dir_content.directories.remove(0);

    println!("{:?}", dir_content.directories);

    let dirs = dir_content.directories;

    for dir in dirs {
        env::set_current_dir(&dir).unwrap();

        println!("Current Directory: {}", &dir);

        if !Path::new(".git/").is_dir() {
            continue;
        }

        Command::new("git")
                .arg("fetch")
                .spawn()
                .unwrap();

        let status_output = Command::new("git")
                .arg("status")
                .output()
                .unwrap();

        let status_output_string = String::from_utf8(status_output.stdout).unwrap();

        if status_output_string.contains("Your branch is up to date") {
            continue;
        } else if status_output_string.contains("No commits yet") {
            Command::new("git")
                    .arg("add")
                    .arg("-A")
                    .spawn()
                    .unwrap();

            Command::new("git")
                    .arg("commit")
                    .arg("-m")
                    .arg("\"initial commit\"")
                    .spawn()
                    .unwrap();

            Command::new("git")
                    .arg("push")
                    .spawn()
                    .unwrap();
        } else if status_output_string.contains("Your branch is behind") && !status_output_string.contains("Changes not staged for commit") && !status_output_string.contains("Untracked files") {
            Command::new("git")
                    .arg("pull")
                    .spawn()
                    .unwrap();
        } else {
            Command::new("git")
                    .arg("pull")
                    .spawn()
                    .unwrap();

            Command::new("git")
                    .arg("add")
                    .arg("-A")
                    .spawn()
                    .unwrap();

            Command::new("git")
                    .arg("commit")
                    .arg("-m")
                    .arg("\"saving work\"")
                    .spawn()
                    .unwrap();

            Command::new("git")
                    .arg("push")
                    .spawn()
                    .unwrap();
        }
    }
}
