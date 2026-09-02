//! Constants for binary info

/// All Raspberry Pi specified IDs have this tag.
///
/// You can create your own for custom fields.
pub const TAG_RASPBERRY_PI: u16 = super::make_tag(b"RP");

/// Used to note the program name - use with StringEntry
pub const ID_RP_PROGRAM_NAME: u32 = 0x02031c86;
/// Used to note the program version - use with StringEntry
pub const ID_RP_PROGRAM_VERSION_STRING: u32 = 0x11a9bc3a;
/// Used to note the program build date - use with StringEntry
pub const ID_RP_PROGRAM_BUILD_DATE_STRING: u32 = 0x9da22254;
/// Used to note the size of the binary - use with IntegerEntry
pub const ID_RP_BINARY_END: u32 = 0x68f465de;
/// Used to note a URL for the program - use with StringEntry
pub const ID_RP_PROGRAM_URL: u32 = 0x1856239a;
/// Used to note a description of the program - use with StringEntry
pub const ID_RP_PROGRAM_DESCRIPTION: u32 = 0xb6a07c19;
/// Used to note some feature of the program - use with StringEntry
pub const ID_RP_PROGRAM_FEATURE: u32 = 0xa1f4b453;
/// Used to note some whether this was a Debug or Release build - use with StringEntry
pub const ID_RP_PROGRAM_BUILD_ATTRIBUTE: u32 = 0x4275f0d3;
/// Used to note the Pico SDK version used - use with StringEntry
pub const ID_RP_SDK_VERSION: u32 = 0x5360b3ab;
/// Used to note which board this program targets - use with StringEntry
pub const ID_RP_PICO_BOARD: u32 = 0xb63cffbb;
/// Used to note which `boot2` image this program uses - use with StringEntry
pub const ID_RP_BOOT2_NAME: u32 = 0x7f8882e1;

/// Availatble GPIO Functions for BI_PIN_WITH_FUNC
pub enum GpioFunction {
    /// eXecute-In-Place Function
    Xip = 0,
    /// GPIO pin is used for Serial Perpheral Interface
    Spi = 1,
    /// GPIO pin is used for Univerasl Asynchronous Receiver/Transmitter
    Uart = 2,
    /// GPIO pin is used for I2C
    I2c = 3,
    /// GPIO pin is used for Pulse Width Modulation
    Pwm = 4,
    /// GPIO pin is used for Single-cycle I/O
    Sio = 5,
    /// GPIO pin is used for Programmable I/O block 0
    Pio0 = 6,
    /// GPIO pin is used for Programmable I/O block 1
    Pio1 = 7,
    /// GPIO pin is used for clock signal
    Gpck = 8,
    /// GPIO pin is used for USB inferface
    Usb = 9,
    /// GPIO pin is used for High Speed Tx
    Hstx = 10,
    /// GPIO pin is used for Programmable I/O block 2
    Pio2 = 11,
    /// GPIO pin is used for Tracing
    CoresightTrace = 12,
    /// GPIO pin is used for Auxilary UART
    UartAux = 13,
    /// GPIO pin is not used
    Null = 0x1f,
}

// End of file
