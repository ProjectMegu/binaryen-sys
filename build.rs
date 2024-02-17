use std::{path::{Path, PathBuf}, process::Command};

const BINARYEN_URL: &str = "https://github.com/WebAssembly/binaryen.git";
const CMAKE_ARGS: [&str;2] = ["-DBUILD_TESTS=OFF", "-DBUILD_TOOLS=OFF"];

fn main() {
    if std::env::var("DOCS_RS").is_err() {
        println!("cargo:rerun-if-changed=build.rs");
        // println!("cargo:rerun-if-changed=Cargo.toml");
    
        let out_dir = std::env::var("OUT_DIR").unwrap();
    
        let blib_path: PathBuf;
        let binclude_path: PathBuf;

        if let Ok(bpath) = std::env::var("BINARYEN_PATH") {
            blib_path = Path::new(&bpath).join("lib");
            binclude_path = Path::new(&bpath).join("include");
        } else if let Some(bpath) = serch_binaryen() {
            blib_path = bpath.join("lib");
            binclude_path = bpath.join("include");
        } else {
            let (lib_path, include_path) = build_binaryen(&out_dir);
            blib_path = lib_path;
            binclude_path = include_path;
        }

        println!("cargo:rustc-link-search={}", blib_path.to_str().unwrap());
        println!("cargo:rustc-link-lib=binaryen");

        // bindgen
        let bindings = bindgen::Builder::default()
            .header(
                binclude_path.join("binaryen-c.h").to_str().unwrap()
            )
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate()
            .expect("Unable to generate bindings");

        bindings.write_to_file(Path::new(&out_dir).join("bindings.rs"))
            .expect("Couldn't write bindings!");
    } else {
        let binding = bindgen::Builder::default()
            .header("libs/include/binaryen-c.h")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate()
            .expect("Unable to generate bindings");
        
        binding.write_to_file(Path::new(&std::env::var("OUT_DIR").unwrap()).join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}

fn build_binaryen(out_dir: &String) -> (PathBuf, PathBuf){
    let _ = Command::new("rm")
        .args(["-rf", Path::new(out_dir).join("binaryen").to_str().unwrap()])
        .output();

    // git clone
    let clone = Command::new("git")
        .args(["clone", BINARYEN_URL])
        .current_dir(out_dir)
        .output();

    if let Err(err) = clone {
        panic!("Failed to clone binaryen: {}\n(need git)", err);
    }

    let check_ninja = Command::new("ninja")
        .arg("--version")
        .output();

    // build binaryen
    // ninja check
    if let Ok(_) = check_ninja {
        // cmake
        let cmake = Command::new("cmake")
            .args(CMAKE_ARGS)
            .args(["-G", "Ninja"])
            .arg(".")
            .current_dir(Path::new(out_dir).join("binaryen"))
            .output();

        if let Err(err) = cmake {
            panic!("Failed to cmake binaryen: {}\n(need cmake)", err);
        }

        // ninja
        let ninja = Command::new("ninja")
            .current_dir(Path::new(out_dir).join("binaryen"))
            .output();

        if let Err(err) = ninja {
            panic!("Failed to ninja binaryen: {}", err);
        }
    } else {
        // cmake
        let cmake = Command::new("cmake")
            .args(CMAKE_ARGS)
            .arg(".")
            .current_dir(Path::new(out_dir).join("binaryen"))
            .output();

        if let Err(err) = cmake {
            panic!("Failed to cmake binaryen: {}\n(need cmake)", err);
        }

        // make
        let make = Command::new("make")
            .current_dir(Path::new(out_dir).join("binaryen"))
            .output();

        if let Err(err) = make {
            panic!("Failed to make binaryen: {}\n(need make or ninja)", err);
        }
    }

    (Path::new(out_dir).join("binaryen").join("lib"), Path::new(out_dir).join("binaryen").join("src"))
}

fn serch_binaryen() -> Option<PathBuf> {
    let os = std::env::var_os("CARGO_CFG_TARGET_OS");

    if let Some(os) = os {
        if os == "windows" {
            let where_ = Command::new("where.exe")
                .arg("wasm-opt")
                .output();
            match where_ {
                Err(_) => None,
                Ok(out) => {
                    if out.status.code().unwrap() != 0 {
                        return None;
                    }
                    
                    let w = std::str::from_utf8(&out.stdout).unwrap();
                    let bin_path = w.split("\n").nth(0).unwrap();
                    Some(Path::new(bin_path).ancestors().nth(2).unwrap().into())
                }
            }
        } else {
            let where_ = Command::new("which")
                .arg("wasm-opt")
                .output();
            match where_ {
                Err(_) => None,
                Ok(out) => {
                    if out.status.code().unwrap() != 0 {
                        return None;
                    }
                    
                    let w = std::str::from_utf8(&out.stdout).unwrap();
                    let bin_path = w.split("\n").nth(0).unwrap();
                    Some(Path::new(bin_path).ancestors().nth(2).unwrap().into())
                }
            }
        }
    } else {
        None
    }
}