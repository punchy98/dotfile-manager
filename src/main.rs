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
    let homedir = env::var("HOME").expect("$HOME` is not set");
    //change to ~/dotfiles after testing
    let dotfiledir = [&homedir,"/projects/dotfiles"].concat();
    println!("Cloning into {}",dotfiledir);
    cloneurl(&dotfiledir,args.githubuser,args.repo);
    //change to .config when done testing
    createdirectories(&homedir,"testingrustconfig");
    generateconflist(&homedir);
}
//clone url from github repo based on user input
fn cloneurl(dotspath: &str,user: String,repo: String) {
    let fullghurl = ["https://github.com/", &user, "/", &repo].concat();
    let ghrepo = match Repository::clone(&fullghurl,&dotspath) {
       Ok(ghrepo) => ghrepo,
       Err(error) => panic!("failed to clone: {:?}", error),
    };
}
//create directories
fn createdirectories(homedir: &str,dir: &str) {
    let newdir = [&homedir,"/",&dir].concat();
    fs::create_dir(&newdir);

}
//create list of conf files to symlink
fn generateconflist(homedir: &str) {
    //change to be ~/dotfiles after testing
    let configdir = [&homedir,"/projects/dotfiles"].concat();
    for entry in WalkDir::new(&configdir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {
        let f_name = entry.file_name().to_string_lossy();
        if f_name.ends_with(".conf") {
            println!("{}", f_name);
        }
    }
}   


