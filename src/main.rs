mod memory;
mod address;
mod opcodes;
mod cpu;
mod bus;

use winit::{
    event::{ Event, WindowEvent },
    event_loop::{ EventLoop, ControlFlow },
    window::WindowBuilder,
    dpi::LogicalSize
};

use pixels::{Pixels, SurfaceTexture};
use rand::RngCore;
use std::time::Instant;

fn main() {
    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new(640, 480);
        WindowBuilder::new()
            .with_title("Vulcan")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let surface_texture = SurfaceTexture::new(640, 480, &window);
        Pixels::new(640, 480, surface_texture).unwrap()
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit
            }
            Event::MainEventsCleared => {
                let start = Instant::now();
                draw(pixels.get_frame());
                let draw_time = Instant::now() - start;
                pixels.render().expect("Problem displaying framebuffer");
                let total_time = Instant::now() - start;
                println!("Tick took {} total, {} to draw", total_time.as_micros(), draw_time.as_micros());
            }
            _ => {}
        }
    })
}

fn draw(frame: &mut [u8]) {
    assert_eq!(frame.len(), 640 * 480 * 4);
    let mut rng = rand::thread_rng();

    for (_, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let p = rng.next_u32();
        let [low, mid, high, _] = p.to_le_bytes();
        pixel[0] = low;
        pixel[1] = mid;
        pixel[2] = high;
        pixel[3] = 0xff;
    }
}