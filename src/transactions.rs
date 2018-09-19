use exonum::crypto::PublicKey;

transactions! {
    pub VoteTransactions {
        const SERVICE_ID = super::SERVICE_ID;

        struct TxCreateCandidate {
            pub_key: &PublicKey,
            name: &str,
            info: &str,
        }

        struct TxCreateVoter {
            pub_key: &PublicKey,
            name: &str,
        }

        struct TxAddVote {
            voter_id: &PublicKey,
            candidate_id: &PublicKey,
        }
    }
}
