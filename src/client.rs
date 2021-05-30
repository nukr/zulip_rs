use crate::message::*;
use crate::rc::{parse_from_str, ZulipRuntimeConfig};
use crate::CommonMutateResponse;
use anyhow::{Context, Result};
use reqwest::{Method, RequestBuilder};

pub struct Client {
    rc: ZulipRuntimeConfig,
}

impl Client {
    pub fn new(zulip_rc: &str) -> Result<Self> {
        let rc = parse_from_str(zulip_rc)?;
        Ok(Client { rc })
    }
    pub fn parse(zulip_rc: &str) -> Result<Self> {
        let rc = parse_from_str(zulip_rc)?;
        Ok(Client { rc })
    }
    pub async fn send_message(&self, req: SendMessageRequest) -> Result<SendMessageResponse> {
        let result = self
            .http_client(Method::POST, "/api/v1/messages")
            .form(&req)
            .send()
            .await?;
        let resp: SendMessageResponse = result.json().await?;
        Ok(resp)
    }
    pub async fn get_messages(&self, req: GetMessagesRequest) -> Result<GetMessagesResponse> {
        let qs = serde_qs::to_string(&req)?;
        let narrow = serde_json::to_string(&req.narrow)?;
        let qs = format!("/api/v1/messages?{}&narrow={}", qs, narrow);
        println!("{}", qs);
        let resp: GetMessagesResponse = self
            .http_client(Method::GET, &qs)
            .send()
            .await
            .context("get messages send failed")?
            .json()
            .await
            .context("deserialize to GetMessagesResponse failed")?;
        Ok(resp)
    }
    pub async fn delete_message(&self, id: i64) -> Result<DeleteMessageResponse> {
        let resp: DeleteMessageResponse = self
            .http_client(Method::DELETE, &format!("/api/v1/messages/{}", id))
            .send()
            .await
            .context("delete message failed")?
            .json()
            .await
            .context("deserialize to DeleteMessageResponse failed")?;
        Ok(resp)
    }
    pub async fn edit_message(&self, req: EditMessageRequest) -> Result<CommonMutateResponse> {
        let resp: CommonMutateResponse = self
            .http_client(
                Method::PATCH,
                &format!("/api/v1/messages/{}", req.message_id),
            )
            .form(&req)
            .send()
            .await
            .context("delete message failed")?
            .json()
            .await
            .context("deserialize to DeleteMessageResponse failed")?;
        Ok(resp)
    }
    pub async fn add_emoji_reaction(
        &self,
        req: AddEmojiReactionRequest,
    ) -> Result<CommonMutateResponse> {
        let resp: CommonMutateResponse = self
            .http_client(
                Method::POST,
                &format!("/api/v1/messages/{}/reactions", req.message_id),
            )
            .form(&req)
            .send()
            .await
            .context("delete message failed")?
            .json()
            .await
            .context("deserialize to DeleteMessageResponse failed")?;
        Ok(resp)
    }
    pub async fn remove_emoji_reaction(
        &self,
        req: RemoveEmojiReactionRequest,
    ) -> Result<CommonMutateResponse> {
        let resp: CommonMutateResponse = self
            .http_client(
                Method::DELETE,
                &format!("/api/v1/messages/{}/reactions", req.message_id),
            )
            .form(&req)
            .send()
            .await
            .context("delete message failed")?
            .json()
            .await
            .context("deserialize to DeleteMessageResponse failed")?;
        Ok(resp)
    }
    fn http_client(&self, method: Method, endpoint: &str) -> RequestBuilder {
        let client = reqwest::Client::new();
        let url = format!("{}{}", &self.rc.api.site, endpoint);
        client
            .request(method, url)
            .basic_auth(&self.rc.api.email, Some(&self.rc.api.key))
            .header("application", "x-www-form-urlencoded")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::{
        Method::{DELETE, GET, POST},
        MockServer,
    };
    use std::net::SocketAddr;

    #[tokio::test]
    async fn test_send_private_message() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).path("/api/v1/messages");
            then.status(200)
                .body(r#"{"result": "success", "msg": "", "id": 123}"#);
        });
        let client = Client::new(&rc(server.address())).unwrap();
        let req = SendMessageRequest::Private {
            to: "[8]".to_string(),
            content: "abc".to_string(),
        };
        let result = client.send_message(req).await;
        mock.assert();
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_send_stream_message() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).path("/api/v1/messages");
            then.status(200)
                .body(r#"{"result": "success", "msg": "", "id": 123}"#);
        });
        let client = Client::new(&rc(server.address())).unwrap();
        let req = SendMessageRequest::Stream {
            to: "[8]".to_string(),
            topic: "test".to_string(),
            content: "abc".to_string(),
        };
        let result = client.send_message(req).await;
        mock.assert();
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_get_messages() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/api/v1/messages");
            then.status(200).body(message_template());
        });
        let client = Client::new(&rc(server.address())).unwrap();

        let req = GetMessagesRequest::new(0, 0);

        let result = client.get_messages(req).await;
        mock.assert();
        assert!(result.is_ok());
    }
    #[tokio::test]
    async fn test_delete_messages() {
        let server = MockServer::start();
        let id = 123;
        let mock = server.mock(|when, then| {
            when.method(DELETE).path(format!("/api/v1/messages/{}", id));
            then.status(200).body(r#"{"result": "success", "msg": ""}"#);
        });
        let client = Client::new(&rc(server.address())).unwrap();
        let result = client.delete_message(id).await;
        mock.assert();
        assert!(result.is_ok());
    }
    fn rc(addr: &SocketAddr) -> String {
        format!(
            "[api]\nemail=test@test.tester.com\nkey=testkey\nsite=http://{}\n",
            addr
        )
    }
    fn message_template() -> String {
        r#"{
    "anchor": 21,
    "found_anchor": true,
    "found_newest": true,
    "messages": [
        {
            "avatar_url": "https://secure.gravatar.com/avatar/6d8cad0fd00256e7b40691d27ddfd466?d=identicon&version=1",
            "client": "populate_db",
            "content": "<p>Security experts agree that relational algorithms are an interesting new topic in the field of networking, and scholars concur.</p>",
            "content_type": "text/html",
            "display_recipient": [
                {
                    "email": "hamlet@zulip.com",
                    "full_name": "King Hamlet",
                    "id": 4,
                    "is_mirror_dummy": false
                },
                {
                    "email": "iago@zulip.com",
                    "full_name": "Iago",
                    "id": 5,
                    "is_mirror_dummy": false
                },
                {
                    "email": "prospero@zulip.com",
                    "full_name": "Prospero from The Tempest",
                    "id": 8,
                    "is_mirror_dummy": false
                }
            ],
            "flags": [
                "read"
            ],
            "id": 16,
            "is_me_message": false,
            "reactions": [],
            "recipient_id": 27,
            "sender_email": "hamlet@zulip.com",
            "sender_full_name": "King Hamlet",
            "sender_id": 4,
            "sender_realm_str": "zulip",
            "subject": "",
            "submessages": [],
            "timestamp": 1527921326,
            "topic_links": [],
            "type": "private"
        },
        {
            "avatar_url": "https://secure.gravatar.com/avatar/6d8cad0fd00256e7b40691d27ddfd466?d=identicon&version=1",
            "client": "populate_db",
            "content": "<p>Wait, is this from the frontend js code or backend python code</p>",
            "content_type": "text/html",
            "display_recipient": "Verona",
            "flags": [
                "read"
            ],
            "id": 21,
            "is_me_message": false,
            "reactions": [],
            "recipient_id": 20,
            "sender_email": "hamlet@zulip.com",
            "sender_full_name": "King Hamlet",
            "sender_id": 4,
            "sender_realm_str": "zulip",
            "stream_id": 5,
            "subject": "Verona3",
            "submessages": [],
            "timestamp": 1527939746,
            "topic_links": [],
            "type": "stream"
        }
    ],
    "msg": "",
    "result": "success"
}"#.to_string()
    }
}
