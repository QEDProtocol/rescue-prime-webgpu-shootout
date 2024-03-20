//@ts-ignore
import init, { greet,bench_merkle_gpu,bench_merkle_cpu } from 'my-crate';
// Don't worry if vscode told you can't find my-crate
// It's because you're using a local crate
// after yarn dev, wasm-pack plugin will install my-crate for you

init().then(() => {
  console.log('init wasm-pack');
  bench_merkle_gpu(18).then((result) => {
    return bench_merkle_cpu(18);
  }).catch(err=>console.error("ERROR: ",err));
});
