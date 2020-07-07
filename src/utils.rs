
pub fn node_promoted_to_leader(
    current_node_type: i32, prev_node_type: i32
) -> bool {
    return current_node_type == 3 
            && prev_node_type == 1;
}

pub fn node_demoted_from_leader(
    current_node_type: i32, prev_node_type: i32
) -> bool {
    return (current_node_type == 1 || 
            current_node_type == 2)
            && prev_node_type == 3;
}