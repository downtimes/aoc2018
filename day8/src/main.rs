
#[derive(Default)]
struct Node {
    num_nodes: usize,
    num_metadata: usize,
    children: Vec<Node>,
    metainformation: Vec<u32>,
}

fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let input = input.trim();
    let input = convert_to_numbers(&input);
    let (tree, _) = convert_to_tree(&input);
    println!("{}", sum_up_metas(&tree));
    println!("{}", calculate_value(&tree));
}


fn convert_to_numbers(input: &str) -> Vec<u32> {
    input.split(' ').filter_map(|num| num.parse().ok()).collect()
}

fn calculate_value(root: &Node) -> u32 {
    if root.num_nodes == 0 {
        return root.metainformation.iter().sum();
    }
    let mut value = 0;
    for &idx in root.metainformation.iter() {
        let idx = idx as usize;
        if idx <= root.num_nodes && idx != 0 {
            value += calculate_value(&root.children[idx - 1]);
        }
    }
    value
}

fn sum_up_metas(root: &Node) -> u32 {
    let mut count = root.metainformation.iter().sum();
    for node in &root.children {
        count += sum_up_metas(node);
    }
    count
}

fn convert_to_tree(input: &[u32]) -> (Node, u32) {
    let mut current_node = Node::default();
    let mut size = 2;
    current_node.num_nodes = input[0] as usize;
    current_node.num_metadata = input[1] as usize;
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

