use crate::*;

pub struct SearchClient<'a> {
    client: &'a Client,
}

impl<'a> SearchClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn search(&self, request: &SearchQuery) -> Result<SearchResponse, ClientError> {
        let mut parts: Vec<String> = Vec::new();
        if let Some(q) = &request.query {
            parts.push(format!("query={q}"));
        }
        if let Some(a) = &request.agent {
            parts.push(format!("agent={a}"));
        }
        if let Some(k) = request.kind {
            parts.push(format!("kind={}", k.as_str()));
        }
        if let Some(t) = &request.texture {
            parts.push(format!("texture={t}"));
        }
        if let Some(l) = &request.level {
            parts.push(format!("level={l}"));
        }
        if let Some(s) = &request.sensation {
            parts.push(format!("sensation={s}"));
        }
        parts.push(format!("limit={}", request.filters.limit));
        parts.push(format!("offset={}", request.filters.offset));

        let path = format!("/search?{}", parts.join("&"));
        self.client.get(&path).await
    }
}
