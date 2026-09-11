pub mod buffer;
pub mod block;
pub mod schema;


#[cfg(feature = "dbg")]
pub mod debug {
    /// Print a byte slice as individual 8-bit binary values.
    ///
    /// Example:
    /// [65, 66] -> 01000001 01000010
    pub fn print_binary(data: &[u8]) {
        for byte in data {
            print!("{:08b} ", byte);
        }
        println!();
    }

    /// Print binary without spaces between bytes.
    ///
    /// Example:
    /// 0100000101000010
    pub fn print_binary_compact(data: &[u8]) {
        for byte in data {
            print!("{:08b}", byte);
        }
        println!();
    }

    /// Print each byte on its own line with its index.
    pub fn print_binary_indexed(data: &[u8]) {
        for (i, byte) in data.iter().enumerate() {
            println!("[{:04}] {:08b}", i, byte);
        }
    }

    /// Print index, decimal, hexadecimal, and binary representation.
    pub fn print_bytes(data: &[u8]) {
        for (i, byte) in data.iter().enumerate() {
            println!(
                "[{:04}] dec={:3} hex=0x{:02X} bin={:08b}",
                i,
                byte,
                byte,
                byte
            );
        }
    }

    /// Print bytes in groups of 8.
    pub fn print_binary_groups(data: &[u8]) {
        for (i, byte) in data.iter().enumerate() {
            if i > 0 && i % 8 == 0 {
                println!();
            }

            print!("{:08b} ", byte);
        }

        println!();
    }

    /// Print bytes with an offset.
    pub fn print_binary_offset(data: &[u8], offset: usize) {
        for (i, byte) in data.iter().enumerate() {
            println!(
                "[{:08}] {:08b}",
                offset + i,
                byte
            );
        }
    }

    /// Print a range of bytes as binary.
    pub fn print_binary_range(data: &[u8], start: usize, end: usize) {
        if start > end || end > data.len() {
            println!(
                "Invalid range {}..{} for buffer of size {}",
                start,
                end,
                data.len()
            );
            return;
        }

        println!("Range {}..{} ({} bytes):", start, end, end - start);

        for (i, byte) in data[start..end].iter().enumerate() {
            println!(
                "[{:08}] {:08b}",
                start + i,
                byte
            );
        }
    }

    /// Print a byte slice in a hexdump-like format.
    pub fn print_hexdump(data: &[u8]) {
        for (row, chunk) in data.chunks(16).enumerate() {
            let offset = row * 16;

            print!("{:08X}  ", offset);

            for byte in chunk {
                print!("{:02X} ", byte);
            }

            // Padding
            for _ in chunk.len()..16 {
                print!("   ");
            }

            print!(" |");

            for byte in chunk {
                let c = *byte as char;

                if c.is_ascii_graphic() || c == ' ' {
                    print!("{}", c);
                } else {
                    print!(".");
                }
            }

            println!("|");
        }
    }

    /// Print a hexdump with binary representation.
    pub fn print_binary_dump(data: &[u8]) {
        for (row, chunk) in data.chunks(4).enumerate() {
            let offset = row * 4;

            print!("{:08X}  ", offset);

            for byte in chunk {
                print!("{:08b} ", byte);
            }

            println!();
        }
    }

    /// Print a slice as bits, including bit indexes.
    pub fn print_bits(data: &[u8]) {
        for (byte_index, byte) in data.iter().enumerate() {
            println!("byte {}: {}", byte_index, format!("{:08b}", byte));

            for bit in 0..8 {
                let value = (byte >> (7 - bit)) & 1;

                println!(
                    "    bit {} = {}",
                    bit,
                    value
                );
            }
        }
    }

    /// Print a slice as a binary string.
    pub fn binary_string(data: &[u8]) -> String {
        data.iter()
            .map(|b| format!("{:08b}", b))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Print a slice as binary with a label.
    pub fn print_binary_label(label: &str, data: &[u8]) {
        println!("{} ({} bytes):", label, data.len());
        print_binary(data);
    }
}