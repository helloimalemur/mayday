use std::process::Command;
use std::thread;

pub async fn start_front_end() {
    thread::spawn(|| {
        if let Err(e) = Command::new("bash")
            .arg("-e")
            .arg("mayday-frontend/start-frontend.sh")
            .spawn() {
            eprintln!("{}", e)
        }
    });
}

#[cfg(test)]
mod tests {
    use crate::frontend::start_front_end;

    #[test]
    // #[ignore]
    fn front_end() {
        let tk = tokio::runtime::Runtime::new();
        tk.unwrap().block_on(start_front_end());
    }
}
