use std::mem::size_of;

struct VeryImportantMessage {
    _message_type: u8,
    _destination: u16,
}

fn main() {
    println!(
        "VeryImportantMessage occupies {} bytes.",
        size_of::<VeryImportantMessage>()
    );
}
// VeryImportantMessage occupies 4 bytes.
// a structure decorated with #[repr(C, packed)] won’t rearrange or pad the structure
