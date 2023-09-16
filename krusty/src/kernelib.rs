
#[allow(non_camel_case_types)]
type int = i32;
#[allow(non_camel_case_types)]
type uint = u32;

#[allow(dead_code)]
extern "C" {
    // pub fn test_print(_ :u64) -> i64;
    pub fn uartputc(_: int); 
}
