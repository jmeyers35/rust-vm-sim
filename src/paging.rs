use std::{mem, ptr};
use crate::defs::*;
use Frame::*;
// System init. Takes in a Memory struct (which is the system). Sets up the frame table.
pub fn system_init(mem: &mut Memory) {

    // Create an array of FTEs. Don't like the unsafe here but we'll roll with it for now

    // let ftes = unsafe {
    //     let mut ftes: [FrameTableEntry; NUM_FRAMES as usize] = mem::uninitialized();

    //     for (i, element) in ftes.iter_mut().enumerate() {
    //         let entry = FrameTableEntry::new();

    //         ptr::write(element, entry);
    //     }
    //     ftes
    // };

    let mut ftes: Box<[FrameTableEntry]> = vec![FrameTableEntry::new(); NUM_FRAMES as usize].into_boxed_slice();
    // Go ahead and mark first entry as protected
    ftes[0].protected = true;
    // Create frame table
    let frame_table = FrameTable(ftes);
    // Set first frame to the frame table
    mem.mem[0] = frame_table;
}

mod tests {
    extern crate arrayvec;
    use Replacement::RANDOM;
    use super::*;

    #[test]
    fn test_system_init() -> Result<(), String> {
        let mut test_mem = Memory::new(RANDOM);
        system_init(&mut test_mem);

        // Test that the first physical frame in memory is the frame table
        if let FrameTable(entries) = &test_mem.mem[0] {
            // Ensure the frame table is protected
            assert!(entries[0].protected)
        } else {
            return Err(String::from("First frame in memory is not the frame table!"));
        }
        Ok(())

    }

}