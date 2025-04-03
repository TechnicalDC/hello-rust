pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

pub struct Comments {
    pub username: String,
    pub comment: String
}

pub struct Youtube {
    pub channel: String,
    pub title: String,
    pub views: i32,
    pub likes: i32,
    pub comments: Vec<Comments>
}

// impl  Youtube {
//     fn get_comments(&self) -> Vec<Comments> {
//         self.comments
//     }
// }

pub trait  Summary {
    fn summarize(&self) -> String;
}

// impl <Traits> for <Type>
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

// impl <Traits> for <Type>
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/* NOTE: For future references.
pub fn get_content<T: Summary>(item: &T) {
    println!("{}", item.summarize())
}

pub fn get_content1<T>(item: &T)
    where T: Summary
{
    println!("{}", item.summarize())
}
*/

pub fn test_traits() {
    let new_article: NewsArticle = NewsArticle {
        author: "dilip chauhan".to_string(),
        headline: "first article".to_string(),
        content: "Working on Traits!".to_string()
    };
    let new_video: Youtube = Youtube {
        channel: "@technicaldc".to_string(),
        title: "First Video".to_string(),
        views: 0,
        likes: 0,
        comments: vec![
            Comments {
                username: "dilip chauhan".to_string(),
                comment: "first comment".to_string()
            }
        ]
    };

    println!("{}", new_video.comments.len());

    println!("Author: {}", new_article.author);
    println!("Headline: {}", new_article.headline);
    println!("Content: {}", new_article.content);
    println!("Summary: {}", new_article.summarize());
}
