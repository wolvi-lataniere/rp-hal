//! Handy macros for making Binary Info entries

/// Generate a static item containing the given environment variable,
/// and return its [`EntryAddr`](super::EntryAddr).
#[macro_export]
macro_rules! env {
    ($tag:expr, $id:expr, $env_var_name:expr) => {
        $crate::str!($tag, $id, {
            let value = concat!(env!($env_var_name), "\0");
            // # Safety
            //
            // We used `concat!` to null-terminate on the line above.
            let value_cstr =
                unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(value.as_bytes()) };
            value_cstr
        })
    };
}

/// Generate a static item containing the given string, and return its
/// [`EntryAddr`](super::EntryAddr).
///
/// You must pass a numeric tag, a numeric ID, and `&CStr` (which is always
/// null-terminated).
#[macro_export]
macro_rules! str {
    ($tag:expr, $id:expr, $str:expr) => {{
        static ENTRY: $crate::StringEntry = $crate::StringEntry::new($tag, $id, $str);
        ENTRY.addr()
    }};
}

/// Generate a static item containing the given integer, and return its
/// [`EntryAddr`](super::EntryAddr).
///
/// You must pass a numeric tag, a numeric ID, and `&CStr` (which is always
/// null-terminated).
#[macro_export]
macro_rules! int {
    ($tag:expr, $id:expr, $int:expr) => {{
        static ENTRY: $crate::IntegerEntry = $crate::IntegerEntry::new($tag, $id, $int);
        ENTRY.addr()
    }};
}

/// Generate a static item containing the given pointer, and return its
/// [`EntryAddr`](super::EntryAddr).
///
/// You must pass a numeric tag, a numeric ID, and a pointer
#[macro_export]
macro_rules! pointer {
    ($tag:expr, $id:expr, $ptr:expr) => {{
        static ENTRY: $crate::PointerEntry = $crate::PointerEntry::new($tag, $id, $ptr);
        ENTRY.addr()
    }};
}

/// Concatenate a list of names for the `PinsWithName` structure, returning a &CStr
///
/// This macro adds a '|' character between each string and converts the result to &CStr.
///
/// This macro is used by [`pins_with_names!`](super::pins_with_names) for the names management.
/// # Example
/// ```
/// # use rp_binary_info::*;
/// # use core::ffi::CStr;
/// let concatenated: &CStr = pins_names_concat!("A", "B", "C");
/// assert_eq!(c"A|B|C", concatenated);
/// ```
#[macro_export]
macro_rules! pins_names_concat {
    // For a list of names in &[], just send them flat to the macro
    (&[$($names: expr),+]) => {
        pins_names_concat!($($names),+)
    };

    // For a single string, convert the result to &CStr
    ($names:expr) => {
        if let Ok(x) = core::ffi::CStr::from_bytes_until_nul(
            (concat!($names, "\0")).as_bytes())
        {
            x
        } else {
            panic!("Failed to convert &str to &Cstr");
        }
    };

    // For two strings, concatenate them to a single string and return its conversion
    ($names:expr, $name:expr) => {
        pins_names_concat!(concat!($names,"|",$name))
    };

    // For multiple strings, recursively concatenate them
    ($names:expr, $second:expr, $($more:expr),+) => {
        pins_names_concat!(concat!($names,"|",$second), $($more),+)
    };
}

