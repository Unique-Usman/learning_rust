pub struct Post {
    content: String,
}

pub struct DraftPost {
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

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            count: 0,
            option: None,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
    count: u32,
    option: Option<Post>,
}

impl PendingReviewPost {
    pub fn approve(self) -> Option<Post> {
        if self.count == 0 {
            return None;
        } else {
            Some(Post {
                content: self.content,
            })
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}
