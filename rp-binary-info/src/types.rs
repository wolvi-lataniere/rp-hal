//! Types for the Binary Info system

use crate::consts::{GpioFunction, TAG_RASPBERRY_PI};

/// This is the 'Binary Info' header block that `picotool` looks for in your UF2
/// file/ELF file/Pico in Bootloader Mode to give you useful metadata about your
/// program.
///
/// It should be placed in the first 4096 bytes of flash, so use your `memory.x`
/// to insert a section between `.text` and `.vector_table` and put a static
/// value of this type in that section.
#[repr(C)]
pub struct Header {
    /// Must be equal to Picotool::MARKER_START
    marker_start: u32,
    /// The first in our table of pointers to Entries
    entries_start: *const EntryAddr,
    /// The last in our table of pointers to Entries
    entries_end: *const EntryAddr,
    /// The first entry in a null-terminated RAM/Flash mapping table
    mapping_table: *const MappingTableEntry,
    /// Must be equal to Picotool::MARKER_END
    marker_end: u32,
}

impl Header {
    /// This is the `BINARY_INFO_MARKER_START` magic value from `picotool`
    const MARKER_START: u32 = 0x7188ebf2;
    /// This is the `BINARY_INFO_MARKER_END` magic value from `picotool`
    const MARKER_END: u32 = 0xe71aa390;

    /// Create a new `picotool` compatible header.
    ///
    /// * `entries_start` - the first [`EntryAddr`] in the table
    /// * `entries_end` - the last [`EntryAddr`] in the table
    /// * `mapping_table` - the RAM/Flash address mapping table
    pub const fn new(
        entries_start: *const EntryAddr,
        entries_end: *const EntryAddr,
        mapping_table: &'static [MappingTableEntry],
    ) -> Self {
        let mapping_table = mapping_table.as_ptr();
        Self {
            marker_start: Self::MARKER_START,
            entries_start,
            entries_end,
            mapping_table,
            marker_end: Self::MARKER_END,
        }
    }
}

// We need this as rustc complains that is is unsafe to share `*const u32`
// pointers between threads. We only allow these to be created with static
// data, so this is OK.
unsafe impl Sync for Header {}

/// This is a reference to an entry. It's like a `&dyn` ref to some type `T:
/// Entry`, except that the run-time type information is encoded into the
/// Entry itself in very specific way.
#[repr(transparent)]
pub struct EntryAddr(*const u32);

// We need this as rustc complains that is is unsafe to share `*const u32`
// pointers between threads. We only allow these to be created with static
// data, so this is OK.
unsafe impl Sync for EntryAddr {}

/// Allows us to tell picotool where values are in the UF2 given their run-time
/// address.
///
/// The most obvious example is RAM variables, which must be found in the
/// `.data` section of the UF2.
#[repr(C)]
pub struct MappingTableEntry {
    /// The start address in RAM (or wherever the address picotool finds will
    /// point)
    pub source_addr_start: *const u32,
    /// The start address in flash (or whever the data actually lives in the
    /// ELF)
    pub dest_addr_start: *const u32,
    /// The end address in flash
    pub dest_addr_end: *const u32,
}

impl MappingTableEntry {
    /// Generate a null entry to mark the end of the list
    pub const fn null() -> MappingTableEntry {
        MappingTableEntry {
            source_addr_start: core::ptr::null(),
            dest_addr_start: core::ptr::null(),
            dest_addr_end: core::ptr::null(),
        }
    }
}

// We need this as rustc complains that is is unsafe to share `*const u32`
// pointers between threads. We only allow these to be created with static
// data, so this is OK.
unsafe impl Sync for MappingTableEntry {}

/// This is the set of data types that `picotool` supports.
#[repr(u16)]
pub enum DataType {
    /// Raw data
    Raw = 1,
    /// Data with a size
    SizedData = 2,
    /// A list of binary data
    BinaryInfoListZeroTerminated = 3,
    /// A BSON encoded blob
    Bson = 4,
    /// An Integer with an ID
    IdAndInt = 5,
    /// A string with an Id
    IdAndString = 6,
    /// A block device
    BlockDevice = 7,
    /// GPIO pins, with their function
    PinsWithFunction = 8,
    /// GPIO pins, with their name
    PinsWithName = 9,
    /// GPIO pins, with multiple names?
    PinsWithNames = 10,
}

