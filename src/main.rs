use clap::Parser;
use ignore::WalkBuilder;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use anyhow::Result;
use arboard::Clipboard;
use colored::*;
use std::collections::HashSet;

mod scanner;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value = ".")]
    path: PathBuf,
    #[arg(long)]
    map: bool,
    #[arg(long, short)]
    smart: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // UI Header
    println!("\n{}", "🚀 context-rs: The Intelligent Context Agent".bold().cyan());
    println!("{}", "============================================".dimmed());

    let mut target_files: Vec<PathBuf> = Vec::new();
    let mut dependency_map: Vec<(String, String)> = Vec::new(); 
    let mut logic_log: Vec<String> = Vec::new(); 

    // --- PHASE 1: GIT DETECTION (The User's Work) ---
    if args.smart {
        println!("{}", "🧠 STEP 1: Querying Git for user activity...".yellow().bold());
        let output = Command::new("git").args(["diff", "--name-only", "HEAD"]).output();

        if let Ok(out) = output {
             let git_out = String::from_utf8_lossy(&out.stdout);
             
             if !git_out.trim().is_empty() {
                 for line in git_out.lines() {
                     target_files.push(PathBuf::from(line));
                     println!("   • [USER TOUCHED] {}", line.green().bold());
                     logic_log.push(format!("User actively modified <b>{}</b>.", line));
                 }
             } else {
                 let untracked = Command::new("git").args(["ls-files", "--others", "--exclude-standard"]).output();
                 if let Ok(u_out) = untracked {
                     for line in String::from_utf8_lossy(&u_out.stdout).lines() {
                        if !line.trim().is_empty() {
                            target_files.push(PathBuf::from(line));
                             println!("   • [USER CREATED] {}", line.green().bold());
                             logic_log.push(format!("User created new file <b>{}</b>.", line));
                        }
                     }
                 }
             }
        }
    } else {
        let walker = WalkBuilder::new(&args.path).hidden(false).git_ignore(true).build();
        for res in walker { if let Ok(e) = res { if e.path().is_file() { target_files.push(e.path().into()); } } }
    }

    // --- PHASE 2: DEEP RECURSIVE AST RESOLUTION (The AI's Work) ---
    if args.smart && !target_files.is_empty() {
        println!("\n{}", "🔬 STEP 2: Deep AST Analysis (Compiler Scan)...".yellow().bold());
        let mut queue = target_files.clone();
        let mut scanned = HashSet::new();
        for f in &target_files { scanned.insert(f.clone()); }

        while let Some(current_file) = queue.pop() {
            let deps = scanner::find_dependencies(&current_file);
            for dep in deps {
                let parent_name = current_file.file_name().unwrap().to_string_lossy().to_string();
                let child_name = dep.file_name().unwrap().to_string_lossy().to_string();
                
                if parent_name != child_name && !dependency_map.contains(&(parent_name.clone(), child_name.clone())) {
                    dependency_map.push((parent_name.clone(), child_name.clone()));
                }

                if !scanned.contains(&dep) {
                    target_files.push(dep.clone());
                    queue.push(dep.clone());
                    scanned.insert(dep.clone());
                    
                    println!("   • [AUTO-RESOLVED] {} {} {}", child_name.cyan(), "found in".dimmed(), parent_name.green());
                    logic_log.push(format!("Compiler found dependency <code>mod {}</code> inside <b>{}</b>.", child_name.replace(".rs",""), parent_name));
                }
            }
        }
    }

    // --- PHASE 3: PACKING & HTML GENERATION ---
    let mut output_buffer = String::new();
    let mut packed_count = 0;
    target_files.sort(); target_files.dedup();

    output_buffer.push_str("<context_rs_packet>\n");
    
    // System Instructions
    output_buffer.push_str("  <system_instructions>\n");
    output_buffer.push_str("    You are a Senior Rust Expert. The user has modified specific 'Seed' files.\n");
    output_buffer.push_str("    We have also auto-packed the complete dependency graph for your reference.\n");
    output_buffer.push_str("    \n");
    output_buffer.push_str("    PROTOCOL:\n");
    output_buffer.push_str("    1. Analyze the changes in the user-modified files first.\n");
    output_buffer.push_str("    2. Review the auto-resolved dependencies (scanner.rs, etc.) for context.\n");
    output_buffer.push_str("    3. IF definitions in dependencies must change to support the Seed, provide the updated code.\n");
    output_buffer.push_str("    4. IF no changes are needed in dependencies, explicitly state: 'No changes required in dependency graph'.\n");
    output_buffer.push_str("  </system_instructions>\n\n");

    for path in &target_files {
        if let Some(ext) = path.extension() {
            if ["png", "exe", "lock", "html"].contains(&ext.to_string_lossy().as_ref()) { continue; }
        }
        if let Ok(content) = fs::read_to_string(path) {
            output_buffer.push_str(&format!("  <file path=\"{}\">\n    <![CDATA[\n{}\n    ]]>\n  </file>\n\n", path.display(), content));
            packed_count += 1;
        }
    }
    output_buffer.push_str("</context_rs_packet>");

    let token_est = output_buffer.len() / 4;
    let cost_est = (token_est as f64 / 1_000_000.0) * 5.00;

    // --- GENERATE DASHBOARD ---
    if args.smart && !dependency_map.is_empty() {
        let mut mermaid_graph = String::from("graph TD;\n");
        mermaid_graph.push_str("    classDef seed fill:#d4f1f9,stroke:#0d47a1,stroke-width:2px,color:#0d47a1;\n");
        mermaid_graph.push_str("    classDef dep fill:#e8f5e9,stroke:#1b5e20,stroke-width:1px,color:#1b5e20;\n");
        
        for (parent, child) in &dependency_map {
            let p_clean = parent.replace(".", "_");
            let c_clean = child.replace(".", "_");
            mermaid_graph.push_str(&format!("    {}-->{};\n", p_clean, c_clean));
            
            if parent.contains("main") {
                mermaid_graph.push_str(&format!("    class {} seed;\n", p_clean));
            } else {
                 mermaid_graph.push_str(&format!("    class {} dep;\n", p_clean));
            }
            mermaid_graph.push_str(&format!("    class {} dep;\n", c_clean));
        }

        let mut log_html = String::new();
        for (i, log) in logic_log.iter().enumerate() {
            log_html.push_str(&format!("<div class='log-item'><span class='step'>{}</span>{}</div>", i + 1, log));
        }

        let escaped_payload = output_buffer
            .replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;");

        // FIX: Using r##" to start and "## to end, so the inner "#" in the URL doesn't break parsing.
        let html_content = format!(r##"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Context Optimization Report</title>
            <style>
                body {{ font-family: 'Segoe UI', sans-serif; background: #f4f6f8; padding: 20px; color: #202124; }}
                .container {{ max-width: 1100px; margin: 0 auto; display: flex; flex-direction: column; gap: 20px; }}
                
                .top-section {{ display: flex; gap: 20px; }}
                .left-panel {{ flex: 2; }}
                .right-panel {{ flex: 1; }}

                .card {{ background: white; border-radius: 8px; padding: 20px; box-shadow: 0 1px 3px rgba(0,0,0,0.1); margin-bottom: 20px; }}
                h2 {{ margin-top: 0; font-size: 18px; color: #5f6368; text-transform: uppercase; letter-spacing: 1px; }}
                
                .stats {{ display: flex; justify-content: space-between; text-align: center; }}
                .stat-box .val {{ font-size: 32px; font-weight: bold; color: #1a73e8; }}
                .stat-box .lbl {{ font-size: 13px; color: #5f6368; }}

                .log-item {{ padding: 10px 0; border-bottom: 1px solid #eee; font-size: 14px; display: flex; align-items: center; }}
                .log-item .step {{ background: #e8f0fe; color: #1a73e8; width: 24px; height: 24px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-weight: bold; margin-right: 10px; font-size: 12px; }}
                
                .legend {{ margin-top: 10px; display: flex; gap: 15px; font-size: 12px; }}
                .dot {{ width: 10px; height: 10px; display: inline-block; border-radius: 50%; margin-right: 5px; }}

                /* Payload Box Styles */
                .payload-box {{ background: #282c34; color: #abb2bf; padding: 15px; border-radius: 6px; font-family: 'Consolas', monospace; font-size: 12px; overflow-x: auto; white-space: pre-wrap; max-height: 400px; overflow-y: auto; }}
                .btn-jump {{ display: inline-block; margin-top: 10px; padding: 8px 16px; background: #1a73e8; color: white; text-decoration: none; border-radius: 4px; font-weight: bold; font-size: 12px; }}
                .btn-jump:hover {{ background: #1557b0; }}
            </style>
        </head>
        <body>
            <div style="text-align:center; margin-bottom: 30px;">
                <h1>🚀 context-rs Report</h1>
                <p style="color:#5f6368;">Compiler-Aware Dependency Analysis</p>
                <a href="#payload" class="btn-jump">View Full XML Payload 👇</a>
            </div>
            
            <div class="container">
                <div class="top-section">
                    <div class="left-panel">
                        <div class="card">
                            <h2>Dependency Graph</h2>
                            <div class="legend">
                                <span><span class="dot" style="background: #d4f1f9; border: 1px solid #0d47a1;"></span>User Modified</span>
                                <span><span class="dot" style="background: #e8f5e9; border: 1px solid #1b5e20;"></span>AI Auto-Resolved</span>
                            </div>
                            <div class="mermaid">
                                {}
                            </div>
                        </div>
                    </div>

                    <div class="right-panel">
                        <div class="card">
                            <h2>Impact Analysis</h2>
                            <div class="stats">
                                <div class="stat-box">
                                    <div class="val">{}</div>
                                    <div class="lbl">Files</div>
                                </div>
                                <div class="stat-box">
                                    <div class="val" style="color: #188038;">${:.5}</div>
                                    <div class="lbl">Cost (GPT-4o)</div>
                                </div>
                            </div>
                        </div>

                        <div class="card">
                            <h2>Logic Trace</h2>
                            {}
                        </div>
                    </div>
                </div>

                <div class="card" id="payload">
                    <h2>Generated Payload (XML)</h2>
                    <p style="font-size: 12px; color: #666; margin-bottom: 10px;">This content has been copied to your clipboard.</p>
                    <div class="payload-box">{}</div>
                </div>
            </div>

            <script type="module">
                import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.esm.min.mjs';
                mermaid.initialize({{ startOnLoad: true }});
            </script>
        </body>
        </html>
        "##, mermaid_graph, packed_count, cost_est, log_html, escaped_payload); // Note the "## close

        fs::write("report.html", html_content)?;
        
        #[cfg(target_os = "windows")]
        Command::new("cmd").args(["/C", "start", "report.html"]).spawn().ok();
        #[cfg(target_os = "macos")]
        Command::new("open").args(["report.html"]).spawn().ok();
        #[cfg(target_os = "linux")]
        Command::new("xdg-open").args(["report.html"]).spawn().ok();
    }

    println!("\n{}", "📦 STEP 3: Packing Complete".green().bold());
    println!("   • Dashboard: {}", "report.html".blue().underline());
    println!("   • Est Cost:  {}", format!("${:.5}", cost_est).green());

    if !args.map {
        let mut clipboard = Clipboard::new()?;
        clipboard.set_text(&output_buffer)?;
        println!("\n{} Payload copied to clipboard!", "📋".green());
    }

    Ok(())
}