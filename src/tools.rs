use std::fs::{File, OpenOptions};

pub fn continue_file(source_file: &str) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(source_file)
        .unwrap()
}

// pub fn generate_random(string_len: usize, custom_charset: Option<String>) -> String {
//     let custom_charset = match custom_charset {
//         Some(set) => set.bytes(),
//         None => {
//             "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~".as_bytes()
//         }
//     };




//     // let mut rng = rand::thread_rng();

//     // let random_string: String = (0..string_len)
//     //     .map(|_| {
//     //         let idx = rng.gen_range(0..custom_charset.as_bytes().len());
//     //         custom_charset.as_bytes()[idx] as char
//     //     })
//     //     .collect();

//     // random_string
// }