#![allow(dead_code)]

use std::fmt;
use std::fmt::Display;
use std::{mem, ptr};
// Types

// 32 bit Virtual Address
pub type VAddr = u32;
// 32 bit Physical Address
pub type PAddr = u32;
// Virtual page numbers can be up to 16 bits.
pub type VPN = u16;
// Physical frame numbers can be up to 16 bits.
pub type PFN = u16;
// Byte addressable machine, so u8 will suffice.
pub type Word = u8;
// Swap Entry
pub type SwapEntry = u64;
// Timestamp used for FIFO eviction
pub type TimeStamp = u32;

// Constants
pub const MAX_PID: u32 = 800;
pub const PADDR_LEN: u32 = 20;
pub const VADDR_LEN: u32 = 24;
pub const OFFSET_LEN: u32 = 14;
pub const MEMORY_ACCESS_TIME: u32 = 200;
pub const DISK_ACCESS_TIME: u64 = 10000000;

pub const PAGE_SIZE: u64 = (1 << OFFSET_LEN);

pub const MEM_SIZE: u64 = (1 << PADDR_LEN);

pub const NUM_PAGES: u64 = (1 << (VADDR_LEN - OFFSET_LEN));
pub const NUM_FRAMES: u64 = (1 << (PADDR_LEN - OFFSET_LEN));



// Structs
#[derive(Copy, Clone, Debug)]
pub struct Process {
    pub pid: u32,
    pub state: ProcessState,
    pub saved_ptbr: PFN,
}

#[derive(Copy, Clone, Debug)]
pub struct PageTableEntry {
    pub valid: bool,
    pub dirty: bool,
    pub pfn: PFN,
    pub swap_entry: SwapEntry,
}

#[derive(Copy, Clone, Debug)]
pub struct FrameTableEntry {
    pub protected: bool,
    pub mapped: bool,
    pub process: Option<Process>,
    pub vpn: VPN,
}

// Physical memory. Has a replacement algorithm and mem as an array of frames
pub struct Memory {
    pub mem: Box<[Frame; NUM_FRAMES as usize]>,
    pub replacement: Replacement,
}


// Enums

// Represents a Frame in memory.
pub enum Frame {
    Data([u8; PAGE_SIZE as usize]),
    FrameTable(Box<[FrameTableEntry]>),
    PageTable(Box<[PageTableEntry]>),
    UNINITIALIZED
}

// Possible replacement types.
#[derive(Copy, Clone, Debug)]
pub enum Replacement {
    RANDOM,
    FIFO,
    CLOCKSWEEP,
    LRU
}

#[derive(Copy, Clone, Debug)]
pub enum ProcessState {
    RUNNING,
    STOPPED,
    UNINITIALIZED
}


// Impls

impl Process {
    pub fn new(pid: u32, state: ProcessState) -> Process {
        Process {
            pid: pid,
            state: state,
            saved_ptbr: 0,
        }
    }

    pub fn default() -> Process {
        Process {
            pid: 0,
            state: ProcessState::UNINITIALIZED,
            saved_ptbr: 0,
        }
    }
}

impl Memory {
    pub fn new(replacement: Replacement) -> Memory {
        let mem = unsafe {
            let mut mem: [Frame; NUM_FRAMES as usize] = mem::uninitialized();

            for (i, element) in mem.iter_mut().enumerate() {
                let frame = Frame::UNINITIALIZED;
                ptr::write(element, frame)
            }
            mem
        };

        Memory {
            mem: box mem,
            replacement: replacement,
        }
    }
}

impl PageTableEntry {
    pub fn new() -> PageTableEntry {
        PageTableEntry {
            valid: false,
            dirty: false,
            pfn: 0,
            swap_entry: 0
        }
    }
}

impl FrameTableEntry {
    pub fn new() -> FrameTableEntry {
        FrameTableEntry {
            protected: false,
            mapped: false, 
            process: None,
            vpn: 0
        }
    }
}
 



impl Display for Replacement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let print_val = match self {
            Replacement::RANDOM => "random",
            Replacement::FIFO => "fifo",
            Replacement::CLOCKSWEEP => "clocksweep",
            Replacement::LRU => "lru"
        };
        write!(f, "{}", print_val)
    }
}

