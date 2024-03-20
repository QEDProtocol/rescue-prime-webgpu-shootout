use miden_crypto::{hash::rpo::Rpo256, merkle::MerkleTree, Felt};

pub fn hash_leaves_cpu(leaves: &[[Felt; 4]]) -> MerkleTree{
  MerkleTree::new(&leaves).unwrap()
}