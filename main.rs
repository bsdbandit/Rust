//RustyBox is just a tool grabber it grabs my favorite tools from gitub and installs each tool
//written by bsdbandit
//March 19, 2021


//Crates 
use git2::Repository;
use std::fs::{self, DirBuilder};
use figlet_rs::FIGfont;
use std::process::Command;

fn main() {
    let standard_font = FIGfont::standand().unwrap();
    let figure = standard_font.convert("Rusty Box");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    apt_update();
    pip3_install();
    sec_lists();
    mitm6_github();
    impacket_github();
    dirsearch_github();
    crackmap_github();
    mitm6_install(); 
    dirsearch_install(); 
    impacket_install(); 
    crackmap_install(); 
}

//updating Kali Linux 
fn apt_update(){
    println!("Updating Kali Linux");
Command::new("apt")
    .arg("update")
    .arg("&&")
    .arg("upgrade")
    .arg("y")
    .spawn()
    .expect("apt failed to execute process");
}

//Installing Pip3 
fn pip3_install(){
Command::new("apt")
    .arg("install")
    .arg("python3-pip")
    .arg("-y")
    .spawn()
    .expect("apt failed to execute process");
}

//Grabbing tools from github 
fn sec_lists() {
    let path = "/home/bsdbandit/SecLists";
    DirBuilder::new()
        .recursive(true)
        .create(path).unwrap();
    assert!(fs::metadata(path).unwrap().is_dir());
    let url = "https://github.com/danielmiessler/SecLists.git";
    let _repo = match Repository::clone(url, "/home/bsdbandit/SecLists"){
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),

};
}

fn mitm6_github() { 
    let path = "/home/bsdbandit/mitm6";
    DirBuilder::new()
        .recursive(true)
        .create(path).unwrap();
    assert!(fs::metadata(path).unwrap().is_dir());
    let url = "https://github.com/fox-it/mitm6.git";
    let _repo = match Repository::clone(url, "/home/bsdbandit/mitm6"){
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
};
}

fn impacket_github() { 
    let path = "/home/bsdbandit/impacket";
    DirBuilder::new()
        .recursive(true)
        .create(path).unwrap();
    assert!(fs::metadata(path).unwrap().is_dir());
    let url ="https://github.com/SecureAuthCorp/impacket.git";
    let _repo = match Repository::clone(url, "/home/bsdbandit/impacket"){
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
};
}

fn dirsearch_github() { 
    let path = "/home/bsdbandit/dirsearch";
    DirBuilder::new()
        .recursive(true)
        .create(path).unwrap();
    assert!(fs::metadata(path).unwrap().is_dir());
    let url ="https://github.com/maurosoria/dirsearch.git";
    let _repo = match Repository::clone(url, "/home/bsdbandit/dirsearch"){
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
};
}

fn crackmap_github() { 
    let path = "/home/bsdbandit/CrackMapExec";
    DirBuilder::new()
        .recursive(true)
        .create(path).unwrap();
    assert!(fs::metadata(path).unwrap().is_dir());
    let url ="https://github.com/byt3bl33d3r/CrackMapExec.git";
    let _repo = match Repository::clone(url, "/home/bsdbandit/CrackMapExec"){
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
};
}

//Installing Tools from Github 
fn mitm6_install() { 

Command::new("pip3")
            .arg("install")
            .arg("-r")
            .arg("/home/bsdbandit/mitm6/requirements.txt")
            .spawn()
            .expect("pip3 command failed to execute");
}

fn dirsearch_install() { 
Command::new("pip3")
            .arg("install")
            .arg("-r")
            .arg("/home/bsdbandit/dirsearch/requirements.txt")
            .spawn()
            .expect("pip3 command failed to execute");
}

fn impacket_install() { 

Command::new("pip3")
            .arg("install")
            .arg("-r")
            .arg("/home/bsdbandit/impacket/requirements.txt")
            .spawn()
            .expect("pip3 command failed to execute");
}
fn crackmap_install() { 

Command::new("pip3")
            .arg("install")
            .arg("-r")
            .arg("/home/bsdbandit/CrackMapExec/requirements.txt")
            .spawn()
            .expect("pip3 command failed to execute");
}
