use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Node(u8, usize);

impl Node {
    fn new(val: u8) -> Node {
        let newval = match val {
            b'S' => b'a' as usize,
            b'E' => b'z' as usize,
            _ => val as usize,
        };
        Node(val, newval)
    }
}

#[derive(Eq, Debug, Copy, Clone, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}
// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
        // .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(adj_list: &Vec<Vec<usize>>, start: usize, end: Vec<usize>) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    let mut candidates = Vec::new();
    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // println!("{} {}", cost, position);
        // Alternatively we could have continued to find all shortest paths

        if end.contains(&position) {
            candidates.push(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for node_pos in &adj_list[position] {
            let next = State {
                cost: cost + 1,
                position: *node_pos,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    candidates.iter().min().copied()
}

fn main() {
    let input = include_bytes!("../input.txt")
        .split(|&c| c == b'\n')
        .filter(|line| line.len() > 0);
    let ncols = input.clone().next().unwrap().len();
    let nodes = input
        .clone()
        .map(|row| row.iter().map(|&b| Node::new(b)).collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();
    let start = nodes.iter().position(|n| n.0 == b'E').unwrap();
    let end = nodes
        .iter()
        .enumerate()
        .filter_map(|(i, n)| match n.1 as u8 {
            b'a' => Some(i),
            _ => None,
        })
        .collect::<Vec<_>>();
    let mut adj_list: Vec<Vec<_>> = Vec::new();
    for (i, node) in nodes.iter().enumerate() {
        adj_list.push(
            vec![
                i as i32 - ncols as i32, // up
                i as i32 + ncols as i32, // down
                match i % ncols {
                    0 => -1, // lazy hack
                    _ => i as i32 - 1,
                }, // left
                match (i + 1) % ncols {
                    0 => -1, // lazy hack
                    _ => i as i32 + 1,
                }, // right
            ]
            .into_iter()
            .filter_map(|i| match nodes.get(i as usize) {
                Some(othernode) if othernode.1 >= node.1 - 1 => Some(i as usize),
                _ => None,
            })
            .collect::<Vec<_>>(),
        )
    }
    println!(
        "Shortest path from start to end is {} steps",
        shortest_path(&adj_list, start, end).unwrap()
    );
}
