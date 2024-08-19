use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
 
fn main() -> io::Result<()> {
    // Get the user's home directory
    let home_dir = env::var("HOME")?;
    let bash_history_path = Path::new(&home_dir).join(".bash_history");
 
    // Define the path for the log file
    let log_file_path = Path::new(&home_dir).join("bash_history_log.txt");
 
    // Read the bash history file
    let history_content = fs::read_to_string(&bash_history_path)?;
 
    // Write the history content to the log file
    fs::write(&log_file_path, history_content)?;
 
    println!("Bash history saved to {:?}", log_file_path);
 
    Ok(())
}
