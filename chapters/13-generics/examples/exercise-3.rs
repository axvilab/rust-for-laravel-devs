//! Вправа 3: дерево коментарів через `Vec` (без Box) і ланцюжок через `Option<Box<_>>`
//!
//! `struct Reply { text: String, parent: Option<Reply> }` дає
//! E0072 recursive type has infinite size (broken/exercise-3.rs):
//! Option<Reply> зберігає Reply всередині себе, а той — ще один Reply...
//! Box ставить між ними вказівник фіксованого розміру.

struct Comment {
    text: String,
    replies: Vec<Comment>, // Vec уже в купі — Box не потрібен
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
        text: "Перший".into(),
        replies: vec![
            Comment {
                text: "Відповідь".into(),
                replies: vec![Comment {
                    text: "Відповідь на відповідь".into(),
                    replies: vec![],
                }],
            },
            Comment {
                text: "Ще одна".into(),
                replies: vec![],
            },
        ],
    };
    println!("коментарів: {} (у гілці «{}»)", total(&thread), thread.text); // 4

    let root = Reply {
        text: "корінь".into(),
        parent: None,
    };
    let child = Reply {
        text: "дитина".into(),
        parent: Some(Box::new(root)),
    };
    println!("глибина «{}»: {}", child.text, depth(&child)); // 2
}
