
#[derive(Default)]
struct Node {
    num_nodes: u8,
    num_metadata: u8,
    children: Vec<Node>,
    metainformation: Vec<u8>,
}

fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let input = convert_to_numbers(&input);
    let (tree, _) = convert_to_tree(&input);
    println!("{}", count_number_of_metas(&tree));
}


fn convert_to_numbers(input: &str) -> Vec<u8> {
    input.split(' ').filter_map(|num| num.parse().ok()).collect()
}

fn count_number_of_metas(root: &Node) -> u32 {
    let mut count = root.num_metadata as u32;
    for node in &root.children {
        count += count_number_of_metas(node);
    }
    count
}

fn convert_to_tree(input: &[u8]) -> (Node, u32) {
    let mut current_node = Node::default();
    let mut size = 2;
    current_node.num_nodes = input[0];
    current_node.num_metadata = input[1];
    let mut children = Vec::new();
    for _ in 0..current_node.num_nodes {
        let (node, node_size) = convert_to_tree(&input[size as usize..]);
        children.push(node);
        size += node_size;
    }
    current_node.children = children;
    let mut metadata = Vec::new();
    for _ in 0..current_node.num_metadata {
        //TODO: We run over the size of the slic here. Check why!
        metadata.push(input[size as usize]);
        size += 1;
    }
    current_node.metainformation = metadata;
    (current_node, size)
}

