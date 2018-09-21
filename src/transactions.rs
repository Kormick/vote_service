use exonum::crypto::PublicKey;

/// Transactions.
transactions! {
    /// Transaction group.
    pub VoteTransactions {
        const SERVICE_ID = super::SERVICE_ID;

        /// Transaction type for creating new candidate.
        struct TxCreateCandidate {
            /// Id of the candidate.
            pub_key: &PublicKey,
            /// Name of the candidate.
            name: &str,
            /// Info about the candidate.
            info: &str,
        }

        /// Transaction type for creating new voter.
        struct TxCreateVoter {
            /// Id of the voter.
            pub_key: &PublicKey,
            /// Name of the voter.
            name: &str,
        }

        /// Transaction type for creating new vote.
        struct TxAddVote {
            pub_key: &PublicKey,
            /// Id of the voter.
            voter_id: &PublicKey,
            /// Id of the candidate.
            candidate_id: &PublicKey,
        }
    }
}
