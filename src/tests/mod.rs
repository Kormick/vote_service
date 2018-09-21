use exonum::crypto::{self, PublicKey, SecretKey};
use exonum::storage::StorageValue;
use exonum_testkit::{ApiKind, TestKit, TestKitBuilder};

use std::borrow::Cow;

use agreement;
use api::BlockQuery;
use cipher::{self, Cipher};
use config::VoteServiceConfig;
use schema::{
    Candidate, DecryptedCandidateResult, EncryptedVote, Vote, VoteResult, VoteServiceSchema, Voter,
};
use transactions::{TxAddVote, TxCreateCandidate, TxCreateVoter};
use VoteService;

#[test]
fn test_create_candidate() {
    let (mut testkit, _) = init_testkit();
    let (tx, _) = create_candidate(&mut testkit, "Alice", "Some info");

    let candidate = get_candidate(&testkit, tx.pub_key());

    assert_eq!(candidate.pub_key(), tx.pub_key());
    assert_eq!(candidate.name(), "Alice");
    assert_eq!(candidate.info(), "Some info");
}

#[test]
fn test_create_voter() {
    let (mut testkit, _) = init_testkit();
    let (tx, _) = create_voter(&mut testkit, "Bob");

    let voter = get_voter(&testkit, tx.pub_key());

    assert_eq!(voter.pub_key(), tx.pub_key());
    assert_eq!(voter.name(), "Bob");
}

#[test]
fn test_get_results() {
    let (mut testkit, key_pair) = init_testkit();

    let (cand_tx, _) = create_candidate(&mut testkit, "Alice", "Some info");
    let cand = get_candidate(&testkit, cand_tx.pub_key());

    let (voter_tx, _) = create_voter(&mut testkit, "Bob");
    let voter = get_voter(&testkit, voter_tx.pub_key());

    add_vote(&mut testkit, voter.pub_key(), cand.pub_key());

    let enc_result = get_vote_result(&testkit);
    assert_eq!(enc_result.candidate_results().len(), 1);

    let service_pub_key = enc_result.pub_key();
    let ephemeral = agreement::generate_ephemeral(service_pub_key.as_ref(), key_pair.secret);

    let cand_res = &enc_result.candidate_results()[0];
    assert_eq!(cand_res.candidate(), cand.pub_key());
    assert_eq!(cand_res.votes().len(), 1);
    assert_eq!(cand_res.vote_num(), 1);

    let enc_vote = &cand_res.votes()[0];
    let dec_vote = decrypt_vote(&enc_vote, &ephemeral.ephemeral_key);
    assert_eq!(dec_vote.from(), voter.pub_key());
    assert_eq!(dec_vote.to(), cand.pub_key());
}

#[test]
fn test_get_decrypted_results() {
    let (mut testkit, _) = init_testkit();

    let (cand_tx, _) = create_candidate(&mut testkit, "Alice", "Some info");
    let cand = get_candidate(&testkit, cand_tx.pub_key());

    let (voter_tx, _) = create_voter(&mut testkit, "Bob");
    let voter = get_voter(&testkit, voter_tx.pub_key());

    add_vote(&mut testkit, voter.pub_key(), cand.pub_key());

    let dec_result = get_vote_result_decrypted(&testkit);

    assert_eq!(dec_result.len(), 1);

    let cand_res = &dec_result[0];
    assert_eq!(cand_res.candidate(), cand.pub_key());
    assert_eq!(cand_res.votes().len(), 1);
    assert_eq!(cand_res.vote_num(), 1);

    let vote = &cand_res.votes()[0];
    assert_eq!(vote.from(), voter.pub_key());
    assert_eq!(vote.to(), cand.pub_key());
}

#[test]
fn test_get_block() {
    let (mut testkit, _) = init_testkit();

    let (cand_tx, _) = create_candidate(&mut testkit, "Alice", "Some info");
    let cand = get_candidate(&testkit, cand_tx.pub_key());

    let (voter_tx, _) = create_voter(&mut testkit, "Bob");
    let voter = get_voter(&testkit, voter_tx.pub_key());

    let (_, height, _) = add_vote(&mut testkit, voter.pub_key(), cand.pub_key());

    let block_height = get_block(&testkit, voter.pub_key());

    assert_eq!(height, block_height);
}

