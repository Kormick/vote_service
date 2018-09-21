use agreement;
use cipher;
use exonum::{
    api::{self, ServiceApiBuilder, ServiceApiState},
    blockchain::{Schema, Transaction},
    crypto::{Hash, PublicKey},
    messages::Message,
    node::TransactionSend,
};
use schema::{
    Candidate, CandidateResult, DecryptedCandidateResult, EncryptedVote, VoteResult,
    VoteServiceSchema, Voter,
};
use transactions::{TxAddVote, VoteTransactions};

/// REST API.

/// Public service API description.
#[derive(Debug, Clone)]
pub struct VoteServiceApi;

/// The structure describes the query parameters for the `get_candidate` endpoint.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct CandidateQuery {
    pub pub_key: PublicKey,
}

/// The structure describes the query parameters for the `get_voter` endpoint.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct VoterQuery {
    pub pub_key: PublicKey,
}

/// The structure describes the query parameters for the `get_block` endpoint.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct BlockQuery {
    pub pub_key: PublicKey,
}

/// The structure returned by REST API.
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub tx_hash: Hash,
}

/// REST API implementation.
impl VoteServiceApi {
    /// Endpoint for getting a candidate.
    pub fn get_candidate(state: &ServiceApiState, query: CandidateQuery) -> api::Result<Candidate> {
        let snapshot = state.snapshot();
        let schema = VoteServiceSchema::new(snapshot);
        schema
            .candidate(&query.pub_key)
            .ok_or_else(|| api::Error::NotFound("Candidate not found".to_string()))
    }

    /// Endpoint for getting all candidates.
    pub fn get_candidates(state: &ServiceApiState, _query: ()) -> api::Result<Vec<Candidate>> {
        let snapshot = state.snapshot();
        let schema = VoteServiceSchema::new(snapshot);
        let idx = schema.candidates();
        let candidates = idx.values().collect();
        Ok(candidates)
    }

    /// Endpoint for getting a voter.
    pub fn get_voter(state: &ServiceApiState, query: VoterQuery) -> api::Result<Voter> {
        let snapshot = state.snapshot();
        let schema = VoteServiceSchema::new(snapshot);
        schema
            .voter(&query.pub_key)
            .ok_or_else(|| api::Error::NotFound("Voter not found".to_string()))
    }

    /// Endpoint for getting all voters.
    pub fn get_voters(state: &ServiceApiState, _query: ()) -> api::Result<Vec<Voter>> {
        let snapshot = state.snapshot();
        let schema = VoteServiceSchema::new(snapshot);
        let idx = schema.voters();
        let voters = idx.values().collect();
        Ok(voters)
    }

    /// Endpoint for getting all encrypted votes.
    pub fn get_votes(state: &ServiceApiState, _query: ()) -> api::Result<Vec<EncryptedVote>> {
        let snapshot = state.snapshot();
        let schema = VoteServiceSchema::new(snapshot);
        let idx = schema.votes();
        let votes = idx.values().collect();
        Ok(votes)
    }

    /// Endpoint for getting all encrypted vote results and service public key.
    pub fn get_results(state: &ServiceApiState, _query: ()) -> api::Result<VoteResult> {
        let service_public_key = agreement::get_ephemeral().public_out_key;
        let service_public_key = match PublicKey::from_slice(&service_public_key) {
            Some(val) => val,
            None => panic!("api::get_results: failed to get ephemeral key"),
        };

        let snapshot = state.snapshot();
        let schema = VoteServiceSchema::new(snapshot);
        let idx = schema.vote_results();
        let candidates = idx.values().collect();
        let results = VoteResult::new(&service_public_key, candidates);

        Ok(results)
    }

    /// Endpoint for getting all decrypted vote results.
    pub fn get_results_decrypted(
        state: &ServiceApiState,
        _query: (),
    ) -> api::Result<Vec<DecryptedCandidateResult>> {
        let snapshot = state.snapshot();
        let schema = VoteServiceSchema::new(snapshot);
        let idx = schema.vote_results();
        let results: Vec<CandidateResult> = idx.values().collect();

        let mut dec_results = vec![];
        for res in results.iter() {
            let mut dec_res_votes = vec![];

            for vote in res.votes().iter() {
                let dec_vote = cipher::decrypt_vote(vote);
                dec_res_votes.push(dec_vote);
            }

            let votes_num = dec_res_votes.len() as u64;
            let mut dec_res =
                DecryptedCandidateResult::new(res.candidate(), dec_res_votes, votes_num);
            dec_results.push(dec_res);
        }

        Ok(dec_results)
    }

    /// Endpoint for getting a block height.
    pub fn get_block(state: &ServiceApiState, query: BlockQuery) -> api::Result<u64> {
        let snapshot = state.snapshot();
        let ex_schema = Schema::new(snapshot);

        let transactions = ex_schema.transactions();
        for raw_mes in transactions.values() {
            let parsed = TxAddVote::from_raw(raw_mes.clone());
            if parsed.is_ok() {
                let mes = parsed.unwrap();
                if mes.voter_id().clone() == query.pub_key {
                    let locations = ex_schema.transactions_locations();
                    let loc = locations.get(&raw_mes.hash()).unwrap();
                    return Ok(loc.block_height().0);
                }
            }
        }

        Err(api::Error::NotFound("Block not found".to_string()))
    }

    /// Common processing for transaction-accepting endpoints.
    pub fn post_transaction(
        state: &ServiceApiState,
        query: VoteTransactions,
    ) -> api::Result<TransactionResponse> {
        let transaction: Box<dyn Transaction> = query.into();
        let tx_hash = transaction.hash();
        state.sender().send(transaction)?;
        Ok(TransactionResponse { tx_hash })
    }

    /// 'ServiceApiBuilder' facilitates conversion between transactions/read requests and REST
    /// endpoints; for example, it parses `POST`ed JSON into the binary transaction
    /// representation used in Exonum internally.
    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            .endpoint("v1/candidate", Self::get_candidate)
            .endpoint("v1/candidates", Self::get_candidates)
            .endpoint("v1/voter", Self::get_voter)
            .endpoint("v1/voters", Self::get_voters)
            .endpoint("v1/votes", Self::get_votes)
            .endpoint("v1/results", Self::get_results)
            .endpoint("v1/results_dec", Self::get_results_decrypted)
            .endpoint("v1/block", Self::get_block)
            .endpoint_mut("v1/candidates", Self::post_transaction)
            .endpoint_mut("v1/voters", Self::post_transaction)
            .endpoint_mut("v1/votes", Self::post_transaction);
    }
}
