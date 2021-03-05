use crate::Fail;
use std::process::{Command, Stdio};

#[cfg(target_os = "linux")]
/// Extract package count for debian-based, arch-based distros or NetBSD
pub fn package_count(fail: &mut Fail) -> String {
    // Instead of having a condition for the millions of distros
    // this function will try and extract package count by checking
    // if a certain package manager is installed
    let wh = Command::new("which")
        .arg("pacman")
        .output()
        .expect("Failed to start 'which' process");
    let which = String::from_utf8(wh.stdout).expect("'which' process stdout was not valid UTF-8");
    if !which.is_empty() {
        let pacman = Command::new("pacman")
            .args(&["-Q", "-q"])
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to start \"pacman\" process");

        let pac_out = pacman
            .stdout
            .expect("ERROR: failed to open \"pacman\" stdout");

        let count = Command::new("wc")
            .arg("-l")
            .stdin(Stdio::from(pac_out))
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to start \"wc\" process");

        let output = count
            .wait_with_output()
            .expect("ERROR: failed to wait for \"wc\" process to exit");
        return String::from_utf8(output.stdout)
            .expect("ERROR: \"pacman -Qq | wc -l\" output was not valid UTF-8")
            .trim()
            .to_string();
    }
    // If pacman is not installed, try dpkg.
    else {
        let wh = Command::new("which")
            .arg("dpkg")
            .output()
            .expect("ERROR: failed to start \"which\" process");
        let which = String::from_utf8(wh.stdout)
            .expect("ERROR: \"which\" process stdout was not valid UTF-8");
        if !which.is_empty() {
            let pacman = Command::new("dpkg")
                .arg("-l")
                .stdout(Stdio::piped())
                .spawn()
                .expect("ERROR: failed to start \"dpkg\" process");

            let pac_out = pacman
                .stdout
                .expect("ERROR: failed to open \"dpkg\" stdout");

            let count = Command::new("wc")
                .arg("-l")
                .stdin(Stdio::from(pac_out))
                .stdout(Stdio::piped())
                .spawn()
                .expect("ERROR: failed to start \"wc\" process");

            let output = count
                .wait_with_output()
                .expect("ERROR: failed to wait for \"wc\" process to exit");
            return String::from_utf8(output.stdout)
                .expect("ERROR: \"dpkg -l | wc -l\" output was not valid UTF-8")
                .trim()
                .to_string();
        }
        // If dpkg is not installed, try emerge.
        else {
            let wh = Command::new("which")
                .arg("emerge")
                .output()
                .expect("ERROR: failed to start \"which\" process");
            let which = String::from_utf8(wh.stdout)
                .expect("ERROR: \"which\" process stdout was not valid UTF-8");
            if !which.is_empty() {
                let pacman = Command::new("ls")
                    .arg("/var/db/pkg/*")
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("ERROR: failed to start \"ls\" process");

                let pac_out = pacman.stdout.expect("ERROR: failed to open \"ls\" stdout");

                let count = Command::new("wc")
                    .arg("-l")
                    .stdin(Stdio::from(pac_out))
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("ERROR: failed to start \"wc\" process");

                let output = count
                    .wait_with_output()
                    .expect("ERROR: failed to wait for \"wc\" process to exit");
                return String::from_utf8(output.stdout)
                    .expect("ERROR: \"dpkg -l | wc -l\" output was not valid UTF-8")
                    .trim()
                    .to_string();
            }
        }
    }
    fail.packages.failed = true;
    // If neither pacman or dpkg is installed, return 0
    return String::from("0");
}

#[cfg(target_os = "netbsd")]
/// Extract package count through `pacman -Qq | wc -l`
pub fn package_count(fail: &mut Fail) -> String {
    let wh = Command::new("which")
        .arg("pkg_info")
        .output()
        .expect("ERROR: failed to start \"which\" process");

    let which =
        String::from_utf8(wh.stdout).expect("ERROR: \"which\" process stdout was not valid UTF-8");
    // Continue only if pacman exists
    if !which.is_empty() {
        let pkg_info = Command::new("pkg_info")
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to spawn \"pkg_info\" process");

        let pkg_out = pkg_info
            .stdout
            .expect("ERROR: failed to open \"pkg_info\" stdout");

        let count = Command::new("wc")
            .arg("-l")
            .stdin(Stdio::from(pkg_out))
            .stdout(Stdio::piped())
            .spawn()
            .expect("ERROR: failed to start \"wc\" process");

        let output = count
            .wait_with_output()
            .expect("ERROR: failed to wait on for \"wc\" process to exit");
        return String::from_utf8(output.stdout)
            .expect("ERROR: \"pkg_info | wc -l\" output was not valid UTF-8")
            .trim()
            .to_string();
    }
    fail.packages.failed = true;
    // If pkg_info is not installed, return 0
    return String::from("0");
}
