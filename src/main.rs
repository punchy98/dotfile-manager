#![allow(unused_must_use)]
#![allow(unused_variables)]

//Dotfile Manager
//TODO:
//done - download dotfiles from github repo
//done - create folder for ~/.config if does not exist
//create folders for each config file in .config
//
use clap::Parser;
use git2::Repository;
use std::fs;
use std::env;
use walkdir::{WalkDir, DirEntry};

#[derive(Parser)]
#[derive(Debug)]

struct CliArgs {
    githubuser: String,
    repo: String,
}

fn main() {
    let args = CliArgs::parse();
    cloneurl(args.githubuser,args.repo);
    //change to .config when done testing
    createdirectories("testingrustconfig");
    generateconflist();
}
//clone url from github repo based on user input
fn cloneurl(user: String,repo: String) {
    let fullghurl = ["https://github.com/", &user, "/", &repo].concat();
    let ghrepo = match Repository::clone(&fullghurl,"/home/punchy/projects/dotfiles") {
       Ok(ghrepo) => ghrepo,
       Err(error) => panic!("failed to clone: {:?}", error),
    };
}
//create directories
fn createdirectories(dir: &str) {
    let homedir = env::var("HOME").expect("$HOME` is not set");
    let newdir = [&homedir,"/",&dir].concat();
    fs::create_dir(&newdir);

}
//create list of conf files to symlink
fn generateconflist(){
    let homedir = env::var("HOME").expect("$HOME` is not set");
    let configdir = [&homedir,"/","projects/dotfiles"].concat();
    let walker = WalkDir::new(&configdir).into_iter();
    for entry in walker {//.filter_entry(|e| is_conf(e)) {
        println!("{:?}", entry?.path().display());
}
}
fn is_conf(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.ends_with("conf"))
         .unwrap_or(false)
}


