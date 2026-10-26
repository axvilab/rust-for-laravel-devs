//! `loop` — нескінченний цикл до `break` (у PHP — `while (true)`)

// Заглушка «підключення»: вдається з третьої спроби.
fn try_connect(attempt: u32) -> bool {
    println!("спроба {attempt}...");
    attempt == 3
}

fn main() {
    let mut attempts = 0;
    loop {
        attempts += 1;
        if try_connect(attempts) {
            break;
        }
        if attempts == 5 {
            panic!("не вдалось підключитися");
        }
    }
    println!("підключено з {attempts}-ї спроби");
}
