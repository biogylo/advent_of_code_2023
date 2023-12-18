use advent_of_code_2023::sandbox::dijkstra::Graph;

#[test]
fn test_basic_case() {
    // Build the graph by making all the points
    let mut graph = Graph::new(5);
    graph.zero_out_self_weights();
    graph.add_weight(0, 1, 4);
    graph.add_weight(0, 2, 2);
    graph.add_weight(1, 2, 3);
    graph.add_weight(2, 4, 2);
    graph.add_weight(1, 4, 5);
    graph.add_weight(1, 3, 3);
    graph.add_weight(3, 4, 2);

    // Shortest distance from 0 to 4 is 4
    // Shortest distance from 0 to 3 is 6
    assert_eq!(graph.shortest_distance(0, 4), 4);
    assert_eq!(graph.shortest_distance(0, 3), 6);
}

#[test]
fn test_other_basic_case() {
    // Build the graph by making all the points
    let mut graph = Graph::new(6);
    graph.zero_out_self_weights();
    graph.add_weight(0, 1, 9);
    graph.add_weight(0, 2, 4);

    graph.add_weight(1, 2, 2);
    graph.add_weight(1, 3, 7);
    graph.add_weight(1, 4, 3);

    graph.add_weight(2, 3, 1);
    graph.add_weight(2, 4, 6);

    graph.add_weight(3, 4, 4);
    graph.add_weight(3, 5, 8);

    graph.add_weight(4, 5, 2);
    assert_eq!(graph.shortest_distance(0, 5), 11);
}
