use cipher;
use errors::Error;
use exonum::{
    blockchain::{ExecutionResult, Transaction},
    crypto::CryptoHash,
    storage::Fork,
};
use schema::{Candidate, CandidateResult, Vote, VoteServiceSchema, Voter};
use transactions::{TxAddVote, TxCreateCandidate, TxCreateVoter};

impl Transaction for TxCreateCandidate {
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
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

            let candidate_res = CandidateResult::new(self.pub_key(), vec![], 0);
            println!(
                "TxCreateCandidate::execute: Create Candidate result: {:?}",
                candidate_res
            );
            schema.vote_results_mut().put(self.pub_key(), candidate_res);

            Ok(())
        } else {
            Err(Error::CandidateAlreadyExists)?
        }
    }
}

impl Transaction for TxCreateVoter {
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        let mut schema = VoteServiceSchema::new(view);
        if schema.voter(self.pub_key()).is_none() {
            let voter = Voter::new(self.pub_key(), self.name());
            println!("TxCreateVoter::execute: Create the voter: {:?}", voter);
            schema.voters_mut().put(self.pub_key(), voter);
            Ok(())
        } else {
            Err(Error::VoterAlreadyExists)?
        }
    }
}

impl Transaction for TxAddVote {
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        let mut schema = VoteServiceSchema::new(view);

        if schema.candidate(self.candidate_id()).is_none() {
            Err(Error::CandidateNotFound)?
        }

        if schema.voter(self.voter_id()).is_none() {
            Err(Error::VoterNotFound)?
        }

        let voter_hash = self.voter_id().hash();
        if schema.vote(&voter_hash).is_none() {
            let vote = Vote::new(self.voter_id(), self.candidate_id());
            let enc_vote = cipher::encrypt_vote(&vote);
            println!("TxAddVote::execute: Add encrypted vote {:?}", enc_vote);
            schema.votes_mut().put(&voter_hash, enc_vote.clone());

            let result = match schema.candidate_result(self.candidate_id()) {
                Some(res) => res,
                None => Err(Error::CandidateResultNotFound)?,
            };
            let mut votes = result.votes();
            votes.push(enc_vote);
            let votes_num = votes.len() as u64;
            let result = CandidateResult::new(self.candidate_id(), votes, votes_num);
            schema.vote_results_mut().put(self.candidate_id(), result);
            Ok(())
        } else {
            Err(Error::VoteAlreadyExists)?
        }
    }
}
