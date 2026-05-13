use crate::platform;
use std::path::Path;
use std::process::Command;

pub fn run_binary(project_name: &str, profile: &str, target_dir: Option<&str>) -> i32 {
    let bin_path = platform::bin_path(profile, project_name, target_dir);
    if Path::new(&bin_path).exists() {
        let output = Command::new(&bin_path).output();
        match output {
            Ok(o) => {
                print!("{}", String::from_utf8_lossy(&o.stdout));
                eprint!("{}", String::from_utf8_lossy(&o.stderr));
                return o.status.code().unwrap_or(0);
            }
            Err(_) => {
                return 1;
            }
        }
    }
    1
}
