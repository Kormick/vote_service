#[macro_use]
extern crate exonum;
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod schema {
    use exonum::{
        crypto::PublicKey,
        storage::{Fork, MapIndex, Snapshot},
    };

    encoding_struct! {
        struct Candidate {
            pub_key: &PublicKey,
            name: &str,
            info: &str,
        }
    }

    encoding_struct! {
        struct Voter {
            pub_key: &PublicKey,
            name: &str,
        }
    }

    encoding_struct! {
        struct Vote {
            from: &PublicKey,
            to: &PublicKey,
        }
    }

    encoding_struct! {
        struct CandidateResult {
            candidate: &PublicKey,
            votes: u64,
            voters: Vec<Voter>,
        }
    }

    #[derive(Debug)]
    pub struct VoteServiceSchema<T> {
        view: T,
    }

    impl<T: AsRef<dyn Snapshot>> VoteServiceSchema<T> {
        pub fn new(view: T) -> Self {
            VoteServiceSchema { view }
        }

        pub fn candidates(&self) -> MapIndex<&dyn Snapshot, PublicKey, Candidate> {
            MapIndex::new("voteservice.candidates", self.view.as_ref())
        }

        pub fn candidate(&self, pub_key: &PublicKey) -> Option<Candidate> {
            self.candidates().get(pub_key)
        }

        pub fn voters(&self) -> MapIndex<&dyn Snapshot, PublicKey, Voter> {
            MapIndex::new("voteservice.voters", self.view.as_ref())
        }

        pub fn voter(&self, pub_key: &PublicKey) -> Option<Voter> {
            self.voters().get(pub_key)
        }

        pub fn votes(&self) -> MapIndex<&dyn Snapshot, PublicKey, Vote> {
            MapIndex::new("voteservice.votes", self.view.as_ref())
        }

        pub fn vote(&self, pub_key: &PublicKey) -> Option<Vote> {
            self.votes().get(pub_key)
        }

        pub fn vote_results(&self) -> MapIndex<&dyn Snapshot, PublicKey, CandidateResult> {
            MapIndex::new("voteservice.results", self.view.as_ref())
        }

        pub fn candidate_result(&self, pub_key: &PublicKey) -> Option<CandidateResult> {
            self.vote_results().get(pub_key)
        }
    }

    impl<'a> VoteServiceSchema<&'a mut Fork> {
        pub fn candidates_mut(&mut self) -> MapIndex<&mut Fork, PublicKey, Candidate> {
            MapIndex::new("voteservice.candidates", &mut self.view)
        }

        pub fn voters_mut(&mut self) -> MapIndex<&mut Fork, PublicKey, Voter> {
            MapIndex::new("voteservice.voters", &mut self.view)
        }

        pub fn votes_mut(&mut self) -> MapIndex<&mut Fork, PublicKey, Vote> {
            MapIndex::new("voteservice.votes", &mut self.view)
        }

        pub fn vote_results_mut(&mut self) -> MapIndex<&mut Fork, PublicKey, CandidateResult> {
            MapIndex::new("voteservice.results", &mut self.view)
        }
    }
}

pub mod transactions {
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
}

pub mod contracts {
    use exonum::{
        blockchain::{ExecutionResult, Transaction},
        messages::Message,
        storage::Fork,
    };

    // use errors::Error;
    use schema::{Candidate, CandidateResult, Vote, VoteServiceSchema, Voter};
    use transactions::{TxAddVote, TxCreateCandidate, TxCreateVoter};

    impl Transaction for TxCreateCandidate {
        fn verify(&self) -> bool {
            // self.verify_signature(self.pub_key())
            true // FIXME
        }

        fn execute(&self, view: &mut Fork) -> ExecutionResult {
            let mut schema = VoteServiceSchema::new(view);
            if schema.candidate(self.pub_key()).is_none() {
                let candidate = Candidate::new(self.pub_key(), self.name(), self.info());
                println!(
                    "TxCreateCandidate::execute: Create the candidate: {:?}",
                    candidate
                );
                schema.candidates_mut().put(self.pub_key(), candidate);

                let candidate_res = CandidateResult::new(self.pub_key(), 0, vec![]);
                println!(
                    "TxCreateCandidate::execute: Create Candidate result: {:?}",
                    candidate_res
                );
                schema.vote_results_mut().put(self.pub_key(), candidate_res);

                Ok(())
            } else {
                // TODO error
                println!("TxCreateCandidate::execute: candidate already exists");
                Ok(())
            }
        }
    }

