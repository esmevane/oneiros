use crate::*;

pub struct SearchClient<'a> {
    client: &'a Client,
}

impl<'a> SearchClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn search(&self, request: &SearchQuery) -> Result<SearchResponse, ClientError> {
        let query = request.current()?;
        let mut parts: Vec<String> = Vec::new();
        if let Some(q) = &query.query {
            parts.push(format!("query={q}"));
        }
        if let Some(a) = &query.agent {
            parts.push(format!("agent={a}"));
        }
        if let Some(k) = query.kind {
            parts.push(format!("kind={}", k.as_str()));
        }
        if let Some(t) = &query.texture {
            parts.push(format!("texture={t}"));
        }
        if let Some(l) = &query.level {
            parts.push(format!("level={l}"));
        }
        if let Some(s) = &query.sensation {
            parts.push(format!("sensation={s}"));
        }
        parts.push(format!("limit={}", query.filters.limit));
        parts.push(format!("offset={}", query.filters.offset));

        let path = format!("/search?{}", parts.join("&"));
        self.client.get(&path).await
    }
}