/// Generate a static item containing a bi_pin_with_names description and returns its
/// [`EntryAddr`](super::EntryAddr).
///
/// Usage: `pins_with_names!(pins: &[u32], names)`
///
/// * `pins` is the list of pins to be assigned, pins numbers should be scrictly ascending,
/// * `names` is the name(s) for the pins with eigher:
///     - a single `&str` to name all the pins with the same label,
///     - a list of `&str` to individually name the pins (either flat or in a `&[&str]`).
///
/// # Example
/// ```
/// # use rp_binary_info::*;
/// let pin_with_one_name : EntryAddr =  pins_with_names!([0,1].as_slice(), "UART");
/// let pin_with_two_names : EntryAddr =  pins_with_names!([0,1].as_slice(), &["RX","TX"]);
/// let pin_with_two_names_without_array : EntryAddr =  pins_with_names!([0,1].as_slice(), "RX","TX");
/// ```
#[macro_export]
macro_rules! pins_with_names {
    ($pins:expr, &[$($names: expr),+]) => {
        pins_with_names!($pins, $($names),+)
    };

    ($pins:expr, $($names:expr),+) => {{
        static ENTRY: $crate::PinsWithName =
            $crate::PinsWithName::new($pins, ($crate::pins_names_concat!($($names),+)));
        ENTRY.addr()
    }};
}

/// Generate a static item containing the bi_pins_with_func descriptor for individual pins and returns its
/// [`EntryAddr`](super::EntryAddr).
///
/// Usage: `pins_with_func!(pins: &[u32], func: PinFunction)`
/// * `pins` is the list of pins to label,
/// * `func` is the [`PinFunction`](super::PinFunction) to label the pins with.
///
/// **NOTE** Using this method, you can only assign up to 5 pins, for more pinsm see
/// [`pins_range_with_func`](pins_range_with_func)
///
/// # Example
/// ```
/// # use rp_binary_info::*;
/// let pins_functions: EntryAddr = pins_with_func!(&[0,1], PinFunction::Uart);
/// ```
#[macro_export]
macro_rules! pins_with_func {
    ($pins:expr, $func: expr) => {{
        static ENTRY: $crate::PinsWithFunction = $crate::PinsWithFunction::new($pins, $func);
        ENTRY.addr()
    }};
}

/// Generate a static item containing the bi_pins_with_func descriptor for a range of pins and
/// returns its [`EntryAddr`](super::EntryAddr).
///
/// Usage: `pins_range_with_func!(low: u32, high: u32, func: PinFunction)`
/// * `low` and `high` are the boundaries of the pins range `[low;high]`,
/// * `func` is the function to assign to there pins.
///
/// # Example
/// ```
/// # use rp_binary_info::*;
/// let pins_functions: EntryAddr = pins_range_with_func!(2,5, PinFunction::Spi);
/// ```
#[macro_export]
macro_rules! pins_range_with_func {
    ($low:expr, $high:expr, $func:expr) => {{
        static ENTRY: $crate::PinsWithFunction =
            $crate::PinsWithFunction::new_range($low, $high, $func);
        ENTRY.addr()
    }};
}

/// Generate a static item containing the program name, and return its
/// [`EntryAddr`](super::EntryAddr).
#[macro_export]
macro_rules! rp_program_name {
    ($name:expr) => {
        $crate::str!(
            $crate::consts::TAG_RASPBERRY_PI,
            $crate::consts::ID_RP_PROGRAM_NAME,
            $name
        )
    };
}

/// Generate a static item containing the `CARGO_BIN_NAME` as the program name,
/// and return its [`EntryAddr`](super::EntryAddr).
#[macro_export]
macro_rules! rp_cargo_bin_name {
    () => {
        $crate::env!(
            $crate::consts::TAG_RASPBERRY_PI,
            $crate::consts::ID_RP_PROGRAM_NAME,
            "CARGO_BIN_NAME"
        )
    };
}

/// Generate a static item containing the program version, and return its
/// [`EntryAddr`](super::EntryAddr).
#[macro_export]
macro_rules! rp_program_version {
    ($version:expr) => {{
        $crate::str!(
            $crate::consts::TAG_RASPBERRY_PI,
            $crate::consts::ID_RP_PROGRAM_VERSION,
            $version
        )
    }};
}