/// All Entries start with this common header
#[repr(C)]
struct EntryCommon {
    data_type: DataType,
    tag: u16,
}

/// An entry which contains both an ID (e.g. `ID_RP_PROGRAM_NAME`) and a pointer
/// to a null-terminated string.
#[repr(C)]
pub struct StringEntry {
    header: EntryCommon,
    id: u32,
    value: *const core::ffi::c_char,
}

impl StringEntry {
    /// Create a new `StringEntry`
    pub const fn new(tag: u16, id: u32, value: &'static core::ffi::CStr) -> StringEntry {
        StringEntry {
            header: EntryCommon {
                data_type: DataType::IdAndString,
                tag,
            },
            id,
            value: value.as_ptr(),
        }
    }

    /// Get this entry's address
    pub const fn addr(&self) -> EntryAddr {
        EntryAddr(self as *const Self as *const u32)
    }
}

// We need this as rustc complains that is is unsafe to share `*const
// core::ffi::c_char` pointers between threads. We only allow these to be
// created with static string slices, so it's OK.
unsafe impl Sync for StringEntry {}

/// An entry which contains both an ID (e.g. `ID_RP_BINARY_END`) and an integer.
#[repr(C)]
pub struct IntegerEntry {
    header: EntryCommon,
    id: u32,
    value: u32,
}

impl IntegerEntry {
    /// Create a new `IntegerEntry`
    pub const fn new(tag: u16, id: u32, value: u32) -> IntegerEntry {
        IntegerEntry {
            header: EntryCommon {
                data_type: DataType::IdAndInt,
                tag,
            },
            id,
            value,
        }
    }

    /// Get this entry's address
    pub const fn addr(&self) -> EntryAddr {
        EntryAddr(self as *const Self as *const u32)
    }
}

/// An alias for IntegerEntry, taking a pointer instead of an integer
#[repr(C)]
pub struct PointerEntry {
    header: EntryCommon,
    id: u32,
    value: *const (),
}

impl PointerEntry {
    /// Create a new `PointerEntry`
    ///
    /// Pointers will be marked as 32-bit integers in the binary information
    /// structure, as there is no separate data type tag for pointers. This
    /// assumes that pointers are 32 bit wide, which is obviously true for
    /// rp2040/rp2350. On 64 bit architectures, it will create a binary
    /// structure that likely can't be parsed by picotool.
    pub const fn new(tag: u16, id: u32, value: *const ()) -> PointerEntry {
        PointerEntry {
            header: EntryCommon {
                data_type: DataType::IdAndInt,
                tag,
            },
            id,
            value,
        }
    }

    /// Get this entry's address
    pub const fn addr(&self) -> EntryAddr {
        EntryAddr(self as *const Self as *const u32)
    }
}

// We need this as rustc complains that is is unsafe to share `*const u32`
// pointers between threads. We only allow these to be created with static
// data, so this is OK.
unsafe impl Sync for PointerEntry {}

/// A structure for multiple pins with name info
#[repr(C)]
pub struct PinsWithName {
    header: EntryCommon,
    mask: u32,
    label: *const core::ffi::c_char,
}

impl PinsWithName {
    const fn fold_config(mask: u32, rest: &[u32]) -> u32 {
        match rest {
            [] => mask,
            [first_pin, rest_pins @ ..] => {
                let masked_pin = 1u32
                    .checked_shl(*first_pin)
                    .expect("Pin number should be between 0 and 31");

                assert!(mask < masked_pin, "Pins should be in increasing order");

                Self::fold_config(mask | masked_pin, rest_pins)
            }
        }
    }

    /// Create a new `PinWithName`
    ///
    /// Represents names info for multiple pins
    pub const fn new(pins: &[u32], label: &'static core::ffi::c_str::CStr) -> Self {
        Self {
            header: EntryCommon {
                data_type: DataType::PinsWithName,
                tag: TAG_RASPBERRY_PI,
            },
            mask: Self::fold_config(0, pins),
            label: label.as_ptr(),
        }
    }
    /// Get this entry's address
    pub const fn addr(&self) -> EntryAddr {
        EntryAddr(self as *const Self as *const u32)
    }
}

