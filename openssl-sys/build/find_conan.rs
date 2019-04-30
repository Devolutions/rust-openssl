
use std::env;
use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use std::process::Command;

extern crate serde_json;
use self::serde_json::Value;

pub fn get_openssl(_target: &str) -> (PathBuf, PathBuf) {
    let profile = env::var("PROFILE").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let build_type = if profile == "debug" { "Debug" } else { "Release" };
    let mut conan_file = manifest_dir.to_path_buf();
    conan_file.push("build");
    conan_file.push("conanfile.txt");

    Command::new("conan")
        .arg("install")
        .arg("-pr")
        .arg(format!("{}-{}", &target_os, &target_arch))
        .arg("-s")
        .arg(format!("build_type={}", &build_type))
        .arg("-if")
        .arg(&out_dir)
        .arg(&conan_file.to_str().unwrap())
        .output()
        .expect("failed to execute conan");

    let mut conan_build_info = out_dir.clone();
    conan_build_info.push("conanbuildinfo.cargo");
    let mut file = File::open(&conan_build_info).expect("Error opening conanbuildinfo.cargo");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read conanbuildinfo.cargo");
    println!("{}", contents);

    let mut conan_build_info = out_dir.clone();
    conan_build_info.push("conanbuildinfo.json");
    let mut file = File::open(&conan_build_info).expect("Error opening conanbuildinfo.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read conanbuildinfo.json");

    let root_value: Value = serde_json::from_str(&contents).expect("Unable to parse conanbuildinfo.json");
    let dependencies = root_value["dependencies"].as_array().unwrap();

    let mut openssl_lib_dir: String = String::from("");
    let mut openssl_inc_dir: String = String::from("");

    for value in dependencies.iter() {
    	if value["name"] == "openssl" {
    		openssl_lib_dir = value["lib_paths"][0].as_str().unwrap().to_string();
    		openssl_inc_dir = value["include_paths"][0].as_str().unwrap().to_string();
    	}
    }

    (
        PathBuf::from(openssl_lib_dir),
        PathBuf::from(openssl_inc_dir),
    )
}
