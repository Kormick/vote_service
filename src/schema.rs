use exonum::{
    crypto::{Hash, PublicKey},
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
    struct EncryptedVote {
        data: Vec<u8>,
    }
}

encoding_struct! {
    struct DecryptedCandidateResult {
        candidate: &PublicKey,
        votes: Vec<Vote>,
        vote_num: u64
    }
}

encoding_struct! {
    struct CandidateResult {
        candidate: &PublicKey,
        votes: Vec<EncryptedVote>,
        vote_num: u64
    }
}

encoding_struct! {
    struct VoteResult {
        pub_key: &PublicKey,
        candidate_results: Vec<CandidateResult>,
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

    pub fn votes(&self) -> MapIndex<&dyn Snapshot, Hash, EncryptedVote> {
        MapIndex::new("voteservice.votes", self.view.as_ref())
    }

    pub fn vote(&self, hash: &Hash) -> Option<EncryptedVote> {
        self.votes().get(hash)
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

    pub fn votes_mut(&mut self) -> MapIndex<&mut Fork, Hash, EncryptedVote> {
        MapIndex::new("voteservice.votes", &mut self.view)
    }

    pub fn vote_results_mut(&mut self) -> MapIndex<&mut Fork, PublicKey, CandidateResult> {
        MapIndex::new("voteservice.results", &mut self.view)
    }
}
