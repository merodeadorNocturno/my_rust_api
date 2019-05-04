use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use uuid::Uuid;

#[derive(Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct Post {
  pub title: String,
  pub body: String,
  pub author: String,
  pub datetime: DateTime<UTC>,
  pub uuid:Uuid,
}

impl Post {
  pub fn new(title: &str,
    body: &str,
    author: &str,
    datetime: DateTime<UTC>,
    uuid: Uuid
  ) -> Post {
      Post {
        title: title.to_string(),
        body: body.to_string(),
        author: author.to_string(),
        datetime,
        uuid,
      }
    }

  pub fn uuid(&self) -> &Uuid {
    &self.uuid
  }
}