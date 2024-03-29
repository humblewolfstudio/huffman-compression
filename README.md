# Huffman Tree Implementation in Rust

This is my crude implementation of the Huffman Tree Algorithm.

### Example using Text:
```rust
fn main() {
    let text = "Hello world!".to_string();

    let huffman_tree = HuffmanTree::new(text.clone());

    let encoded_message = huffman_tree.encode(&text);
    let decoded_message = huffman_tree.decode(&encoded_message);

    println!("Compressed: {}", encoded_message);
    println!("Uncompressed: {}", decoded_message);

    println!("Huffman Codes: {:#?}", huffman_tree.get_code_table());
}
```
### Example using 
```rust
fn main() {
    let huffman_table = HashMap::from([
        ('o', String::from("100")),
        ('l', String::from("01")),
        ('e', String::from("111")),
        (' ', String::from("0000")),
        ('!', String::from("110")),
        ('w', String::from("101")),
        ('d', String::from("0011")),
        ('H', String::from("0010")),
        ('r', String::from("0001")),
    ]);

    let huffman_tree = HuffmanTree::new_decoder(huffman_table);

    let encoded_message = huffman_tree.encode("Hello world!");
    let decoded_message = huffman_tree.decode("0010111010110000001011000001010011110");

    println!("Compressed: {}", encoded_message);
    println!("Uncompressed: {}", decoded_message);
}
```