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
