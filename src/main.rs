use std::io::{stdout};
use std::io;

fn main() {
    let standard_output = stdout();
    println!("Please input the message you wish to encrypt: ");
    let mut message = String::new();
    io::stdin()
        .read_line(&mut message);

    println!("Please input the key to be used for encryption: ");
    let mut key = String::new();
    io::stdin()
        .read_line(&mut key);

    let mut ciphertext = Encrypt(key, message);

    println!("The ciphertext is: {}", ciphertext);

    println!("Please input the key to be used for decryption: ");
    let mut key2 = String::new();
    io::stdin().read_line(&mut key2);

    let mut decrypted_message = Decrypt(key2, ciphertext);

    println!("The decrypted message is: {}", decrypted_message);

}

fn Encrypt(key: String, message: String) -> String {
    let mut key_bytes = text_to_bytes(key);
    let mut message_bytes = text_to_bytes(message);
    let mut ciphertext_bytes = applyXOR(key_bytes, message_bytes);
    let mut ciphertext = bytes_to_text(ciphertext_bytes.as_ref());
    return ciphertext;
}

fn Decrypt(key: String, ciphertext: String) -> String {
    let mut key_bytes = text_to_bytes(key);
    let mut ciphertext_bytes= text_to_bytes(ciphertext);
    let mut message_bytes = applyXOR(key_bytes, ciphertext_bytes);
    let mut message = bytes_to_text(message_bytes.as_ref());
    return  message;
}

fn text_to_bytes(text: String) -> &'static [u8] {
    return text.as_bytes();
}

fn bytes_to_text(text: &[u8]) -> String {
    let mut text = String::from_utf8(Vec::from(text)).unwrap();
    return text;

}

// Still modifying this function.
fn applyXOR<'a>(key:  &'a [u8], text: &'a [u8]) -> &'a [u8] {
    let mut xor: &[u8] = &[];
    for b in text {
        xor = b ^ key
    }
    return xor;

}

