use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let target_dir = PathBuf::from(out_dir).join("..").join("..").join("..").join("..").canonicalize().expect("Failed to canonicalize path");

    // Path to the SDL2.dll file relative to the project root
    let sdl2_dll = "src/SDL2.dll";

    // Copy SDL2.dll to the debug directory
    let debug_dir = target_dir.join("debug");
    fs::create_dir_all(&debug_dir).expect("Failed to create debug directory");
    fs::copy(sdl2_dll, debug_dir.join("SDL2.dll")).expect("Failed to copy SDL2.dll to debug directory");

    // Copy SDL2.dll to the release directory
    let release_dir = target_dir.join("release");
    fs::create_dir_all(&release_dir).expect("Failed to create release directory");
    fs::copy(sdl2_dll, release_dir.join("SDL2.dll")).expect("Failed to copy SDL2.dll to release directory");
}
