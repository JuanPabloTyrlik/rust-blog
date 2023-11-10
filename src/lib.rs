pub mod oop {
    use std::cell::RefCell;

    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }
        pub fn add_text(&mut self, text: &str) {
            self.content = self.state.as_ref().unwrap().add_text(&self.content, text);
        }
        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }
        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }
        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
        pub fn reject(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.reject())
            }
        }
    }

    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn reject(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, _post: &'a Post) -> &'a str {
            ""
        }
        fn add_text(&self, current_content: &str, _text_to_append: &str) -> String {
            current_content.to_string()
        }
    }

    struct Draft {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {
                approvals: RefCell::new(0),
            })
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn add_text(&self, current_content: &str, text_to_append: &str) -> String {
            format!("{}{}", current_content, text_to_append)
        }
    }

    struct PendingReview {
        approvals: RefCell<u8>,
    }

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            *self.approvals.borrow_mut() += 1;
            if *self.approvals.borrow() > 1 {
                Box::new(Published {})
            } else {
                self
            }
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft {})
        }
    }

    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }
}

pub mod rust_way {
    pub struct Post {
        content: String,
    }

    impl Post {
        pub fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }

        pub fn content(&self) -> &str {
            &self.content
        }
    }
    pub struct DraftPost {
        content: String,
    }

    impl DraftPost {
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }
    pub struct PendingReviewPost {
        content: String,
    }

    impl PendingReviewPost {
        pub fn approve(self) -> ApprovedPendingReviewPost {
            ApprovedPendingReviewPost {
                content: self.content,
            }
        }

        pub fn reject(self) -> DraftPost {
            DraftPost {
                content: self.content,
            }
        }
    }

    pub struct ApprovedPendingReviewPost {
        content: String,
    }

    impl ApprovedPendingReviewPost {
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }

        pub fn reject(self) -> DraftPost {
            DraftPost {
                content: self.content,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod oop {
        use crate::oop::Post;

        #[test]
        fn it_adds_text_only_in_draft() {
            let mut post = Post::new();

            post.add_text("I ate a salad for lunch today");
            assert_eq!("", post.content());

            post.request_review();
            assert_eq!("", post.content());

            post.add_text("and a steak for dinner");
            assert_eq!("", post.content());

            post.approve();
            assert_eq!("", post.content());

            post.approve();
            assert_eq!("I ate a salad for lunch today", post.content());
        }

        #[test]
        fn it_can_reject_posts() {
            let mut post = Post::new();

            post.add_text("I ate a salad for lunch today");
            assert_eq!("", post.content());

            post.request_review();
            assert_eq!("", post.content());

            post.reject();
            assert_eq!("", post.content());

            post.add_text(" and a steak for dinner");
            assert_eq!("", post.content());

            post.request_review();
            post.approve();
            post.approve();
            assert_eq!(
                "I ate a salad for lunch today and a steak for dinner",
                post.content()
            );
        }

        #[test]
        fn it_requires_two_approvals() {
            let mut post = Post::new();

            post.add_text("I ate a salad for lunch today");
            assert_eq!("", post.content());

            post.request_review();
            assert_eq!("", post.content());

            post.approve();
            assert_eq!("", post.content());
        }

        #[test]
        fn it_prints_content_when_published() {
            let mut post = Post::new();

            post.add_text("I ate a salad for lunch today");
            assert_eq!("", post.content());

            post.request_review();
            assert_eq!("", post.content());

            post.approve();
            assert_eq!("", post.content());

            post.approve();
            assert_eq!("I ate a salad for lunch today", post.content());
        }
    }

    mod rust_way {
        use crate::rust_way::*;

        #[test]
        fn it_can_create_and_publish_a_blog() {
            let mut post = Post::new();

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

        #[test]
        fn it_can_reject_a_blog() {
            let mut post = Post::new();

            post.add_text("I ate a salad for lunch today");
            post.add_text(" and it was delicious!");

            let post = post.request_review();

            let post = post.reject();

            let post = post.request_review();

            let post = post.approve();
            let post = post.approve();
            assert_eq!(
                "I ate a salad for lunch today and it was delicious!",
                post.content()
            );
        }
    }
}
