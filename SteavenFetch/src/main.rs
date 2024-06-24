use std::process::Command;

fn main() {
    // Run flag
    let run = 1;

    if run == 1 {

        // Software 
        let output = Command::new("bash")
            .arg("-c")
            .arg("echo '┏━━━━━━━━━━━━━ Software Information ━━━━━━━━━━━━━┓'")
            .output()
            .expect("Failed to execute command");
        print!("{}", String::from_utf8_lossy(&output.stdout));

        // Username@Hostname
    let output = Command::new("bash")
        .arg("-c")
        .arg("whoami")
        .output()
        .expect("Failed to execute command");
    // Print the combined output in a single line
    print!("{}", String::from_utf8_lossy(&output.stdout).trim());
    let output = Command::new("bash")
        .arg("-c")
        .arg("echo @")
        .output()
        .expect("Failed to execute command");

    // Print the combined output in a single line
    print!("{}", String::from_utf8_lossy(&output.stdout).trim());


    let output = Command::new("bash")
        .arg("-c")
        .arg("/bin/cat /etc/hostname")
        .output()
        .expect("Failed to execute command");
        print!("{}", String::from_utf8_lossy(&output.stdout));


        let output = Command::new("bash")
        .arg("-c")
        .arg("echo Pacman:⠀")
        .output()
        .expect("Failed to execute command");
        print!("{}", String::from_utf8_lossy(&output.stdout).trim());

        // PACMAN
        let output = Command::new("bash")
            .arg("-c")
            .arg("pacman -Qq | wc -l")
            .output()
            .expect("Failed to execute command");
            print!("{}", String::from_utf8_lossy(&output.stdout));
        // OS: Linux
        let output = Command::new("bash")
            .arg("-c")
            .arg("echo 'OS: Linux'")
            .output()
            .expect("Failed to execute command");
        print!("{}", String::from_utf8_lossy(&output.stdout));

        // Linux Distro
        let output = Command::new("bash")
            .arg("-c")
            .arg("echo -n 'Linux Distro: ' && grep -E '^(VERSION|NAME)=' /etc/os-release")
            .output()
            .expect("Failed to execute command");
        print!("{}", String::from_utf8_lossy(&output.stdout));

        // Kernel
        let output = Command::new("bash")
            .arg("-c")
            .arg("echo -n 'Kernel: ' && uname -r")
            .output()
            .expect("Failed to execute command");
        print!("{}", String::from_utf8_lossy(&output.stdout));

        // DE
    let output = Command::new("bash")
        .arg("-c")
        .arg("echo DE: $XDG_SESSION_DESKTOP")
        .output()
        .expect("Failed to execute command");
        print!("{}", String::from_utf8_lossy(&output.stdout));

        // DS
    let output = Command::new("bash")
        .arg("-c")
        .arg("echo DS: $XDG_SESSION_TYPE")
        .output()
        .expect("Failed to execute command");
        print!("{}", String::from_utf8_lossy(&output.stdout));


        // Shell
        let output = Command::new("bash")
        .arg("-c")
        .arg("echo Shell: $SHELL")
        .output()
        .expect("Failed to execute command");
        print!("{}", String::from_utf8_lossy(&output.stdout));

        let output = Command::new("bash")
            .arg("-c")
            .arg("echo '━━━━━━━━━━━━━ Hardware Information ━━━━━━━━━━━━━'")
            .output()
            .expect("Failed to execute command");
        print!("{}", String::from_utf8_lossy(&output.stdout));
        // Model
        let output = Command::new("bash")
            .arg("-c")
            .arg("echo -n 'Model: ' && sudo dmidecode -s system-product-name")
            .output()
            .expect("Failed to execute command");
        print!("{}", String::from_utf8_lossy(&output.stdout));

        // Serial Number
        let output = Command::new("bash")
            .arg("-c")
            .arg("echo -n 'Serial Number: ' && sudo dmidecode -s system-serial-number")
            .output()
            .expect("Failed to execute command");
        print!("{}", String::from_utf8_lossy(&output.stdout));

        // CPU
        let output = Command::new("bash")
            .arg("-c")
            .arg("echo -n 'CPU: ' && lscpu | grep 'Model name:' | sed -r 's/Model name:\\s{1,}//g'")
            .output()
            .expect("Failed to execute command");
        print!("{}", String::from_utf8_lossy(&output.stdout));

        // GPU
        let output = Command::new("bash")
            .arg("-c")
            .arg("echo -n 'GPU: ' && lspci | grep -i VGA")
            .output()
            .expect("Failed to execute command");
        print!("{}", String::from_utf8_lossy(&output.stdout));

        // RAM
        let output = Command::new("bash")
            .arg("-c")
            .arg("echo -n 'RAM: ' && grep MemTotal /proc/meminfo")
            .output()
            .expect("Failed to execute command");
        print!("{}", String::from_utf8_lossy(&output.stdout));

        // Uptime
        let output = Command::new("bash")
            .arg("-c")
            .arg("echo -n 'Uptime: ' && uptime -p")
            .output()
            .expect("Failed to execute command");
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }
}
