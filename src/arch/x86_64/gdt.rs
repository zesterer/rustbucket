//gdt.rs

//defines and provides methods for the Global Descriptor Table, for use in long mode.
//a table has already been defined for protected mode, in boot.asm

use core::mem::size_of;

const GDT_LENGTH: usize = 3;

//contains the structure of a gdt entry
#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct GdtEntry {
    //limit: size of entry
    limit_low: u16,
    //base: offset in memory of entry
    base_low: u16,
    base_middle: u8,
    //determine the access level of the segment
    access: u8,
    //determine the granularity of the segment
    //rest of limit stored in granularity byte
    granularity: u8,
    base_high: u8
}

//various binary flags that appear in the access field of a gdt entry
//they determine the properties of the entry, and how data is manipulated/accessed
enum AccessFlags {
    //for code, indicate it is readable
    //for data, indicate it is writable
    ReadWrite = 0b00000010,
    Executable = 0b00001000, //indicate a code segment
    Present = 0b10000000, //indicate a valid sector
    One = 0b00010000 //self-explanatory -- always set
}

//various binary flags that appear in the granularity field of a gdt entry
enum GranularityFlags {
    Page = 0b1000,
    LongMode_64 = 0b0010
}

//contains the pointer to the gdt that must be passed to assembly
//will be aligned to the largest item in the struct (4 bytes), as per Rust implementation
#[repr(packed)]
pub struct GdtPointer {
    pub limit: u16,
    pub base: u64
}

//set a static variable containing the GDT pointer
//we use a static variable, since we can find its location in memory with "VAR".as_ptr()
pub static mut GDT_POINTER: GdtPointer = GdtPointer {
        limit: 0,
        base: 0
};

//set a static variable containing the GDT
//we use a static variable, since we can find its location in memory with "VAR".as_ptr()
pub static mut GDT: [GdtEntry; 3] = [
    //initialise with null entries, since Rust does not support forward declaration
    GdtEntry {
        base_low: 0,
        base_middle: 0,
        base_high: 0,
        limit_low: 0,
        granularity: 0,
        access: 0
    }, GdtEntry {
        base_low: 0,
        base_middle: 0,
        base_high: 0,
        limit_low: 0,
        granularity: 0,
        access: 0
    }, GdtEntry {
        base_low: 0,
        base_middle: 0,
        base_high: 0,
        limit_low: 0,
        granularity: 0,
        access: 0
    }
];

impl GdtEntry {
    pub fn set_up(base_in: u32, limit_in: u32, access_in: u8, gran_in: u8) -> GdtEntry {
        let temp_flags: u8 = ((limit_in >> 16) & 0x0F) as u8;
        let flags: u8 = temp_flags | ((gran_in << 4) &0x0F) as u8;

        GdtEntry {
            //set the base (offset) of the entry
            base_low: ((base_in >> 0) & 0xFFFF) as u16,
            base_middle: ((base_in >> 16) & 0xFF) as u8,
            base_high: ((base_in >> 24) & 0xFF) as u8,

            //set the size of the entry
            limit_low: (limit_in & 0xFFFF) as u16,
            granularity: flags,

            //set the access level of the entry
            access: access_in
        }
    }
}

pub fn gdt_init() {
    //set access flags for code segments
    let code_flags: u8 =
        AccessFlags::ReadWrite as u8 |
        AccessFlags::Executable as u8 |
        AccessFlags::One as u8 |
        AccessFlags::Present as u8;
    //set access flags for data segments
    let data_flags: u8 =
        AccessFlags::ReadWrite as u8 |
        AccessFlags::One as u8 |
        AccessFlags::Present as u8;
    //set granularity flags, indicate a 64-bit table
    let granularity_flags: u8 =
        GranularityFlags::Page as u8 |
        GranularityFlags::LongMode_64 as u8;

    //set up gdt entries
    unsafe {
        GDT[0] = GdtEntry::set_up(0, 0, 0, 0);
        GDT[1] = GdtEntry::set_up(0, 0xFFFFF, code_flags, granularity_flags);
        GDT[2] = GdtEntry::set_up(0, 0xFFFFF, data_flags, granularity_flags);
    }

    //set up gdt pointer structure
    unsafe {
        GDT_POINTER.limit = (GDT_LENGTH as u16 * size_of::<GdtEntry>() as u16) - 1;
        GDT_POINTER.base = GDT.as_ptr() as u64;
    }

    //set gdt to cpu
    unsafe { gdt_install(&GDT_POINTER); }
}

unsafe fn gdt_install(gdt: &GdtPointer) {
    asm!("lgdt ($0)" :: "r" (gdt) : "memory");
}
