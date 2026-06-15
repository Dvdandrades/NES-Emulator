pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod opcodes;
pub mod ppu;
pub mod trace;

use bus::Bus;
use cartridge::Rom;
use cpu::CPU;
use trace::trace;

use sdl2::pixels::PixelFormatEnum;

fn main() {
    // Init sdl2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(
            "Snake game",
            (32.0 * 10.0/* scale factor */) as u32,
            (32.0 * 10.0/* scale factor */) as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let _event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(10.0, 10.0).unwrap();

    let creator = canvas.texture_creator();
    let _texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 32, 32)
        .unwrap();

    // Load the game
    let bytes: Vec<u8> = std::fs::read("nestest.nes").unwrap();
    let rom = Rom::new(&bytes).unwrap();
    let bus = Bus::new(rom);
    let mut cpu = CPU::new(bus);
    cpu.reset();
    cpu.program_counter = 0xC000;

    // Run the game cycle
    cpu.run_with_callback(move |cpu| {
        println!("{}", trace(cpu));
    });
}
