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
