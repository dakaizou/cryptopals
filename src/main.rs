use itertools::Itertools;

// https://cryptopals.com/sets/1
fn main() {
    // let inp = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!(
        "{}",
        base64::encode(
            fixed_xor(
                str_to_vecu8("1c0111001f010100061a024b53535009181c"),
                str_to_vecu8("686974207468652062756c6c277320657965")
            )
            .unwrap()
        )
    );
    println!("{}", str_to_b64("746865206b696420646f6e277420706c6179"));
}

fn str_to_vecu8(byte_str: &str) -> Vec<u8> {
    byte_str
        .chars()
        .collect_vec()
        .chunks(2)
        .map(|s| u8::from_str_radix(s.iter().collect::<String>().as_str(), 16).unwrap())
        .collect_vec()
}

fn str_to_b64(bytes: &str) -> String {
    base64::encode(&str_to_vecu8(bytes))
}

fn fixed_xor(bytes1: Vec<u8>, bytes2: Vec<u8>) -> Option<Vec<u8>> {
    if bytes1.len() != bytes2.len() {
        None
    } else {
        Some(
            bytes1
                .iter()
                .zip(bytes2.iter())
                .map(|(b1, b2)| b1 ^ b2)
                .collect_vec(),
        )
    }
}
