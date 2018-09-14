use exonum::crypto::PublicKey;

use service::SERVICE_ID;

transactions! {
    pub VoteTransactions {
        const SERVICE_ID = SERVICE_ID;

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
