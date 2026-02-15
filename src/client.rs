use crate::Config;
use crate::CreateEntryResponse;
use crate::DeleteEntryResponse;
use crate::EntryId;
use crate::EntryParams;
use crate::GetEntryResponse;
use crate::ListCategoriesResponse;
use crate::ListEntriesResponse;
use crate::UpdateEntryResponse;
use reqwest::Method;
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug)]
pub struct Client {
    config: Config,
    http_client: reqwest::Client,
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("request error")]
    RequestError(#[from] reqwest::Error),
    #[error("bad request")]
    BadRequest,
    #[error("unauthorized")]
    Unauthorized,
    #[error("not found")]
    NotFound,
    #[error("method not allowed")]
    MethodNotAllowed,
    #[error("internal server error")]
    InternalServerError,
    #[error("unknown status code")]
    UnknownStatusCode,
}

impl Client {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn create_entry(
        &self,
        entry_params: EntryParams,
    ) -> Result<CreateEntryResponse, ClientError> {
        let body = entry_params.into_xml();
        self.request(Method::POST, &self.collection_uri(None), Some(body))
            .await
            .map(CreateEntryResponse::from)
    }

    pub async fn delete_entry(
        &self,
        entry_id: &EntryId,
    ) -> Result<DeleteEntryResponse, ClientError> {
        self.request(Method::DELETE, &self.member_uri(entry_id), None)
            .await
            .map(DeleteEntryResponse::from)
    }

    pub async fn get_entry(&self, entry_id: &EntryId) -> Result<GetEntryResponse, ClientError> {
        self.request(Method::GET, &self.member_uri(entry_id), None)
            .await
            .map(GetEntryResponse::from)
    }

    pub async fn list_categories(&self) -> Result<ListCategoriesResponse, ClientError> {
        self.request(Method::GET, &self.category_document_uri(), None)
            .await
            .map(ListCategoriesResponse::from)
    }

    pub async fn list_entries_in_page(
        &self,
        page: Option<&str>,
    ) -> Result<ListEntriesResponse, ClientError> {
        self.request(Method::GET, &self.collection_uri(page), None)
            .await
            .map(ListEntriesResponse::from)
    }

    pub async fn update_entry(
        &self,
        entry_id: &EntryId,
        entry_params: EntryParams,
    ) -> Result<UpdateEntryResponse, ClientError> {
        let body = entry_params.into_xml();
        self.request(Method::PUT, &self.member_uri(entry_id), Some(body))
            .await
            .map(UpdateEntryResponse::from)
    }

    fn category_document_uri(&self) -> String {
        let config = &self.config;
        format!(
            "{}/{}/{}/atom/category",
            config.base_url, config.hatena_id, config.blog_id
        )
    }

    fn collection_uri(&self, page: Option<&str>) -> String {
        let config = &self.config;
        format!(
            "{}/{}/{}/atom/entry{}",
            config.base_url,
            config.hatena_id,
            config.blog_id,
            page.map(|s| format!("?page={}", urlencoding::encode(s)))
                .unwrap_or_else(|| "".to_string())
        )
    }

    fn member_uri(&self, entry_id: &EntryId) -> String {
        let config = &self.config;
        format!(
            "{}/{}/{}/atom/entry/{}",
            config.base_url, config.hatena_id, config.blog_id, entry_id,
        )
    }

