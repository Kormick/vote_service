use exonum::crypto::PublicKey;

/// VoteServiceConfig used to store service configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteServiceConfig {
    /// PublicKey to create ephemeral key for vote encryption.
    pub author_public_key: Option<PublicKey>,
}

impl Default for VoteServiceConfig {
    fn default() -> Self {
        Self {
            author_public_key: None,
        }
    }
}
