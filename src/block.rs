use std::array::TryFromSliceError;

pub fn split_u16(v: u16) -> [u8; 2] {
    v.to_be_bytes()
}

pub fn split_u32(v: u32) -> [u8; 4] {
    v.to_be_bytes()
}

pub fn split_u64(v: u64) -> [u8; 8] {
    v.to_be_bytes()
}

pub fn combine_u16(v: [u8; 2]) -> u16 {
    u16::from_be_bytes(v)
}

pub fn combine_u32(v: [u8; 4]) -> u32 {
    u32::from_be_bytes(v)
}

pub fn combine_u64(v: [u8; 8]) -> u64 {
    u64::from_be_bytes(v)
}


pub fn bytes_to_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(&bytes).to_string()
}

pub fn convert_array_u8_64(p0: &[u8]) -> Vec<u64> {
    if p0.len() %8 != 0 {
        panic!("Cannot divide the array by 8. got : {}", p0.len());
    }
    let mut ret: Vec<u64> = vec![0u64; (p0.len() / 8)  as usize];

    for i in 0..(p0.len() / 8) {

println!("{:?}",(i*8..(i*8)+8));
        let u_64 : Result<[u8; 8], TryFromSliceError> = <[u8;8]>::try_from(&p0[i*8..(i*8)+8]);

        if u_64.is_err() {
            println!("{:?}",u_64.unwrap_err());

        }

        ret[i] = combine_u64(u_64.unwrap());

    }
ret

}