use exonum::{
    crypto::{Hash, PublicKey},
    storage::{Fork, MapIndex, Snapshot},
};

/// Persistent data.

encoding_struct! {
    /// Candidate struct used to persist data for candidate within service.
    struct Candidate {
        /// Id of the candidate.
        pub_key: &PublicKey,
        /// Name of the candidate.
        name: &str,
        /// Info about candidate.
        info: &str,
    }
}

encoding_struct! {
    /// Voter struct used to persist data for voter within service.
    struct Voter {
        /// Id of the voter.
        pub_key: &PublicKey,
        /// Name of the voter.
        name: &str,
    }
}

encoding_struct! {
    /// Vote struct used to persist data for vote within service.
    struct Vote {
        /// Id of the voter.
        from: &PublicKey,
        /// Id of the candidate.
        to: &PublicKey,
    }
}

encoding_struct! {
    /// EncryptedVote struct used to persist encrypted data for vote within service.
    struct EncryptedVote {
        /// Encrypted data vector.
        data: Vec<u8>,
    }
}

encoding_struct! {
    /// DecryptedCandidateResult struct used to persist decrypted data for candidate vote result within service.
    struct DecryptedCandidateResult {
        /// Id of the candidate.
        candidate: &PublicKey,
        /// Vector of decrypted votes.
        votes: Vec<Vote>,
        /// Number of the votes.
        vote_num: u64
    }
}

encoding_struct! {
    /// CandidateResult struct used to persist encrypted data for candidate vote result within service.
    struct CandidateResult {
        /// Id of the candidate.
        candidate: &PublicKey,
        /// Vector of encrypted votes.
        votes: Vec<EncryptedVote>,
        /// Number of the votes.
        vote_num: u64
    }
}

encoding_struct! {
    /// VoteResult struct used to persist encrypted data for vote result within service.
    struct VoteResult {
        /// Public key of vote service.
        pub_key: &PublicKey,
        /// Vector of encrypted results for candidates.
        candidate_results: Vec<CandidateResult>,
    }
}

/// Schema of the key-value storage used by vote service.
#[derive(Debug)]
pub struct VoteServiceSchema<T> {
    view: T,
}

/// Declare the layout of data managed by the service.
impl<T: AsRef<dyn Snapshot>> VoteServiceSchema<T> {
    /// Creates a new schema instance.
    pub fn new(view: T) -> Self {
        VoteServiceSchema { view }
    }

    /// Returns an immutable version of candidates table.
    pub fn candidates(&self) -> MapIndex<&dyn Snapshot, PublicKey, Candidate> {
        MapIndex::new("voteservice.candidates", self.view.as_ref())
    }

    /// Returns a specific candidate data.
    pub fn candidate(&self, pub_key: &PublicKey) -> Option<Candidate> {
        self.candidates().get(pub_key)
    }

    /// Returns an immutable version of voters table.
    pub fn voters(&self) -> MapIndex<&dyn Snapshot, PublicKey, Voter> {
        MapIndex::new("voteservice.voters", self.view.as_ref())
    }

    /// Returns a specific voter data.
    pub fn voter(&self, pub_key: &PublicKey) -> Option<Voter> {
        self.voters().get(pub_key)
    }

    /// Returns an immutable version of votes table.
    pub fn votes(&self) -> MapIndex<&dyn Snapshot, Hash, EncryptedVote> {
        MapIndex::new("voteservice.votes", self.view.as_ref())
    }

    /// Returns a specific vote data.
    pub fn vote(&self, hash: &Hash) -> Option<EncryptedVote> {
        self.votes().get(hash)
    }

    /// Returns an immutable version of vote results table.
    pub fn vote_results(&self) -> MapIndex<&dyn Snapshot, PublicKey, CandidateResult> {
        MapIndex::new("voteservice.results", self.view.as_ref())
    }

    /// Returns a specific candidate vote result data.
    pub fn candidate_result(&self, pub_key: &PublicKey) -> Option<CandidateResult> {
        self.vote_results().get(pub_key)
    }
}

/// A mutable version of schema.
impl<'a> VoteServiceSchema<&'a mut Fork> {
    /// Returns a mutable version of candidates table.
    pub fn candidates_mut(&mut self) -> MapIndex<&mut Fork, PublicKey, Candidate> {
        MapIndex::new("voteservice.candidates", &mut self.view)
    }

    /// Returns a mutable version of voter table.
    pub fn voters_mut(&mut self) -> MapIndex<&mut Fork, PublicKey, Voter> {
        MapIndex::new("voteservice.voters", &mut self.view)
    }

    /// Returns a mutable version of votes table.
    pub fn votes_mut(&mut self) -> MapIndex<&mut Fork, Hash, EncryptedVote> {
        MapIndex::new("voteservice.votes", &mut self.view)
    }

    /// Returns a mutable version of vote results table.
    pub fn vote_results_mut(&mut self) -> MapIndex<&mut Fork, PublicKey, CandidateResult> {
        MapIndex::new("voteservice.results", &mut self.view)
    }
}
