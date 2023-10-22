use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq, Eq, Hash, Debug)]
struct Vertex {
    pos: (usize, usize),
    val: u8,
}

impl Vertex {
    fn new(pos: (usize, usize), val: u8) -> Vertex {
        let val = match val {
            b'S' => b'a' - 1,
            b'E' => b'z' + 1,
            _ => val,
        };
        Vertex { pos, val }
    }
}

fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

fn main() {
    let grid = include_bytes!("../input.txt")
        .split(|&c| c == b'\n')
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, &b)| Vertex::new((i, j), b))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut edges = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, node) in row.iter().enumerate() {
            let nodes = vec![
                (i as i32 + 1, j as i32),
                (i as i32 - 1, j as i32),
                (i as i32, j as i32 + 1),
                (i as i32, j as i32 - 1),
            ]
            .into_iter()
            .filter_map(|(i, j)| grid.get(i as usize).and_then(|row| row.get(j as usize)))
            .filter(|othernode| node.val <= othernode.val + 1)
            .collect::<Vec<_>>();
            edges.insert(node, nodes);
        }
    }
    println!("{:?}", edges);

}
