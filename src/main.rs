#![allow(unused)]
use rt::run;
use wgpu::Backends;

fn main() {
    let instances = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: Backends::all(),
        ..Default::default()
    });
    for adapter in instances.enumerate_adapters(wgpu::Backends::all()) {
        println!("{:?}", adapter.get_info());
    }

    run();
}
