
#[cfg(not(feature = "prod"))]
macro_rules! println {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut output = jlink_rtt::NonBlockingOutput::new();
        let _ = writeln!(&mut output, $($arg)*);
    });
}

#[cfg(feature = "prod")]
macro_rules! println {
    ($($arg:tt)*) => ({
    });
}