    async fn request(
        &self,
        method: Method,
        url: &str,
        body: Option<String>,
    ) -> Result<String, ClientError> {
        let config = &self.config;
        let request = self
            .http_client
            .request(method, url)
            .basic_auth(&config.hatena_id, Some(&config.api_key));
        let request = if let Some(body) = body {
            request.body(body)
        } else {
            request
        };
        let response = request.send().await?;
        match response.status() {
            status_code if status_code.is_success() => {
                let body = response.text().await?;
                Ok(body)
            }
            StatusCode::BAD_REQUEST => Err(ClientError::BadRequest),
            StatusCode::UNAUTHORIZED => Err(ClientError::Unauthorized),
            StatusCode::NOT_FOUND => Err(ClientError::NotFound),
            StatusCode::METHOD_NOT_ALLOWED => Err(ClientError::MethodNotAllowed),
            StatusCode::INTERNAL_SERVER_ERROR => Err(ClientError::InternalServerError),
            _ => Err(ClientError::UnknownStatusCode),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn config() -> Config {
        Config::new("HATENA_ID", Some("BASE_URL"), "BLOG_ID", "API_KEY")
    }

    #[test]
    fn new() {
        let config = config();
        let client = Client::new(&config);
        assert_eq!(client.config, config);
    }

    #[test]
    fn collection_uri() {
        let client = Client::new(&config());
        assert_eq!(
            client.collection_uri(None),
            "BASE_URL/HATENA_ID/BLOG_ID/atom/entry"
        )
    }

    #[test]
    fn member_uri() -> anyhow::Result<()> {
        let client = Client::new(&config());
        let entry_id = "ENTRY_ID".parse::<EntryId>()?;
        assert_eq!(
            client.member_uri(&entry_id),
            "BASE_URL/HATENA_ID/BLOG_ID/atom/entry/ENTRY_ID"
        );
        Ok(())
    }

    #[test]
    fn create_entry() {
        // See: examples/create_entry.rs
    }

    #[test]
    fn delete_entry() {
        // See: examples/delete_entry.rs
    }

    #[test]
    fn get_entry() {
        // See: examples/get_entry.rs
    }

    #[test]
    fn list_categories() {
        // See: examples/list_categories.rs
    }

    #[test]
    fn list_entries_in_page() {
        // See: examples/list_entries.rs
    }

    #[test]
    fn update_entry() {
        // See: examples/update_entry.rs
    }

    const ENTRY_XML: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<entry xmlns="http://www.w3.org/2005/Atom"
       xmlns:app="http://www.w3.org/2007/app">
  <id>tag:blog.hatena.ne.jp,2013:blog-test_user-20000000000000-3000000000000000</id>
  <link rel="edit" href="https://blog.hatena.ne.jp/test_user/test_blog/atom/entry/2500000000"/>
  <link rel="alternate" type="text/html" href="http://test_blog.hatenablog.com/entry/2013/09/02/112823"/>
  <author><name>test_user</name></author>
  <title>記事タイトル</title>
  <updated>2013-09-02T11:28:23+09:00</updated>
  <published>2013-09-02T11:28:23+09:00</published>
  <app:edited>2013-09-02T11:28:23+09:00</app:edited>
  <summary type="text"> 記事本文 リスト1 リスト2 内容 </summary>
  <content type="text/x-hatena-syntax">
** 記事本文
- リスト1
- リスト2
内容
  </content>
  <hatena:formatted-content type="text/html" xmlns:hatena="http://www.hatena.ne.jp/info/xmlns#">
    <div class="section">
    <h4>記事本文</h4>
    <ul>
    <li>リスト1</li>
    <li>リスト2</li>
    </ul><p>内容</p>
    </div>
  </hatena:formatted-content>
  <category term="Scala" />
  <category term="Perl" />
  <app:control>
    <app:draft>no</app:draft>
  </app:control>
</entry>"#;

    const FEED_XML: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom"
      xmlns:app="http://www.w3.org/2007/app">
  <link rel="first" href="https://blog.hatena.ne.jp/test_user/test_blog/atom/entry" />
  <link rel="next" href="https://blog.hatena.ne.jp/test_user/test_blog/atom/entry?page=1377584217" />
  <title>ブログタイトル</title>
  <link rel="alternate" href="http://test_blog.hatenablog.com/"/>
  <updated>2013-08-27T15:17:06+09:00</updated>
  <author>
    <name>test_user</name>
  </author>
  <generator uri="http://blog.hatena.ne.jp/" version="100000000">Hatena::Blog</generator>
  <id>hatenablog://blog/2000000000000</id>
  <entry>
    <id>tag:blog.hatena.ne.jp,2013:blog-test_user-20000000000000-3000000000000000</id>
    <link rel="edit" href="https://blog.hatena.ne.jp/test_user/test_blog/atom/entry/2500000000"/>
    <link rel="alternate" type="text/html" href="http://test_blog.hatenablog.com/entry/2013/09/02/112823"/>
    <author><name>test_user</name></author>
    <title>記事タイトル</title>
    <updated>2013-09-02T11:28:23+09:00</updated>
    <published>2013-09-02T11:28:23+09:00</published>
    <app:edited>2013-09-02T11:28:23+09:00</app:edited>
    <summary type="text"> 記事本文 リスト1 リスト2 内容 </summary>
    <content type="text/x-hatena-syntax">
** 記事本文
- リスト1
- リスト2
内容
    </content>
    <hatena:formatted-content type="text/html" xmlns:hatena="http://www.hatena.ne.jp/info/xmlns#">
      <div class="section">
      <h4>記事本文</h4>
      <ul>
      <li>リスト1</li>
      <li>リスト2</li>
      </ul><p>内容</p>
      </div>
    </hatena:formatted-content>
    <category term="Scala" />
    <category term="Perl" />
    <app:control>
      <app:draft>no</app:draft>
    </app:control>
  </entry>
</feed>"#;

    const CATEGORY_DOCUMENT_XML: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<app:categories
    xmlns:app="http://www.w3.org/2007/app"
    xmlns:atom="http://www.w3.org/2005/Atom"
    fixed="no">
  <atom:category term="Perl" />
  <atom:category term="Scala" />
</app:categories>"#;

    fn mock_config(server_url: &str) -> Config {
        Config::new("test_user", Some(server_url), "test_blog", "test_api_key")
    }

    #[tokio::test]
    async fn create_entry_with_mock() -> anyhow::Result<()> {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("POST", "/test_user/test_blog/atom/entry")
            .match_header(
                "authorization",
                mockito::Matcher::Regex("Basic .+".to_string()),
            )
            .with_status(201)
            .with_body(ENTRY_XML)
            .create_async()
            .await;
        let client = Client::new(&mock_config(&server.url()));
        let params = EntryParams::new(
            "test_user".to_string(),
            "記事タイトル".to_string(),
            "** 記事本文".to_string(),
            "2013-09-02T11:28:23+09:00".to_string(),
            vec!["Scala".to_string()],
            false,
        );
        let response = client.create_entry(params).await?;
        assert_eq!(response.to_string(), ENTRY_XML);
        mock.assert_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn delete_entry_with_mock() -> anyhow::Result<()> {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("DELETE", "/test_user/test_blog/atom/entry/2500000000")
            .match_header(
                "authorization",
                mockito::Matcher::Regex("Basic .+".to_string()),
            )
            .with_status(200)
            .create_async()
            .await;
        let client = Client::new(&mock_config(&server.url()));
        let entry_id = "2500000000".parse::<EntryId>()?;
        let response = client.delete_entry(&entry_id).await?;
        assert_eq!(response.to_string(), "");
        mock.assert_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn get_entry_with_mock() -> anyhow::Result<()> {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/test_user/test_blog/atom/entry/2500000000")
            .match_header(
                "authorization",
                mockito::Matcher::Regex("Basic .+".to_string()),
            )
            .with_status(200)
            .with_body(ENTRY_XML)
            .create_async()
            .await;
        let client = Client::new(&mock_config(&server.url()));
        let entry_id = "2500000000".parse::<EntryId>()?;
        let response = client.get_entry(&entry_id).await?;
        assert_eq!(response.to_string(), ENTRY_XML);
        mock.assert_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn list_categories_with_mock() -> anyhow::Result<()> {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/test_user/test_blog/atom/category")
            .match_header(
                "authorization",
                mockito::Matcher::Regex("Basic .+".to_string()),
            )
            .with_status(200)
            .with_body(CATEGORY_DOCUMENT_XML)
            .create_async()
            .await;
        let client = Client::new(&mock_config(&server.url()));
        let response = client.list_categories().await?;
        assert_eq!(response.to_string(), CATEGORY_DOCUMENT_XML);
        mock.assert_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn list_entries_in_page_with_mock() -> anyhow::Result<()> {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/test_user/test_blog/atom/entry")
            .match_header(
                "authorization",
                mockito::Matcher::Regex("Basic .+".to_string()),
            )
            .with_status(200)
            .with_body(FEED_XML)
            .create_async()
            .await;
        let client = Client::new(&mock_config(&server.url()));
        let response = client.list_entries_in_page(None).await?;
        assert_eq!(response.to_string(), FEED_XML);
        mock.assert_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn list_entries_in_page_with_page_with_mock() -> anyhow::Result<()> {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/test_user/test_blog/atom/entry?page=1377584217")
            .match_header(
                "authorization",
                mockito::Matcher::Regex("Basic .+".to_string()),
            )
            .with_status(200)
            .with_body(FEED_XML)
            .create_async()
            .await;
        let client = Client::new(&mock_config(&server.url()));
        let response = client.list_entries_in_page(Some("1377584217")).await?;
        assert_eq!(response.to_string(), FEED_XML);
        mock.assert_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn update_entry_with_mock() -> anyhow::Result<()> {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("PUT", "/test_user/test_blog/atom/entry/2500000000")
            .match_header(
                "authorization",
                mockito::Matcher::Regex("Basic .+".to_string()),
            )
            .with_status(200)
            .with_body(ENTRY_XML)
            .create_async()
            .await;
        let client = Client::new(&mock_config(&server.url()));
        let entry_id = "2500000000".parse::<EntryId>()?;
        let params = EntryParams::new(
            "test_user".to_string(),
            "記事タイトル".to_string(),
            "** 記事本文".to_string(),
            "2013-09-02T11:28:23+09:00".to_string(),
            vec!["Scala".to_string()],
            false,
        );
        let response = client.update_entry(&entry_id, params).await?;
        assert_eq!(response.to_string(), ENTRY_XML);
        mock.assert_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn get_entry_unauthorized_with_mock() -> anyhow::Result<()> {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/test_user/test_blog/atom/entry/2500000000")
            .with_status(401)
            .create_async()
            .await;
        let client = Client::new(&mock_config(&server.url()));
        let entry_id = "2500000000".parse::<EntryId>()?;
        let result = client.get_entry(&entry_id).await;
        assert!(matches!(result, Err(ClientError::Unauthorized)));
        mock.assert_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn get_entry_not_found_with_mock() -> anyhow::Result<()> {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/test_user/test_blog/atom/entry/2500000000")
            .with_status(404)
            .create_async()
            .await;
        let client = Client::new(&mock_config(&server.url()));
        let entry_id = "2500000000".parse::<EntryId>()?;
        let result = client.get_entry(&entry_id).await;
        assert!(matches!(result, Err(ClientError::NotFound)));
        mock.assert_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn get_entry_internal_server_error_with_mock() -> anyhow::Result<()> {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/test_user/test_blog/atom/entry/2500000000")
            .with_status(500)
            .create_async()
            .await;
        let client = Client::new(&mock_config(&server.url()));
        let entry_id = "2500000000".parse::<EntryId>()?;
        let result = client.get_entry(&entry_id).await;
        assert!(matches!(result, Err(ClientError::InternalServerError)));
        mock.assert_async().await;
        Ok(())
    }
}
