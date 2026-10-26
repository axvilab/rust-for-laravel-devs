//! Exercise 3: a comment tree with `Vec` (no Box) and a chain with `Option<Box<_>>`
//!
//! `struct Reply { text: String, parent: Option<Reply> }` gives
//! E0072 recursive type has infinite size (broken/exercise-3.rs):
//! Option<Reply> stores a Reply inside itself, which holds another Reply...
//! Box puts a fixed-size pointer between them.

struct Comment {
    text: String,
    replies: Vec<Comment>, // Vec is already on the heap — no Box needed
}

fn total(c: &Comment) -> usize {
    1 + c.replies.iter().map(total).sum::<usize>()
}

struct Reply {
    text: String,
    parent: Option<Box<Reply>>,
}

fn depth(r: &Reply) -> usize {
    1 + r.parent.as_deref().map(depth).unwrap_or(0)
}

fn main() {
    let thread = Comment {
        text: "First".into(),
        replies: vec![
            Comment {
                text: "A reply".into(),
                replies: vec![Comment {
                    text: "A reply to the reply".into(),
                    replies: vec![],
                }],
            },
            Comment {
                text: "Another one".into(),
                replies: vec![],
            },
        ],
    };
    println!(
        "comments: {} (in the thread '{}')",
        total(&thread),
        thread.text
    ); // 4

    let root = Reply {
        text: "root".into(),
        parent: None,
    };
    let child = Reply {
        text: "child".into(),
        parent: Some(Box::new(root)),
    };
    println!("depth of '{}': {}", child.text, depth(&child)); // 2
}