fn init_testkit() -> (TestKit, agreement::KeyPair) {
    let author_key_pair = agreement::generate_key_pair();

    let cfg = VoteServiceConfig {
        author_public_key: Some(PublicKey::from_slice(&author_key_pair.public.clone()).unwrap()),
    };

    agreement::init_ephemeral(&author_key_pair.public.clone());

    (
        TestKitBuilder::validator()
            .with_service(VoteService { config: cfg })
            .create(),
        author_key_pair,
    )
}

fn create_candidate(
    testkit: &mut TestKit,
    name: &str,
    info: &str,
) -> (TxCreateCandidate, SecretKey) {
    let (public, secret) = crypto::gen_keypair();
    let tx = TxCreateCandidate::new(&public, name, info, &secret);
    testkit.create_block_with_transaction(tx.clone());

    (tx, secret)
}

fn try_get_candidate(testkit: &TestKit, pub_key: &PublicKey) -> Option<Candidate> {
    let snapshot = testkit.snapshot();
    VoteServiceSchema::new(&snapshot).candidate(pub_key)
}

fn get_candidate(testkit: &TestKit, pub_key: &PublicKey) -> Candidate {
    try_get_candidate(testkit, pub_key).expect("Candidate not found")
}

fn create_voter(testkit: &mut TestKit, name: &str) -> (TxCreateVoter, SecretKey) {
    let (public, secret) = crypto::gen_keypair();
    let tx = TxCreateVoter::new(&public, name, &secret);
    testkit.create_block_with_transaction(tx.clone());

    (tx, secret)
}

fn try_get_voter(testkit: &TestKit, pub_key: &PublicKey) -> Option<Voter> {
    let snapshot = testkit.snapshot();
    VoteServiceSchema::new(&snapshot).voter(pub_key)
}

fn get_voter(testkit: &TestKit, pub_key: &PublicKey) -> Voter {
    try_get_voter(testkit, pub_key).expect("Voter not found")
}

fn add_vote(
    testkit: &mut TestKit,
    from: &PublicKey,
    to: &PublicKey,
) -> (TxAddVote, u64, SecretKey) {
    let (public, secret) = crypto::gen_keypair();
    let tx = TxAddVote::new(&public, from, to, &secret);
    let block = testkit.create_block_with_transaction(tx.clone());

    (tx, block.height().0, secret)
}

fn get_vote_result(testkit: &TestKit) -> VoteResult {
    let api = testkit.api();

    let entry: Option<VoteResult> = api
        .public(ApiKind::Service("voteservice"))
        .get("v1/results")
        .unwrap();

    entry.unwrap()
}

fn get_vote_result_decrypted(testkit: &TestKit) -> Vec<DecryptedCandidateResult> {
    let api = testkit.api();

    let entry: Option<Vec<DecryptedCandidateResult>> = api
        .public(ApiKind::Service("voteservice"))
        .get("v1/results_dec")
        .unwrap();

    entry.unwrap()
}

fn get_block(testkit: &TestKit, pub_key: &PublicKey) -> u64 {
    let api = testkit.api();

    let entry: Option<u64> = api
        .public(ApiKind::Service("voteservice"))
        .query(&BlockQuery { pub_key: *pub_key })
        .get("v1/block")
        .unwrap();

    entry.unwrap()
}

fn decrypt_vote(vote: &EncryptedVote, key: &Vec<u8>) -> Vote {
    let mut dec = cipher::CipherChaChaPoly::default();
    dec.set(&key);

    let mut dec_output = [0u8; 128];
    let dec_size = dec.decrypt(0, &[], &vote.data(), &mut dec_output).unwrap();
    let dec_output = &dec_output[..dec_size];

    let vote = Vote::from_bytes(Cow::Borrowed(dec_output));

    vote
}
