use crate::Word;
use crate::memory::PeekPoke;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct DisplayRegisters {
    mode: u8,
    screen: u32,
    palette: u32,
    font: u32,
    height: u32,
    width: u32,
    row_offset: u32,
    col_offset: u32
}

struct VideoMode {
    gfx: bool,
    highres: bool,

}

impl Default for DisplayRegisters {
    fn default() -> Self {
        Self {
            mode: 7,
            screen: 0x10000,
            palette: 0x20000 - 0x100,
            font: 0x20000 - 0x100 - 0x2000,
            height: 60,
            width: 80,
            row_offset: 0,
            col_offset: 0
        }
    }
}

fn read_display_registers<P: PeekPoke>(machine: &P, start: Word) -> DisplayRegisters {
    DisplayRegisters {
        mode: machine.peek(start),
        screen: machine.peek24(start + 1),
        palette: machine.peek24(start + 4),
        font: machine.peek24(start + 7),
        height: machine.peek24(start + 10),
        width: machine.peek24(start + 13),
        row_offset: machine.peek24(start + 16),
        col_offset: machine.peek24(start + 19)
    }
}

fn init_display_registers<P: PeekPoke>(machine: &mut P, start: Word) {
    let dr = DisplayRegisters::default();
    machine.poke(start, dr.mode);
    machine.poke24(start + 1, dr.screen.into());
    machine.poke24(start + 4, dr.palette.into());
    machine.poke24(start + 7, dr.font.into());
    machine.poke24(start + 10, dr.height.into());
    machine.poke24(start + 13, dr.width.into());
    machine.poke24(start + 16, dr.row_offset.into());
    machine.poke24(start + 19, dr.col_offset.into());
}

pub fn draw<P: PeekPoke>(machine: &P, frame: &mut[u8]) {
    let reg = read_display_registers(machine, 16.into());
    let (gfx, highres, paletted) = (reg.mode & 1 > 0, reg.mode & 2 > 0, reg.mode & 4 > 0);

    match (gfx, highres, paletted) {
        (true, true, true) => draw_gfx_high_paletted(machine, reg, frame),
        (true, true, false) => draw_gfx_high_direct(machine, reg, frame),
        (_, _, _) => {}
    }
}

pub fn reset<P: PeekPoke>(machine: &mut P) {
    init_display_registers(machine, 16.into());
}

fn to_byte_address((x, y): (u32, u32), reg: DisplayRegisters) -> u32 {
    let row_start = (y + reg.row_offset % reg.height) * reg.width + reg.screen;
    ((x + reg.col_offset) % reg.width) + row_start
}

fn draw_gfx_high_direct<P: PeekPoke>(machine: &P, reg: DisplayRegisters, frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let (display_row, display_col) = (i / 640, i % 640);
        let (vulcan_row, vulcan_col) = ((display_row >> 2) as u32, (display_col >> 2) as u32);

        let vb = machine.peek(Word::from(to_byte_address((vulcan_col, vulcan_row), reg)));
        let (red, green, blue) = (vb >> 5, (vb >> 3) & 7, (vb & 3) << 1);

        pixel[0] = red << 5;
        pixel[1] = green << 5;
        pixel[2] = blue << 5;
        pixel[3] = 0xff;
    }
}

fn draw_gfx_high_paletted<P: PeekPoke>(machine: &P, reg: DisplayRegisters, frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let (display_row, display_col) = (i / 640, i % 640);
        let (vulcan_row, vulcan_col) = ((display_row >> 2) as u32, (display_col >> 2) as u32);

        let addr = to_byte_address((vulcan_col, vulcan_row), reg);
        let color_idx = machine.peek(Word::from(addr));
        let color = machine.peek(Word::from(reg.palette + color_idx as u32));
        let (red, green, blue) = (color >> 5, (color >> 3) & 7, (color & 3) << 1);

        pixel[0] = red << 5;
        pixel[1] = green << 5;
        pixel[2] = blue << 5;
        pixel[3] = 0xff;
    }
}