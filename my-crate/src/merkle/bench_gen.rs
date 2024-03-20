use miden_crypto::{hash::rpo::Rpo256, Felt};

pub fn gen_leaves(n: usize) -> Vec<[Felt; 4]>{
  (0..n).map(|f| [Felt::new(n as u64); 4]).collect()
}



