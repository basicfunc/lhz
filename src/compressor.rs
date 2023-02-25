use std::collections::BinaryHeap;
use std::collections::HashMap;

// Step 1: Define a data structure to represent the Huffman tree
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HuffmanNode {
    symbol: Option<u8>,
    weight: usize,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

// Step 2: Build the Huffman tree from the input data
pub fn build_tree(data: &[u8]) -> Box<HuffmanNode> {
    // Calculate the frequency of each symbol
    let mut freq_table: HashMap<u8, usize> = HashMap::new();
    for &symbol in data {
        *freq_table.entry(symbol).or_insert(0) += 1;
    }

    // Use a priority queue to build the tree
    let mut queue: BinaryHeap<Box<HuffmanNode>> = BinaryHeap::new();
    for (symbol, weight) in freq_table {
        queue.push(Box::new(HuffmanNode {
            symbol: Some(symbol),
            weight: weight,
            left: None,
            right: None,
        }));
    }
    while queue.len() > 1 {
        let left = queue.pop().unwrap();
        let right = queue.pop().unwrap();
        queue.push(Box::new(HuffmanNode {
            symbol: None,
            weight: left.weight + right.weight,
            left: Some(left),
            right: Some(right),
        }));
    }
    queue.pop().unwrap()
}

// Step 3: Generate the Huffman codes for each symbol
pub fn generate_codes(
    root: &HuffmanNode,
    prefix: &mut Vec<bool>,
    codes: &mut HashMap<u8, Vec<bool>>,
) {
    if let Some(symbol) = root.symbol {
        codes.insert(symbol, prefix.clone());
    } else {
        prefix.push(false);
        generate_codes(root.left.as_ref().unwrap(), prefix, codes);
        prefix.pop();
        prefix.push(true);
        generate_codes(root.right.as_ref().unwrap(), prefix, codes);
        prefix.pop();
    }
}

// Step 4: Encode the input data using the Huffman codes
pub fn encode(data: &[u8], codes: &HashMap<u8, Vec<bool>>) -> Vec<bool> {
    let mut encoded = Vec::new();
    for &symbol in data {
        encoded.extend(codes[&symbol].iter());
    }
    encoded
}

// Step 5: Decode the compressed data back to the original data
pub fn decode(encoded: &[bool], root: &HuffmanNode) -> Vec<u8> {
    let mut decoded = Vec::new();
    let mut node = root;
    for &bit in encoded {
        node = if bit {
            node.right.as_ref().unwrap()
        } else {
            node.left.as_ref().unwrap()
        };
        if let Some(symbol) = node.symbol {
            decoded.push(symbol);
            node = root;
        }
    }
    decoded
}
