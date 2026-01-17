pub fn print_hex_dump(start_addr: u64, data: &[u8]) {
    for (row_idx, row) in data.chunks(16).enumerate() {
        let addr = start_addr + (row_idx * 16) as u64;
        print!("{:016X}: ", addr);
        for b in row {
            print!("{:02X} ", b);
        }

        if row.len() < 16 {
            for _ in 0..(16 - row.len()) {
                print!("   ");
            }
        }

        print!(" |");

        for b in row {
            let ch = if b.is_ascii_graphic() || *b == b' ' {
                *b as char
            } else {
                '.'
            };
            print!("{}", ch);
        }
        println!("|");
    }
}