unsafe impl Sync for PinsWithName {}

/// A structure for multiple pins with function definition
#[repr(C)]
pub struct PinsWithFunction {
    header: EntryCommon,
    encoding: u32,
}

const BI_PINS_ENCODING_MULTI: u32 = 2;
const BI_PINS_ENCODING_RANGE: u32 = 1;

impl PinsWithFunction {
    const fn fold_iter(
        initial_value: u32,
        values: &[u32],
        position: u32,
        last_value: u32,
    ) -> Option<u32> {
        match values {
            [] => {
                if let Some(value) = last_value.checked_shl(5) {
                    Some(initial_value | value)
                } else {
                    None
                }
            }
            [first, rest @ ..] => {
                if let Some(value) = first.checked_shl(7 + 5 * position) {
                    Self::fold_iter(initial_value | value, rest, position + 1, value)
                } else {
                    None
                }
            }
        }
    }
    /// Create a new `PinWithFunction` for multiple pins
    pub const fn new(pins: &[u32], func: GpioFunction) -> Self {
        let func = (func as u32)
            .checked_shl(3)
            .expect("Failed to shift GpioFunction");
        let base: u32 = BI_PINS_ENCODING_MULTI | func;
        let encoding =
            Self::fold_iter(base, pins, 0, 0).expect("All pins should be between 0 and 31");
        Self {
            header: EntryCommon {
                data_type: DataType::PinsWithFunction,
                tag: TAG_RASPBERRY_PI,
            },
            encoding,
        }
    }

    /// Create a new `PinsWithFunction` from a pins range
    ///
    /// Apply function `func` to pins range going from `pin_low` to `pin_high`
    pub const fn new_range(pin_low: u32, pin_high: u32, func: GpioFunction) -> Self {
        let func = (func as u32)
            .checked_shl(3)
            .expect("Failed to shift GpioFunction");
        let base: u32 = BI_PINS_ENCODING_RANGE
            | func
            | pin_low.checked_shl(7).expect("Failed to shift pin_low")
            | pin_high.checked_shl(12).expect("Failed to shift_pin_high");

        Self {
            header: EntryCommon {
                data_type: DataType::PinsWithFunction,
                tag: TAG_RASPBERRY_PI,
            },
            encoding: base,
        }
    }

    /// Get this entry's address
    pub const fn addr(&self) -> EntryAddr {
        EntryAddr(self as *const Self as *const u32)
    }
}

unsafe impl Sync for PinsWithFunction {}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn pins_with_function_generate_proper_outputs() {
        let pwf = PinsWithFunction::new([2].as_slice(), GpioFunction::Uart);
        assert_eq!(pwf.encoding, 2 | 2 << 3 | 2 << 7 | 2 << 12);

        let pwf = PinsWithFunction::new([2, 3].as_slice(), GpioFunction::Uart);
        assert_eq!(pwf.encoding, 2 | 2 << 3 | 2 << 7 | 3 << 12 | 3 << 17);

        let pwf = PinsWithFunction::new([2, 3, 4, 5, 6].as_slice(), GpioFunction::Uart);
        assert_eq!(
            pwf.encoding,
            2 | 2 << 3 | 2 << 7 | 3 << 12 | 4 << 17 | 5 << 22 | 6 << 27
        );
    }

    #[test]
    #[should_panic]
    fn pin_with_function_should_fail_when_more_than_5_pins() {
        PinsWithFunction::new([2, 3, 4, 5, 6, 8].as_slice(), GpioFunction::Uart);
    }

    #[test]
    fn pin_range_with_function_generate_proper_outputs() {
        let pwf = PinsWithFunction::new_range(0, 4, GpioFunction::Uart);
        assert_eq!(pwf.encoding, 1 | 2 << 3 | 4 << 12);
    }

    #[test]
    fn pins_with_name_fold_returns_the_corresponding_mask() {
        assert_eq!(PinsWithName::fold_config(0, &[1, 2, 3]), 0x0E);
    }

    #[test]
    #[should_panic]
    fn pins_with_name_fold_panics_on_unordered_list() {
        PinsWithName::fold_config(0, &[3, 1]);
    }
}

// End of file
