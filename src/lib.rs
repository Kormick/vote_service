#[macro_use]
extern crate exonum;
// #[macro_use]
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod schema {
    use exonum::{
        crypto::PublicKey, storage::{Fork, MapIndex, Snapshot},
    };

    encoding_struct! {
        struct Candidate {
            pub_key: &PublicKey,
            name: &str,
            votes: u64
        }
    }

    impl Candidate {
        pub fn add_vote(self) -> Self {
            let new_votes = self.votes() + 1;
            Self::new(self.pub_key(), self.name(), new_votes)
        }
    }

    #[derive(Debug)]
    pub struct VoteServiceSchema<T> {
        view: T
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
    }

    impl<'a> VoteServiceSchema<&'a mut Fork> {
        pub fn candidates_mut(&mut self) -> MapIndex<&mut Fork, PublicKey, Candidate> {
            MapIndex::new("voteservice.candidates", &mut self.view)
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
            }

            struct TxAddVote {
                pub_key: &PublicKey,
            }

   //          // test struct
			// struct TxVote {
			// 	pub_key: &PublicKey,
			// 	name: &str
			// }

   //          // test struct
			// struct TxVoteTransfer {
			// 	rom: &PublicKey,
   //              to: &PublicKey,
   //              amount: u64,
   //              seed: u64
			// }
		}
	}
}

pub mod contracts {
	use exonum::{
        blockchain::{ExecutionResult, Transaction},
        messages::Message,
        storage::Fork
    };

    // use errors::Error;
    use schema::{VoteServiceSchema, Candidate};
    use transactions::{TxCreateCandidate, TxAddVote};

    impl Transaction for TxCreateCandidate {
        fn verify(&self) -> bool {
            println!("TxCreateCandidate::verify");
            self.verify_signature(self.pub_key())
        }

        fn execute(&self, view: &mut Fork) -> ExecutionResult {
            println!("TxCreateCandidate::execute");
            let mut schema = VoteServiceSchema::new(view);
            if schema.candidate(self.pub_key()).is_none() {
                let candidate = Candidate::new(self.pub_key(), self.name(), 0);
                println!("Create the candidate: {:?}", candidate);
                schema.candidates_mut().put(self.pub_key(), candidate);
                Ok(())
            } else {
                // TODO error
                println!("TxCreateCandidate::execute: candidate already exists");
                Ok(())
            }
        }
    }

    impl Transaction for TxAddVote {
        fn verify(&self) -> bool {
            println!("TxAddVote::verify");
            self.verify_signature(self.pub_key())
        }

        fn execute(&self, view: &mut Fork) -> ExecutionResult {
            println!("TxAddVote::execute");
            let mut schema = VoteServiceSchema::new(view);

            let candidate = schema.candidate(self.pub_key()).unwrap();
            // let candidate = match schema.candidate(self.pub_key()) {
            //     Some(val) => val,
            //     _ => { println!("TxAddVote::execute: candidate not found"); Ok(()) }
            // };

            let candidate = candidate.add_vote();
            println!("TxAddVote::execute: add vote for {:?}", candidate);

            let mut candidates = schema.candidates_mut();
            candidates.put(self.pub_key(), candidate);

            Ok(())
        }
    }
}

pub mod api {
    use exonum::{
        api::{self, ServiceApiBuilder, ServiceApiState}, 
        blockchain::Transaction,
        crypto::{Hash, PublicKey}, 
        node::TransactionSend
    };

    use schema::{VoteServiceSchema, Candidate};
    use transactions::VoteTransactions;

    #[derive(Debug, Clone)]
    pub struct VoteServiceApi;

    #[derive(Debug, Serialize, Deserialize, Clone, Copy)]
    pub struct CandidateQuery {
        pub pub_key: PublicKey
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TransactionResponse {
        /// Hash of the transaction.
        pub tx_hash: Hash
    }

    impl VoteServiceApi {
        pub fn get_candidate(state: &ServiceApiState, query: CandidateQuery) -> api::Result<Candidate> {
            println!("VoteServiceApi::get_candidate");
            let snapshot = state.snapshot();
            let schema = VoteServiceSchema::new(snapshot);
            // TODO add error
            schema
                .candidate(&query.pub_key)
                .ok_or_else(|| api::Error::NotFound("Candidate not found".to_string()))
        }

        pub fn get_candidates(state: &ServiceApiState, _query: ()) -> api::Result<Vec<Candidate>> {
            println!("VoteServiceApi::get_candidates");
            let snapshot = state.snapshot();
            let schema = VoteServiceSchema::new(snapshot);
            let idx = schema.candidates();
            let candidates = idx.values().collect();
            Ok(candidates)
        }

        pub fn post_transaction(state: &ServiceApiState, query: VoteTransactions) -> api::Result<TransactionResponse> {
            println!("VoteServiceApi::post_transaction");
            let transaction: Box<dyn Transaction> = query.into();
            let tx_hash = transaction.hash();
            state.sender().send(transaction)?;
            Ok(TransactionResponse { tx_hash })
        }

        // test api function
        pub fn foo0(_state: &ServiceApiState, _query: ()) -> api::Result<u32> {
            println!("VoteServiceApi::foo0");
            Ok(0)
        }

        // test api function
        pub fn foo42(_state: &ServiceApiState, _query: ()) -> api::Result<u32> {
            println!("VoteServiceApi::foo42");
            Ok(42)
        }

        pub fn wire(builder: &mut ServiceApiBuilder) {
            println!("VoteServiceApi::wire");
            builder
                .public_scope()
                .endpoint("v1/foo0", Self::foo0) // test function
                .endpoint("v1/foo42", Self::foo42) // test function
                .endpoint("v1/candidate", Self::get_candidate)
                .endpoint("v1/candidates", Self::get_candidates)
                .endpoint_mut("v1/candidates", Self::post_transaction)
                .endpoint_mut("v1/candidates/vote", Self::post_transaction);
        }
    }
}

pub mod service {
	use exonum::{
        blockchain::{Service, Transaction, TransactionSet},
        crypto::Hash,
        messages::RawTransaction, 
        encoding, 
        storage::Snapshot, 
        api::ServiceApiBuilder
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
        	// println!("VoteService::service_id");
        	SERVICE_ID
        }

        fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<dyn Transaction>, encoding::Error> {
        	// TODO

        	println!("VoteService::tx_from_raw");
        	let tx = VoteTransactions::tx_from_raw(raw)?;
        	Ok(tx.into())
        }

        fn state_hash(&self, _: &dyn Snapshot) -> Vec<Hash> {
        	// TODO

        	// println!("VoteService::state_hash");
        	vec![]
        }

        fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        	// TODO

        	println!("VoteService::wire_api");
            VoteServiceApi::wire(builder);
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