/// Generate a static item containing the `CARGO_PKG_VERSION` as the program
/// version, and return its [`EntryAddr`](super::EntryAddr).
#[macro_export]
macro_rules! rp_cargo_version {
    () => {
        $crate::env!(
            $crate::consts::TAG_RASPBERRY_PI,
            $crate::consts::ID_RP_PROGRAM_VERSION_STRING,
            "CARGO_PKG_VERSION"
        )
    };
}

/// Generate a static item containing the program URL, and return its
/// [`EntryAddr`](super::EntryAddr).
#[macro_export]
macro_rules! rp_program_url {
    ($url:expr) => {
        $crate::str!(
            $crate::consts::TAG_RASPBERRY_PI,
            $crate::consts::ID_RP_PROGRAM_URL,
            $url
        )
    };
}

/// Generate a static item containing the `CARGO_PKG_HOMEPAGE` as the program URL,
/// and return its [`EntryAddr`](super::EntryAddr).
#[macro_export]
macro_rules! rp_cargo_homepage_url {
    () => {
        $crate::env!(
            $crate::consts::TAG_RASPBERRY_PI,
            $crate::consts::ID_RP_PROGRAM_URL,
            "CARGO_PKG_HOMEPAGE"
        )
    };
}

/// Generate a static item containing the program description, and return its
/// [`EntryAddr`](super::EntryAddr).
#[macro_export]
macro_rules! rp_program_description {
    ($description:expr) => {
        $crate::str!(
            $crate::consts::TAG_RASPBERRY_PI,
            $crate::consts::ID_RP_PROGRAM_DESCRIPTION,
            $description
        )
    };
}

/// Generate a static item containing the `CARGO_PKG_DESCRIPTION` as the program description,
/// and return its [`EntryAddr`](super::EntryAddr).
#[macro_export]
macro_rules! rp_cargo_description {
    () => {
        $crate::env!(
            $crate::consts::TAG_RASPBERRY_PI,
            $crate::consts::ID_RP_PROGRAM_DESCRIPTION,
            "CARGO_PKG_DESCRIPTION"
        )
    };
}

/// Generate a static item containing whether this is a debug or a release
/// build, and return its [`EntryAddr`](super::EntryAddr).
#[macro_export]
macro_rules! rp_program_build_attribute {
    () => {
        $crate::str!(
            $crate::consts::TAG_RASPBERRY_PI,
            $crate::consts::ID_RP_PROGRAM_BUILD_ATTRIBUTE,
            {
                if cfg!(debug_assertions) {
                    c"debug"
                } else {
                    c"release"
                }
            }
        )
    };
}

/// Generate a static item containing the specific board this program runs on,
/// and return its [`EntryAddr`](super::EntryAddr).
#[macro_export]
macro_rules! rp_pico_board {
    ($board:expr) => {
        $crate::str!(
            $crate::consts::TAG_RASPBERRY_PI,
            $crate::consts::ID_RP_PICO_BOARD,
            $board
        )
    };
}

/// Generate a static item containing the binary end address, and return its
/// [`EntryAddr`](super::EntryAddr). The argument should be a symbol provided
/// by the linker script that is located at the end of the binary.
#[macro_export]
macro_rules! rp_binary_end {
    ($ptr:ident) => {{
        $crate::pointer!(
            $crate::consts::TAG_RASPBERRY_PI,
            $crate::consts::ID_RP_BINARY_END,
            // `unsafe` only needed because MSRV does not yet
            // contain https://github.com/rust-lang/rust/pull/125834
            unsafe { core::ptr::addr_of!($ptr).cast() }
        )
    }};
}

#[cfg(test)]
mod test {
    #[test]
    fn names_concatenation_returns_single_name_if_only_one_provided() {
        assert_eq!(c"a", pins_names_concat!("a"));
        assert_eq!(c"a", pins_names_concat!(&["a"]));
    }

    #[test]
    fn names_concatenation_returns_pipe_separated_list_of_names() {
        assert_eq!(c"a|b|c", pins_names_concat!(&["a", "b", "c"]));
    }
}

// End of file
