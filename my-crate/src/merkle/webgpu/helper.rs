
pub struct WebGpuHelper {
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub module: wgpu::ShaderModule,
}
const DISPATCH_MAX_PER_DIM: u64 = 32768u64;

pub fn get_dispatch_linear(size: u64) -> (u32, u32, u32) {
  if (size <= DISPATCH_MAX_PER_DIM) {
      return (size as u32, 1, 1);
  } else if (size <= DISPATCH_MAX_PER_DIM * DISPATCH_MAX_PER_DIM) {
      assert_eq!(size % DISPATCH_MAX_PER_DIM, 0);
      return (DISPATCH_MAX_PER_DIM as u32, (size / DISPATCH_MAX_PER_DIM) as u32, 1);
  } else if (size <= DISPATCH_MAX_PER_DIM * DISPATCH_MAX_PER_DIM * DISPATCH_MAX_PER_DIM) {
      assert_eq!(size % (DISPATCH_MAX_PER_DIM * DISPATCH_MAX_PER_DIM), 0);
      return (
          DISPATCH_MAX_PER_DIM as u32,
          DISPATCH_MAX_PER_DIM as u32,
          (size / (DISPATCH_MAX_PER_DIM * DISPATCH_MAX_PER_DIM)) as u32,
      );
  } else {
      panic!("size too large for dispatch");
  }
}

impl WebGpuHelper {
  pub fn new() -> Self {
      let p = futures::executor::block_on(Self::new_async());
      p.unwrap()
  }
  pub async fn new_async() -> Option<Self> {
      // Instantiates instance of WebGPU
      let instance = wgpu::Instance::default();

      // `request_adapter` instantiates the general connection to the GPU
      let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await?;

      // `request_device` instantiates the feature specific connection to the GPU, defining some parameters,
      //  `features` being the available features.
      let (device, queue) = adapter
          .request_device(
              &wgpu::DeviceDescriptor {
                  label: None,
                  required_features: wgpu::Features::empty(),
                  required_limits: wgpu::Limits::downlevel_defaults(),
              },
              None,
          )
          .await
          .unwrap();

      let cs_module = device.create_shader_module(/*wgpu::ShaderModuleDescriptor {
          label: None,
          source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("hsh.wgsl"))),
      }*/ wgpu::include_wgsl!("shader.wgsl"));

      Some(Self {
          device,
          queue,
          module: cs_module,
      })
  }
}

impl Default for WebGpuHelper {
  fn default() -> Self {
      WebGpuHelper::new()
  }
}
unsafe impl Send for WebGpuHelper{}
unsafe impl Sync for WebGpuHelper{}