extern crate cfg_if;
extern crate wasm_bindgen;

mod merkle;
mod utils;
use cfg_if::cfg_if;
use merkle::webgpu::helper::WebGpuHelper;
use wasm_bindgen::prelude::*;

use crate::{merkle::cpu::hash_leaves_cpu, utils::set_panic_hook};

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello,{}!", name));
}

#[wasm_bindgen]
pub async fn bench_merkle_gpu(height: usize) -> u32 {
    set_panic_hook();
    let window = web_sys::window().expect("should have a window in this context");
    let performance = window
        .performance()
        .expect("performance should be available");

    let helper = WebGpuHelper::new_async().await.unwrap();
    let leaves = merkle::bench_gen::gen_leaves(1 << height);

    let start = performance.now();
    let result = merkle::webgpu::merkle::web_gpu_generate_merkle_tree(&helper, &leaves).await;
    let rr = result.unwrap();

    let end = performance.now();

    console_log!("webgpu finished in {}ms", (end - start));
    console_log!(
        "webgpu result - height = {}, root [{},{},{},{}]",
        height,
        rr[1][0].inner(),
        rr[1][1].inner(),
        rr[1][2].inner(),
        rr[1][3].inner()
    );
    //[(result[0].inner()&0xffffffffu64) as u32, (result[0].inner()>>32u64) as u32, (result[1].inner()&0xffffffffu64) as u32, (result[1].inner()>>32u64) as u32, (result[2].inner()&0xffffffffu64) as u32, (result[2].inner()>>32u64) as u32, (result[3].inner()&0xffffffffu64) as u32, (result[3].inner()>>32u64) as u32]
    1
}

#[wasm_bindgen]
pub async fn bench_merkle_cpu(height: usize) -> u32 {
    let window = web_sys::window().expect("should have a window in this context");
    let performance = window
        .performance()
        .expect("performance should be available");

    let leaves = merkle::bench_gen::gen_leaves(1 << height);

    let start = performance.now();
    let mt = hash_leaves_cpu(&leaves);
    let rt = mt.root();
    let rr = rt.as_elements();

    let end = performance.now();

    console_log!("cpu finished in {}ms", (end - start));
    console_log!(
        "cpu result - height = {}, root [{},{},{},{}]",
        height,
        rr[0].inner(),
        rr[1].inner(),
        rr[2].inner(),
        rr[3].inner()
    );

    //[(result[0].inner()&0xffffffffu64) as u32, (result[0].inner()>>32u64) as u32, (result[1].inner()&0xffffffffu64) as u32, (result[1].inner()>>32u64) as u32, (result[2].inner()&0xffffffffu64) as u32, (result[2].inner()>>32u64) as u32, (result[3].inner()&0xffffffffu64) as u32, (result[3].inner()>>32u64) as u32]
    0
}
