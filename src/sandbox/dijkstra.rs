use itertools::Itertools;
use std::collections::HashSet;

pub struct Graph {
    adjacency_grid: Vec<Vec<Option<usize>>>,
    number_of_nodes: usize,
}

impl Graph {
    pub fn new(number_of_nodes: usize) -> Graph {
        let row: Vec<Option<usize>> = (0..number_of_nodes).map(|_| None).collect_vec();
        let adjacency_grid = (0..number_of_nodes).map(|_| row.clone()).collect_vec();
        Graph {
            adjacency_grid,
            number_of_nodes,
        }
    }

    pub fn zero_out_self_weights(&mut self) {
        for i in 0..self.number_of_nodes {
            self.adjacency_grid[i][i] = Some(0);
        }
    }
    pub fn add_weight(&mut self, node_a: usize, node_b: usize, weight: usize) {
        self.adjacency_grid[node_a][node_b] = Some(weight);
        self.adjacency_grid[node_b][node_a] = Some(weight);
    }

    pub fn get_adjacent_nodes(&self, node: usize) -> Vec<usize> {
        let mut adjacent_nodes = vec![];
        for (other_node, row) in self.adjacency_grid.iter().enumerate() {
            if row[node].is_some() {
                adjacent_nodes.push(other_node)
            }
        }
        adjacent_nodes
    }
    pub fn shortest_distance(&self, node_a: usize, node_b: usize) -> usize {
        let mut unvisited_nodes: HashSet<usize> = HashSet::new();
        (0..self.number_of_nodes).for_each(|node| {
            unvisited_nodes.insert(node);
        });
        let mut shortest_distance: Vec<usize> = vec![usize::MAX; self.number_of_nodes];
        let mut current_node = node_a;

        // Start setting values for our start point
        shortest_distance[node_a] = 0; // Duh

        // We will go through all adjacent non-visited nodes
        loop {
            unvisited_nodes.remove(&current_node);
            let adjacent_nodes = self.get_adjacent_nodes(current_node);
            for adjacent_node in adjacent_nodes
                .iter()
                .filter(|&node| unvisited_nodes.contains(node))
            {
                let distance = shortest_distance[current_node]
                    + self.adjacency_grid[current_node][*adjacent_node]
                        .expect("Has to be adjacent since we filtered");
                if shortest_distance[*adjacent_node] > distance {
                    shortest_distance[*adjacent_node] = distance;
                }
            }
            if let Some(next_node) = unvisited_nodes
                .iter()
                .sorted_by(|&a, &b| shortest_distance[*a].cmp(&shortest_distance[*b]))
                .next()
            {
                current_node = *next_node;
            } else {
                return shortest_distance[node_b];
            }
        }
    }
}
