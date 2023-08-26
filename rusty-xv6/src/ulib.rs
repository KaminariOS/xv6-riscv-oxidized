type int = i32;
extern "C" {
    pub fn printf(s: *const u8, args: ...);
    pub fn exit(status: int) -> int;
    pub fn fork() -> int;
    pub fn pipe(_: *mut int) -> int;
    pub fn wait(_: *mut int) -> int;
    pub fn close(_: *mut int) -> int;
    pub fn uptime() -> int;
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        printf(concat!($fmt, "\0").as_bytes().as_ptr() $(, $($arg)+)?);
    }
}


#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        printf(concat!($fmt, "\n\0").as_bytes().as_ptr() $(, $($arg)+)?)
    };
    () => {
        print!("\n")
    }
}
