//! A couple of macros to implement limited Rust-style printing via `xil::print`
//! and `format_args`.

/// Adds a newline (\n\r) to the format string and calls print. Prints up to 64
/// characters.
#[macro_export]
macro_rules! println64
{
    () => ({
        print64!("\n\r")
    });
    ($fmt:expr) => ({
        print64!(concat!($fmt, "\n\r"))
    });
    ($fmt:expr, $($args:tt)+) => ({
        print64!(concat!($fmt, "\n\r"), $($args)+)
    });
}

/// Prints text, up to 64 characters.
#[macro_export]
macro_rules! print64
{
    ($fmt:expr) => ({
        let mut buf = arrayvec::ArrayString::<[u8; 128]>::new();
        core::fmt::write(
            &mut buf,
            format_args!(concat!($fmt, "\0")),
        )
        .unwrap();
        unsafe { xil::print(buf.as_ptr()); }
    });
    ($fmt:expr, $($args:tt)+) => ({
        let mut buf = arrayvec::ArrayString::<[u8; 64]>::new();
        core::fmt::write(
            &mut buf,
            format_args!(concat!($fmt, "\0"), $($args)+),
        )
        .unwrap();
        unsafe { xil::print(buf.as_ptr()); }
    });
}
