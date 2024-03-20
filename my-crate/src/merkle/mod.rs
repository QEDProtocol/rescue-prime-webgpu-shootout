use miden_crypto::Felt;

use self::webgpu::helper::WebGpuHelper;

pub mod cpu;
pub mod webgpu;
pub mod bench_gen;

pub async fn bench_merkle_cpu(height: usize) -> [Felt; 4]{
  let leaves = bench_gen::gen_leaves(1 << height);
  let tree: miden_crypto::merkle::MerkleTree = cpu::hash_leaves_cpu(&leaves);
  let root_elements = tree.root();
  let elements: &[Felt] = root_elements.as_elements();
  [elements[0], elements[1], elements[2], elements[3]]
}




pub async fn bench_merkle_gpu(helper: &WebGpuHelper, height: usize) -> [Felt; 4]{
  let leaves = bench_gen::gen_leaves(1 << height);
  webgpu::merkle::web_gpu_generate_merkle_tree(&helper, &leaves).await.unwrap()[1]
}

