use exonum::crypto::PublicKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteServiceConfig {
    pub author_public_key: Option<PublicKey>,
}

impl Default for VoteServiceConfig {
    fn default() -> Self {
        Self {
            author_public_key: None,
        }
    }
}
