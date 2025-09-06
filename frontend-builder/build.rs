use std::process::Command;
use std::path::Path;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=../pinochle-frontend/src");
    println!("cargo:rerun-if-changed=../pinochle-frontend/public");
    println!("cargo:rerun-if-changed=../pinochle-frontend/package.json");
    println!("cargo:rerun-if-changed=../pinochle-frontend/package-lock.json");

    let frontend_dir = Path::new("../pinochle-frontend");
    
    // Check if frontend directory exists
    if !frontend_dir.exists() {
        panic!("Frontend directory not found at ../pinochle-frontend");
    }

    // Install dependencies - handle npm bug with optional dependencies
    println!("Installing frontend dependencies...");
    
    // Always remove node_modules and package-lock.json first to handle npm optional dependencies bug
    let node_modules = frontend_dir.join("node_modules");
    let package_lock = frontend_dir.join("package-lock.json");
    
    let _ = std::fs::remove_dir_all(&node_modules);
    let _ = std::fs::remove_file(&package_lock);
    
    // Use npm install instead of npm ci to avoid optional dependencies bug
    let status = Command::new("npm")
        .args(&["install"])
        .current_dir(frontend_dir)
        .status()
        .expect("Failed to run npm install - make sure Node.js is installed");
        
    if !status.success() {
        panic!("npm install failed");
    }

    // Build the React app (skip TypeScript checks for quick deployment)
    println!("Building React frontend with Vite only (skipping TS checks)...");
    let status = Command::new("npx")
        .args(&["vite", "build", "--mode", "production"])
        .current_dir(frontend_dir)
        .status()
        .expect("Failed to run vite build");
    
    if !status.success() {
        panic!("Frontend build failed");
    }

    // Copy build output to target directory for easy access
    let out_dir = env::var("OUT_DIR").unwrap();
    let build_src = frontend_dir.join("dist"); // Vite outputs to "dist", not "build"
    let build_dst = Path::new(&out_dir).join("../../../frontend-build");
    
    if build_src.exists() {
        // Remove old build
        if build_dst.exists() {
            std::fs::remove_dir_all(&build_dst).ok();
        }
        
        // Copy new build
        copy_dir(&build_src, &build_dst).expect("Failed to copy build output");
        println!("Frontend built and copied to {:?}", build_dst);
    }
}

fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if src_path.is_dir() {
            copy_dir(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}