    impl Transaction for TxCreateVoter {
        fn verify(&self) -> bool {
            // self.verify_signature(self.pub_key())
            true // FIXME
        }

        fn execute(&self, view: &mut Fork) -> ExecutionResult {
            let mut schema = VoteServiceSchema::new(view);
            if schema.voter(self.pub_key()).is_none() {
                let voter = Voter::new(self.pub_key(), self.name());
                println!("TxCreateVoter::execute: Create the voter: {:?}", voter);
                schema.voters_mut().put(self.pub_key(), voter);
                Ok(())
            } else {
                // TODO error
                println!("TxCreateVoter::execute: Voter already exists");
                Ok(())
            }
        }
    }

    impl Transaction for TxAddVote {
        fn verify(&self) -> bool {
            // self.verify_signature(self.pub_key());
            true // FIXME
        }

        fn execute(&self, view: &mut Fork) -> ExecutionResult {
            let mut schema = VoteServiceSchema::new(view);

            if schema.candidate(self.candidate_id()).is_none() {
                // TODO error
                println!("TxAddVote::execute: Candidate not found");
                return Ok(());
            }

            if schema.voter(self.voter_id()).is_none() {
                // TODO error
                println!("TxAddVote::execute: Voter not found");
                return Ok(());
            }

            if schema.vote(self.voter_id()).is_none() {
                let vote = Vote::new(self.voter_id(), self.candidate_id());
                println!("TxAddVote::execute: Add vote {:?}", vote);
                schema.votes_mut().put(self.voter_id(), vote);

                let result = schema.candidate_result(self.candidate_id()).unwrap();
                let mut voters = result.voters();
                voters.push(schema.voter(self.voter_id()).unwrap());
                let result = CandidateResult::new(result.candidate(), result.votes() + 1, voters);
                schema.vote_results_mut().put(self.candidate_id(), result);

                Ok(())
            } else {
                // TODO error
                println!("TxAddVote::execute: vote already exists");
                Ok(())
            }
        }
    }
}

pub mod api {
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
        pub fn get_candidate(
            state: &ServiceApiState,
            query: CandidateQuery,
        ) -> api::Result<Candidate> {
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

        pub fn get_results(
            state: &ServiceApiState,
            _query: (),
        ) -> api::Result<Vec<CandidateResult>> {
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
}

pub mod service {
    use exonum::{
        api::ServiceApiBuilder,
        blockchain::{Service, Transaction, TransactionSet},
        crypto::Hash,
        encoding,
        messages::RawTransaction,
        storage::Snapshot,
    };

    use api::VoteServiceApi;
    use transactions::VoteTransactions;

    pub const SERVICE_ID: u16 = 42;

    #[derive(Debug)]
    pub struct VoteService;

    impl Service for VoteService {
        fn service_name(&self) -> &'static str {
            println!("VoteService::service_name");
            "voteservice"
        }

        fn service_id(&self) -> u16 {
            SERVICE_ID
        }

        fn tx_from_raw(
            &self,
            raw: RawTransaction,
        ) -> Result<Box<dyn Transaction>, encoding::Error> {
            println!("VoteService::tx_from_raw");
            let tx = VoteTransactions::tx_from_raw(raw)?;
            Ok(tx.into())
        }

        fn state_hash(&self, _: &dyn Snapshot) -> Vec<Hash> {
            vec![]
        }

        fn wire_api(&self, builder: &mut ServiceApiBuilder) {
            println!("VoteService::wire_api");
            VoteServiceApi::wire(builder);
        }
    }
}

pub mod factory {
    use exonum::blockchain;
    use exonum::helpers::fabric;
    use service::VoteService;

    #[derive(Debug, Clone, Copy)]
    pub struct ServiceFactory;

    impl fabric::ServiceFactory for ServiceFactory {
        fn service_name(&self) -> &str {
            "voteservice"
        }

        fn make_service(&mut self, _: &fabric::Context) -> Box<dyn blockchain::Service> {
            Box::new(VoteService)
        }
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
