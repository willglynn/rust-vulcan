mod memory;
mod word;
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
use winit::window::Window;
use crate::cpu::CPU;
use crate::memory::{ Memory, PeekPoke };
use crate::word::Word;

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

    let mut rng = rand::thread_rng();

    let memory = Memory::from(rng);
    let cpu = CPU::new(memory);
    window_loop(event_loop, window, pixels, cpu)
}

fn window_loop(event_loop: EventLoop<()>, window: Window, mut pixels: Pixels, mut cpu: CPU) -> ! {
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
                draw(pixels.get_frame(), &mut cpu);
                let draw_time = Instant::now() - start;
                pixels.render().expect("Problem displaying framebuffer");
                let total_time = Instant::now() - start;
                //println!("Tick took {} total, {} to draw", total_time.as_micros(), draw_time.as_micros());
            }
            _ => {}
        }
    })
}

fn draw(frame: &mut [u8], cpu: &mut CPU) {
    assert_eq!(frame.len(), 640 * 480 * 4);

    // For now, assume 160x120 direct graphics mode
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let (display_row, display_col) = (i / 640, i % 640);
        let (vulcan_row, vulcan_col) = (display_row >> 2, display_col >> 2);
        let vb = cpu.peek(Word::from((0x10000 + vulcan_row * 160 + vulcan_col) as u32));
        let (red, green, blue) = (vb >> 5, (vb >> 3) & 7, (vb & 3) << 1);

        pixel[0] = red << 5;
        pixel[1] = green << 5;
        pixel[2] = blue << 5;
        pixel[3] = 0xff;
    }
}