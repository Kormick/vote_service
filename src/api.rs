use exonum::{
    api::{self, ServiceApiBuilder, ServiceApiState},
    blockchain::{Schema, Transaction},
    crypto::{Hash, PublicKey},
    messages::Message,
    node::TransactionSend,
};

use schema::{Candidate, CandidateResult, Vote, VoteServiceSchema, Voter};
use transactions::{TxAddVote, VoteTransactions};

#[derive(Debug, Clone)]
pub struct VoteServiceApi;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct CandidateQuery {
    pub pub_key: PublicKey,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct VoterQuery {
    pub pub_key: PublicKey,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct VoteQuery {
    pub pub_key: PublicKey,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct BlockQuery {
    pub pub_key: PublicKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub tx_hash: Hash,
}

impl VoteServiceApi {
    pub fn get_candidate(state: &ServiceApiState, query: CandidateQuery) -> api::Result<Candidate> {
        let snapshot = state.snapshot();
        let schema = VoteServiceSchema::new(snapshot);
        schema
            .candidate(&query.pub_key)
            .ok_or_else(|| api::Error::NotFound("Candidate not found".to_string()))
    }

    pub fn get_candidates(state: &ServiceApiState, _query: ()) -> api::Result<Vec<Candidate>> {
        let snapshot = state.snapshot();
        let schema = VoteServiceSchema::new(snapshot);
        let idx = schema.candidates();
        let candidates = idx.values().collect();
        Ok(candidates)
    }

    pub fn get_voter(state: &ServiceApiState, query: VoterQuery) -> api::Result<Voter> {
        let snapshot = state.snapshot();
        let schema = VoteServiceSchema::new(snapshot);
        schema
            .voter(&query.pub_key)
            .ok_or_else(|| api::Error::NotFound("Voter not found".to_string()))
    }

    pub fn get_voters(state: &ServiceApiState, _query: ()) -> api::Result<Vec<Voter>> {
        let snapshot = state.snapshot();
        let schema = VoteServiceSchema::new(snapshot);
        let idx = schema.voters();
        let voters = idx.values().collect();
        Ok(voters)
    }

    pub fn get_vote(state: &ServiceApiState, query: VoteQuery) -> api::Result<Vote> {
        let snapshot = state.snapshot();
        let schema = VoteServiceSchema::new(snapshot);
        schema
            .vote(&query.pub_key)
            .ok_or_else(|| api::Error::NotFound("Vote not found".to_string()))
    }

    pub fn get_votes(state: &ServiceApiState, _query: ()) -> api::Result<Vec<Vote>> {
        let snapshot = state.snapshot();
        let schema = VoteServiceSchema::new(snapshot);
        let idx = schema.votes();
        let votes = idx.values().collect();
        Ok(votes)
    }

    pub fn get_results(state: &ServiceApiState, _query: ()) -> api::Result<Vec<CandidateResult>> {
        let snapshot = state.snapshot();
        let schema = VoteServiceSchema::new(snapshot);
        let idx = schema.vote_results();
        let results = idx.values().collect();
        Ok(results)
    }

    pub fn post_transaction(
        state: &ServiceApiState,
        query: VoteTransactions,
    ) -> api::Result<TransactionResponse> {
        println!("VoteServiceApi::post_transaction");
        let transaction: Box<dyn Transaction> = query.into();
        let tx_hash = transaction.hash();
        state.sender().send(transaction)?;
        Ok(TransactionResponse { tx_hash })
    }

    // test api function
    pub fn foo42(_state: &ServiceApiState, _query: ()) -> api::Result<u32> {
        Ok(42)
    }

    pub fn get_block(state: &ServiceApiState, query: BlockQuery) -> api::Result<u64> {
        println!("VoteServiceApi::get_block");
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

        Ok(43) // FIXME
    }

    pub fn wire(builder: &mut ServiceApiBuilder) {
        println!("VoteServiceApi::wire");
        builder
                .public_scope()
                .endpoint("v1/foo42", Self::foo42) // test function
                .endpoint("v1/candidate", Self::get_candidate)
                .endpoint("v1/candidates", Self::get_candidates)
                .endpoint("v1/voter", Self::get_voter)
                .endpoint("v1/voters", Self::get_voters)
                .endpoint("v1/vote", Self::get_vote)
                .endpoint("v1/votes", Self::get_votes)
                .endpoint("v1/results", Self::get_results)
                .endpoint("v1/block", Self::get_block)
                .endpoint_mut("v1/candidates", Self::post_transaction)
                .endpoint_mut("v1/voters", Self::post_transaction)
                .endpoint_mut("v1/votes", Self::post_transaction);
    }
}
