use std::error::Error;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::io;

use convert_case::Casing;

const OUTPUT_DIR: &str = "output";

pub struct Project {
    pub name: String,
    pub path: PathBuf,
}

pub fn create_project(project_name: String) -> Project {
    let project_name = project_name.to_case(convert_case::Case::Snake);
    create_output_dir().expect("could not create output directory");
    let path = create_project_dir(project_name.clone());
    Project { name: project_name, path }
}

pub fn run_program(project: &Project) {
    let output = Command::new("cargo")
        .current_dir(&project.path)
        .arg("build")
        .arg("--release")
        .arg("--color=always")
        .output().expect("could not build project");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Error compiling program: {}", stderr);
    }

    let exe_path = project.path.join("target").join("release").join(&project.name);
    let output = Command::new(exe_path)
        .output().expect("could not run project");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Error running program: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);
}

fn get_project_name() -> String {
    println!("Project Name:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().to_string()
}


fn create_output_dir() -> Result<(), Box<dyn Error>> {
    let output_dir = Path::new(OUTPUT_DIR);
    if !output_dir.exists() {
        fs::create_dir(output_dir)?;
    }
    Ok(())
}

fn create_project_dir(project_name: String) -> PathBuf {
    let output_dir = Path::new(OUTPUT_DIR);
    let output = Command::new("cargo")
        .current_dir(output_dir)
        .arg("new")
        .arg("--bin")
        .arg(&project_name)
        .arg("--color=always")
        .output()
        .expect("Error creating project directory");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Error creating project: {}", stderr);
    }

    Path::new(output_dir).join(project_name)
}

pub fn write_to_program(file_contents: String, path: &PathBuf) {
    let src_path = path.join("src").join("main.rs");
    let mut file = File::create(&src_path).expect("could not create file");
    file.write_all(file_contents.as_bytes()).expect("could not write to file");
}



pub fn delete_folder(path: &PathBuf) {
    fs::remove_dir_all(path).expect("could not delete folder");
}
