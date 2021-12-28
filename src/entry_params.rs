#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryParams {
    author_name: String,
    title: String,
    content: String,
    updated: String, // YYYY-MM-DDTHH:MM:SS
    categories: Vec<String>,
    draft: bool,
}

impl EntryParams {
    pub fn new(
        author_name: String,
        title: String,
        content: String,
        updated: String, // YYYY-MM-DDTHH:MM:SS
        categories: Vec<String>,
        draft: bool,
    ) -> Self {
        Self {
            author_name,
            title,
            content,
            updated,
            categories,
            draft,
        }
    }

    pub fn into_xml(self) -> String {
        fn escape(t: &mut String, s: String) {
            for c in s.chars() {
                match c {
                    '"' => t.push_str("&quot;"),
                    '&' => t.push_str("&amp;"),
                    '\'' => t.push_str("&apos;"),
                    '<' => t.push_str("&lt;"),
                    '>' => t.push_str("&gt;"),
                    _ => t.push(c),
                }
            }
        }

        let mut s = String::new();
        s.push_str(r#"<?xml version="1.0" encoding="utf-8"?>"#);
        s.push('\n');
        s.push_str(r#"<entry xmlns="http://www.w3.org/2005/Atom""#);
        s.push('\n');
        s.push_str(r#"       xmlns:app="http://www.w3.org/2007/app">"#);
        s.push('\n');

        s.push_str(r#"  <title>"#);
        escape(&mut s, self.title);
        s.push_str(r#"</title>"#);
        s.push('\n');

        s.push_str(r#"  <author><name>"#);
        escape(&mut s, self.author_name);
        s.push_str(r#"</name></author>"#);
        s.push('\n');

        s.push_str(r#"  <content type="text/plain">"#);
        escape(&mut s, self.content);
        s.push_str(r#"</content>"#);
        s.push('\n');

        s.push_str(r#"  <updated>"#);
        escape(&mut s, self.updated);
        s.push_str(r#"</updated>"#);
        s.push('\n');

        for category in self.categories.into_iter() {
            s.push_str(r#"  <category term=""#);
            escape(&mut s, category);
            s.push_str(r#"" />"#);
            s.push('\n');
        }

        s.push_str(r#"  <app:control>"#);
        s.push('\n');
        s.push_str(r#"    <app:draft>"#);
        s.push_str(if self.draft { "yes" } else { "no" });
        s.push_str(r#"</app:draft>"#);
        s.push('\n');
        s.push_str(r#"  </app:control>"#);
        s.push('\n');

        s.push_str(r#"</entry>"#);
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_dummy() -> EntryParams {
        EntryParams::new(
            "AUTHOR_NAME".to_string(),
            "TITLE".to_string(),
            "CONTENT".to_string(),
            "2020-02-07T00:00:00Z".to_string(),
            vec!["CATEGORY".to_string()],
            true,
        )
    }

    #[test]
    fn into_xml() {
        let entry = new_dummy();
        assert_eq!(
            entry.into_xml(),
            r#"<?xml version="1.0" encoding="utf-8"?>
<entry xmlns="http://www.w3.org/2005/Atom"
       xmlns:app="http://www.w3.org/2007/app">
  <title>TITLE</title>
  <author><name>AUTHOR_NAME</name></author>
  <content type="text/plain">CONTENT</content>
  <updated>2020-02-07T00:00:00Z</updated>
  <category term="CATEGORY" />
  <app:control>
    <app:draft>yes</app:draft>
  </app:control>
</entry>"#
        );
    }
}
