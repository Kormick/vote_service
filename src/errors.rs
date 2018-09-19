#![allow(bare_trait_objects)]

use exonum::blockchain::ExecutionError;

#[derive(Debug, Fail)]
#[repr(u8)]
pub enum Error {
    #[fail(display = "Candidate already exists")]
    CandidateAlreadyExists = 0,

    #[fail(display = "Voter already exists")]
    VoterAlreadyExists = 1,

    #[fail(display = "Vote already exists")]
    VoteAlreadyExists = 2,

    #[fail(display = "Candidate not found")]
    CandidateNotFound = 3,

    #[fail(display = "Candidate result not found")]
    CandidateResultNotFound = 4,

    #[fail(display = "Voter not found")]
    VoterNotFound = 5,

    #[fail(display = "Invalid ephemeral")]
    InvalidEphemeral = 6,
}

impl From<Error> for ExecutionError {
    fn from(value: Error) -> ExecutionError {
        let description = format!("{}", value);
        ExecutionError::with_description(value as u8, description)
    }
}
