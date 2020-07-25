#![no_std]
#![cfg_attr(not(test), no_main)] // #![no_main] interfers with 'cargo test' when targeting the host machine.

extern crate avr_std_stub;

#[no_mangle]
#[cfg(not(test))] // The main function interfers with 'cargo test' when targeting the host machine.
fn main() {
}

#[cfg(test)]
mod test {
    #[test]
    fn test_foo() {
        assert_eq!(1, 1);
    }
}
