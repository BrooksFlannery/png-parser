const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprint!("Incorrect args");
        std::process::exit(1);
    }

    let file_path = &args[1];

    let bytes = std::fs::read(file_path).unwrap_or_else(|e| {
        eprintln!("Failed to read file: {e}");
        std::process::exit(1);
    });

    //read first bytes to see if we have a png. if not, fail.
    if bytes[0..8] != PNG_SIGNATURE {
        eprintln!("Not a png");
        std::process::exit(1);
    }

    let mut offset = 8;

    while offset < bytes.len() {
        let chunk = read_chunk(&mut offset, &bytes);
        println!(
            "Chunk: {}, Length: {}, Crc: {}",
            chunk.chunk_type, chunk.length, chunk.crc
        );
    }
}

fn read_chunk(offset: &mut usize, bytes: &[u8]) -> Chunk {
    let length = u32::from_be_bytes(bytes[*offset..*offset + 4].try_into().unwrap());

    *offset += 4;

    let chunk_type = std::str::from_utf8(&bytes[*offset..*offset + 4])
        .unwrap()
        .to_string();

    *offset += 4;

    let data_len = length as usize; //why is this conversion necessary?
    let _data = bytes[*offset..*offset + data_len].to_vec();

    *offset += data_len;

    let crc = u32::from_be_bytes(bytes[*offset..*offset + 4].try_into().unwrap());

    *offset += 4;

    Chunk {
        length,
        chunk_type,
        _data,
        crc,
    }
}

struct Chunk {
    length: u32,
    chunk_type: String,
    _data: Vec<u8>,
    crc: u32,
}
