use std::option::Option::Some;

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft)),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(&mut self) {
        if let Some(current_state) = self.state.take() {
            self.state = Some(current_state.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(current_state) = self.state.take() {
            self.state = Some(current_state.approve());
        }
    }

    pub fn reject(&mut self) {
        if let Some(current_state) = self.state.take() {
            self.state = Some(current_state.reject());
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
}


trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    //returned string slice's lifetime as same as the post
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft;

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview)
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }


    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview;

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft)
    }


    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published)
    }
}

struct Published;

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }


    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
