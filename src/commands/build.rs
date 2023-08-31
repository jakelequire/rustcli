// sudo ln -s /home/jake/Desktop/programming/rustcli /usr/local/bin/rustcli
use crate::types::BuildArgs;

pub fn execute(args: &BuildArgs) {
    println!("Executing build command with args: {:?}", args);

    let target_os = std::env::var("TARGET").unwrap();
    println!("target_os: {}", target_os);

    // Identify what OS is being ran:
    if cfg!(target_os = "linux") {
        println!("This is linux");
    } else if cfg!(target_os = "windows") {
        println!("This is windows");
    } else if cfg!(target_os = "macos") {
        println!("This is macos");
    } else {
        println!("This is unknown");
    }

    // For linux
    let sudo = std::process::Command::new("sudo")
        .arg("ln")
        .arg("-s")
        .arg("/home/jake/Desktop/programming/rustcli/target/debug/rustcli")
        .arg("/usr/local/bin/rustcli")
        .spawn()
        .expect("Failed to execute command");

    let output = sudo;
    println!("output: {:?}", output);

    // // For windows
    // let win = std::process::Command::new("mklink")
    //     .arg("C:/Programming/rustcli/target/debug/rustcli")
    //     .arg("C:/Users/jake/AppData/Local/Microsoft/WindowsApps/rustcli")
    //     .spawn()
    //     .expect("Failed to execute command");
    
}