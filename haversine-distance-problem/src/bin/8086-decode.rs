#![allow(unused)]

use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Reg {
    Ax,
    Cx,
    Dx,
    Bx,
    Sp,
    Bp,
    Si,
    Di,
}

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Reg::Ax => "ax",
            Reg::Cx => "cx",
            Reg::Dx => "dx",
            Reg::Bx => "bx",
            Reg::Sp => "sp",
            Reg::Bp => "bp",
            Reg::Si => "si",
            Reg::Di => "di",
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum RegHalf {
    Al,
    Cl,
    Dl,
    Bl,
    Ah,
    Ch,
    Dh,
    Bh,
}

impl Display for RegHalf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            RegHalf::Al => "al",
            RegHalf::Cl => "cl",
            RegHalf::Dl => "dl",
            RegHalf::Bl => "bl",
            RegHalf::Ah => "ah",
            RegHalf::Ch => "ch",
            RegHalf::Dh => "dh",
            RegHalf::Bh => "bh",
        })
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum RegMemMode {
    MemNoDisp,
    Mem8bitDisp,
    Mem16bitDisp,
    RegNoDisp,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct EncInstruction {
    opcode_dw: u8,
    mod_reg_rm: u8,
}

impl EncInstruction {
    fn opcode(&self) -> u8 {
        self.opcode_dw >> 2
    }

    fn w(&self) -> bool {
        self.opcode_dw & 0b01 > 0
    }

    fn d(&self) -> bool {
        self.opcode_dw & 0b10 > 0
    }

    fn rmmod(&self) -> RegMemMode {
        match (self.mod_reg_rm >> 6) & 0b11 {
            0b00 => RegMemMode::MemNoDisp,
            0b01 => RegMemMode::Mem8bitDisp,
            0b10 => RegMemMode::Mem16bitDisp,
            0b11 => RegMemMode::RegNoDisp,
            _ => unreachable!(),
        }
    }

    fn reg(&self) -> Reg {
        Self::decode_reg(self.mod_reg_rm >> 3)
    }

    fn reg_half(&self) -> RegHalf {
        Self::decode_reg_half(self.mod_reg_rm >> 3)
    }

    fn rm(&self) -> Reg {
        Self::decode_reg(self.mod_reg_rm)
    }

    fn rm_half(&self) -> RegHalf {
        Self::decode_reg_half(self.mod_reg_rm)
    }

    fn decode_reg(byte: u8) -> Reg {
        match byte & 0b111 {
            0b000 => Reg::Ax,
            0b001 => Reg::Cx,
            0b010 => Reg::Dx,
            0b011 => Reg::Bx,
            0b100 => Reg::Sp,
            0b101 => Reg::Bp,
            0b110 => Reg::Si,
            0b111 => Reg::Di,
            _ => unreachable!(),
        }
    }

    fn decode_reg_half(byte: u8) -> RegHalf {
        match byte & 0b111 {
            0b000 => RegHalf::Al,
            0b001 => RegHalf::Cl,
            0b010 => RegHalf::Dl,
            0b011 => RegHalf::Bl,
            0b100 => RegHalf::Ah,
            0b101 => RegHalf::Ch,
            0b110 => RegHalf::Dh,
            0b111 => RegHalf::Bh,
            _ => unreachable!(),
        }
    }
}

enum Instruction {
    Mov(Reg, Reg),
    MovHalf(RegHalf, RegHalf),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Mov(dst, src) => write!(f, "mov {dst}, {src}"),
            Instruction::MovHalf(dst, src) => write!(f, "mov {dst}, {src}"),
        }
    }
}

fn decode(inst: EncInstruction) -> Instruction {
    match inst.opcode() {
        0b100010 => {
            // mov reg <- reg

            if inst.w() {
                // full registers

                let reg_dst;
                let reg_src;
                if inst.d() {
                    reg_dst = inst.reg();
                    reg_src = inst.rm();
                } else {
                    reg_src = inst.reg();
                    reg_dst = inst.rm();
                }

                Instruction::Mov(reg_dst, reg_src)
            } else {
                // half registers

                let reg_dst;
                let reg_src;
                if inst.d() {
                    reg_dst = inst.reg_half();
                    reg_src = inst.rm_half();
                } else {
                    reg_src = inst.reg_half();
                    reg_dst = inst.rm_half();
                }

                Instruction::MovHalf(reg_dst, reg_src)
            }
        }
        _ => panic!("unknown opcode: {:#b}", inst.opcode()),
    }
}

fn main() {
    let arg = std::env::args().nth(1);

    let Some(path) = arg else {
        panic!("usage: 8086-decode <path to compiled file>");
    };

    let file = File::open(path).unwrap();
    let mut file = BufReader::new(file);

    let mut inst_buf: [u8; 2] = [0; 2];
    while file.read_exact(&mut inst_buf).is_ok() {
        let inst = EncInstruction {
            opcode_dw: inst_buf[0],
            mod_reg_rm: inst_buf[1],
        };

        let inst = decode(inst);
        println!("{inst}");
    }
}
