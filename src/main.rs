use std::collections::HashMap;

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

/// Given the Huffman code HashMap and the message to decode, it reverts the Huffman code Hashmap and returns the decoded message
fn huffman_decode(huffman_code: &HashMap<char, String>, message_to_decode: &str) -> String {
    let reverted = revert_huffman_code(huffman_code);
    let mut decoded_message = String::new();

    let mut bits = String::new();
    for bit in message_to_decode.chars() {
        bits.push(bit);
        if reverted.contains_key(&bits) {
            decoded_message.push(reverted.get(&bits).unwrap().clone());
            bits = String::new();
        }
    }

    decoded_message
}

/// Auxiliar method to reverse the Huffman code Hashmap so you can search the char by its code.
///
/// Returns a HashMap with the Huffman code String in the key and the character in the value.
fn revert_huffman_code(huffman_code: &HashMap<char, String>) -> HashMap<String, char> {
    let mut reverted: HashMap<String, char> = HashMap::new();

    for (key, value) in huffman_code.into_iter() {
        reverted.insert(value.clone(), *key);
    }

    reverted
}

/// Given the Huffman code HashMap and the message to encode.
///
/// Returns a String with the message encoded.
fn huffman_encode(huffman_code: &HashMap<char, String>, message_to_encode: &str) -> String {
    let mut encoded_message = String::new();

    for c in message_to_encode.chars() {
        let bits = huffman_code.get(&c).unwrap();
        encoded_message.push_str(bits);
    }

    encoded_message
}

/// Given the root node, it iterates through its children to find all the characters and their corresponding codes.
///
/// It returns a HashMap that has the characters as keys, and its compressed code in the value as a String.
fn iterate_tree(root_node: Node) -> HashMap<char, String> {
    let mut dict: HashMap<Node, String> = HashMap::new();
    let mut char_dict: HashMap<char, String> = HashMap::new();

    dict.insert(root_node, "".to_string());

    while dict.len() > 0 {
        let node = &dict.keys().next().unwrap().clone();
        let value = dict.get(node).unwrap().clone();

        if node.is_leaf {
            // If its end of tree
            let char = node.leaf_value.unwrap();
            char_dict.insert(char, value);

            dict.remove(node);
            continue;
        }
        let children = node.children.clone().unwrap();
        dict.insert(children[0].clone(), add_bit_1(&value));
        dict.insert(children[1].clone(), add_bit_0(&value));

        dict.remove(node);
    }

    char_dict
}

/// Auxiliar method to add a 0 bit.
///
/// For when we choose the left child.
fn add_bit_0(bits: &str) -> String {
    bits.to_owned() + "0"
}

/// Auxiliar method to add a 1 bit.
///
/// For when we choose the right child.
fn add_bit_1(bits: &str) -> String {
    bits.to_owned() + "1"
}

/// Converts the input string reference to a HashMap with the characters of the string as keys and the number of appearences as values
/// # Examples
/// ````
/// // If the input text is "Hello World!" it will return the followinf HashMap
/// let dictionary = input_to_dictionary("Hello World!");
/// println("{:#?}", dictionary);
///
/// Output: {
/// 'o': 2,
/// 'r': 1,
/// 'l': 3,
/// 'H': 1,
/// ' ': 1,
/// 'd': 1,
/// 'e': 1,
/// '!': 1,
/// 'W': 1,
/// }
/// ````
fn input_to_dictionary(input: &str) -> HashMap<char, i32> {
    let mut dictionary: HashMap<char, i32> = HashMap::new();

    for c in input.chars() {
        if dictionary.contains_key(&c) {
            let val = dictionary.get_mut(&c).unwrap();
            *val += 1;
        } else {
            dictionary.insert(c, 1);
        }
    }

    dictionary
}

/// Given a HashMap<char, i32>, it returns a HashMap<Node, i32> with all the characters stored in a Node as a key abd the number of appearences as values
fn char_dictionary_to_node_dictionary(dict: HashMap<char, i32>) -> HashMap<Node, i32> {
    let mut node_dictionary: HashMap<Node, i32> = HashMap::new();

    for c in dict.keys() {
        let value = dict.get(c).unwrap();
        let node = Node::new_leaf(*c, *value);

        node_dictionary.insert(node, *value);
    }

    node_dictionary
}

