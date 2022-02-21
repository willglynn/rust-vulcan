mod memory;
mod word;
mod opcodes;
mod cpu;
mod bus;
mod display;

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
    let mut cpu = CPU::new(memory);
    display::reset(&mut cpu);
    for n in 0..256 {
        let color = ((n / 32) << 3) as u8;
        cpu.poke(Word::from(0x20000 - 0x100 + n as u32), color);
    }
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

    display::draw(cpu, frame);
}