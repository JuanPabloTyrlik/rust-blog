use blog::{oop, rust_way};

fn main() {
    let mut post = oop::Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.add_text("and a steak for dinner");
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    let mut post = rust_way::Post::new();

    post.add_text("I ate a salad for lunch today");
    post.add_text(" and it was delicious!");

    let post = post.request_review();

    let post = post.approve();

    let post = post.approve();
    assert_eq!(
        "I ate a salad for lunch today and it was delicious!",
        post.content()
    );
}