/// Given a HashMap<Node, i32>, it returns all the Nodes with the lowest appearences.
///
/// If for example there's various Nodes that appear one time, It'll return all those Nodes.
///
/// But if there's only one that appear one time, It'll search the next value with Nodes and append those to the original we had.
/// This way it'll always have more than one to make as children of the new node.
fn get_minimum_value(dict: HashMap<Node, i32>) -> Vec<Node> {
    let mut minimum_value = i32::MAX;
    let mut nodes: Vec<Node> = Vec::new();

    for node in dict.keys() {
        let value = dict.get(node).unwrap();
        if value < &minimum_value {
            minimum_value = *value;
            nodes = vec![node.clone()];
        } else if value == &minimum_value {
            nodes.push(node.clone());
        }
    }

    if nodes.len() == 1 {
        let mut changed_dict = dict.clone();
        changed_dict.remove(&nodes[0]);
        let mut next_min_nodes = get_minimum_value(changed_dict);
        nodes.append(&mut next_min_nodes);
    }

    nodes
}

/// Given a HashMap<Node, i32> where Node is the characters and i32 are the number of appearences, It'll combine them using the Huffman Algorithm to make a Tree.
fn huffman_compress(dict: HashMap<Node, i32>) -> Node {
    let mut dict_mut = dict.clone();

    // While there's more than one node
    while dict_mut.len() > 1 {
        //Getting all the nodes with the smallest value
        let min_values: Vec<Node> = get_minimum_value(dict_mut.clone());

        //Getting the first two nodes
        let nodes = min_values[0..2].to_vec();

        //Combining both its values
        let combined_values = &nodes[0].get_value() + &nodes[1].get_value();

        //Creating parent node
        let parent_node = Node::new_parent(combined_values, nodes.clone());

        //Removing nodes from dictionary
        dict_mut.remove(&nodes[0]);
        dict_mut.remove(&nodes[1]);

        //Adding the new parent_node to dictionary
        dict_mut.insert(parent_node, combined_values);
    }

    let nodes: Vec<Node> = dict_mut.keys().cloned().collect();

    nodes[0].clone()
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
/// Node of the Huffman Binary Tree
struct Node {
    /// If the node is a leaf (end of the tree), this boolean is true
    is_leaf: bool,
    /// If the node is a leaf, this has the value of that character
    leaf_value: Option<char>,
    /// Has the value of the node (number of appearences of the character, or sum of children nodes)
    value: i32,
    /// If it's not a leaf, here are stored the children of the node.
    children: Option<Vec<Node>>,
}

impl Node {
    /// When we are creating a leaf node, we use this constructor, to give it a character value, and the number of appearences.
    /// The is_leaf bool is set automatically to true.
    fn new_leaf(leaf_value: char, value: i32) -> Node {
        Node {
            is_leaf: true,
            leaf_value: Some(leaf_value),
            value: value,
            children: None,
        }
    }

    /// When we are creating a parent node, we use this constructor, to give it a sum value of the children and both of its children.
    /// The is_leaf bool is set automatically to false.
    ///
    /// If the Vec of children contain more or less than two children, it panics.
    fn new_parent(value: i32, children: Vec<Node>) -> Node {
        if children.len() != 2 {
            panic!("A parent node must have two children");
        }

        Node {
            is_leaf: false,
            leaf_value: None,
            value: value,
            children: Some(children),
        }
    }

    /// Method to get the value of the node
    fn get_value(&self) -> i32 {
        self.value
    }
}

#[derive(Debug)]
pub struct HuffmanTree {
    /// The original text used to create the Huffman Code Table
    original_text: Option<String>,
    /// The Huffman Code Table, responsible for encoding and decoding the messages
    code_table: HashMap<char, String>,
}

impl HuffmanTree {
    /// Constructor with the original text
    pub fn new(original_text: String) -> HuffmanTree {
        let original_text_dictionary = input_to_dictionary(&original_text);
        let nodes_dictionary = char_dictionary_to_node_dictionary(original_text_dictionary);
        let root_node = huffman_compress(nodes_dictionary);
        let huffman_code = iterate_tree(root_node.clone());

        HuffmanTree {
            original_text: Some(original_text),
            code_table: huffman_code,
        }
    }

    /// Constructor with the Huffman Code Table
    pub fn new_decoder(code_table: HashMap<char, String>) -> HuffmanTree {
        HuffmanTree {
            original_text: None,
            code_table,
        }
    }

    /// Getter for the Huffman Code Table
    pub fn get_code_table(&self) -> HashMap<char, String> {
        self.code_table.clone()
    }

    /// Getter for the original text.
    ///
    /// If there's no original text, it panics
    pub fn get_original_text(&self) -> String {
        match &self.original_text {
            Some(text) => text.clone(),
            None => {
                panic!("Trying to get Original Text without initializing HuffmanTree with it!")
            }
        }
    }

    /// Method to encode a message using the Huffman Code Table
    pub fn encode(&self, message: &str) -> String {
        huffman_encode(&self.code_table, message)
    }

    /// Method to decode a message using the Huffman Code Table
    pub fn decode(&self, coded_message: &str) -> String {
        huffman_decode(&self.code_table, coded_message)
    }
}
