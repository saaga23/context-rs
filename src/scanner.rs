use std::fs;
use std::path::{Path, PathBuf};
use syn::Item;

// THE FIX: Tell Rust to look for utils.rs in the same folder
#[path = "utils.rs"] 
mod utils; 

/// Scans a Rust file to find all modules it imports (e.g., "mod rag;")
pub fn find_dependencies(path: &Path) -> Vec<PathBuf> {
    let mut deps = Vec::new();
    
    // 1. Read the file
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return deps, 
    };

    // 2. Parse it into an Abstract Syntax Tree (AST)
    let syntax = match syn::parse_file(&content) {
        Ok(ast) => ast,
        Err(_) => return deps, 
    };

    // 3. Look for "mod name;" statements
    for item in syntax.items {
        if let Item::Mod(item_mod) = item {
            let mod_name = item_mod.ident.to_string();
            
            // Construct the likely path: ./mod_name.rs
            if let Some(parent) = path.parent() {
                let dep_path = parent.join(format!("{}.rs", mod_name));
                if dep_path.exists() {
                    deps.push(dep_path);
                }
                // Also check for ./mod_name/mod.rs (older Rust style)
                let folder_path = parent.join(&mod_name).join("mod.rs");
                if folder_path.exists() {
                    deps.push(folder_path);
                }
            }
        }
    }
    
    deps
}