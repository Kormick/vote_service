#![allow(bare_trait_objects)]

use exonum::blockchain::ExecutionError;

/// Contract errors.

/// Error codes emitted by transactions during execution.
#[derive(Debug, Fail)]
#[repr(u8)]
pub enum Error {
    /// Candidate already exists.
    ///
    /// Can be emitted by `TxCreateCandidate`.
    #[fail(display = "Candidate already exists")]
    CandidateAlreadyExists = 0,

    /// Voter already exists.
    ///
    /// Can be emitted by `TxCreateVoter`.
    #[fail(display = "Voter already exists")]
    VoterAlreadyExists = 1,

    /// Vote already exists.
    ///
    /// Can be emitted by `TxAddVote`.
    #[fail(display = "Vote already exists")]
    VoteAlreadyExists = 2,

    /// Candidate not found.
    ///
    /// Can be emitted by `TxAddVote`.
    #[fail(display = "Candidate not found")]
    CandidateNotFound = 3,

    /// Candidate vote result not found.
    ///
    /// Can be emitted by `TxAddVote`.
    #[fail(display = "Candidate result not found")]
    CandidateResultNotFound = 4,

    /// Voter not found.
    ///
    /// Can be emitted by `TxAddVote`.
    #[fail(display = "Voter not found")]
    VoterNotFound = 5,
}

impl From<Error> for ExecutionError {
    fn from(value: Error) -> ExecutionError {
        let description = format!("{}", value);
        ExecutionError::with_description(value as u8, description)
    }
}
