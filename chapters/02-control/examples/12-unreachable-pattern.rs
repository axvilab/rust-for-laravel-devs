//! `_` угорі ковтає все: гілки нижче недосяжні
//!
//! Компілятор попереджає `unreachable pattern` («no value can reach this»).
//! Атрибут `allow` тут лише для CI (`-D warnings`); прибери його, щоб побачити попередження.
#![allow(unreachable_patterns)]

fn main() {
    let status = 404;
    let label = match status {
        _ => "unknown",
        404 => "not found", // сюди ніхто не дійде
    };
    println!("{label}"); // unknown
}
