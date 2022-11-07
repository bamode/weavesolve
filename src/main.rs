mod dict;

use std::collections::{HashMap, HashSet, VecDeque};

use clap::Parser;
use colored::Colorize;

use crate::dict::DICT;

fn main() {
    let cli = Cli::parse();
    let graph = build_graph_from_dict(&DICT);
    let path = find_shortest_path(graph, &cli.start, &cli.stop);
    print_path(&path, &cli.stop);
}

/// A helper function for printing the solution path nicely
fn print_path(path: &Vec<&str>, stop: &str) {
    for &word in path.iter() {
        for (cword, cstop) in word.chars().zip(stop.chars()) {
            if cword == cstop {
                print!("{}", format!("{}", cword).green());
            } else {
                print!("{}", format!("{}", cword));
            }
        }
        if word == stop {
            break
        } else {
            print!(" -> ");
        }
    }
    println!();
}

/// Defines the CLI for Weavesolve
#[derive(Parser, Debug)]
struct Cli {
    /// Starting word
    start: String,

    /// Ending word
    stop: String,
}

/// We can represent the word ladder data as a `HashMap` keyed by strings
/// with `Vec`s of strings as values
type Graph<'g> = HashMap<&'g str, Vec::<&'g str>>;

/// A simple wrapper around the rust VecDeque type, in order to match the
/// `enqueue` and `dequeue` functions used in the breadth-first search
/// pseudocode
struct Queue<T> {
    queue: VecDeque<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { queue: VecDeque::new() }
    } 

    fn enqueue(&mut self, val: T) {
        self.queue.push_back(val);
    }

    fn dequeue(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

/// Determines whether two strings of the same length
/// differ by only one character. Will behave unexpectedly
/// if the strings are different lengths because of the use
/// of `zip`. No check is performed because this problem
/// is solved by only having a dictionary of 4-letter words
/// to reference.
fn is_one_char_diff(s1: &str, s2: &str) -> bool {
    let iter = s1.chars().zip(s2.chars());
    let mut counter = 0;

    for (c1, c2) in iter {
        if c1 != c2 {
            counter += 1;
        }
    }

    if counter == 1 {
        true
    } else {
        false
    }
}

/// Takes a dictionary of words and then builds the graph of words
/// that are connected one they differ by a single letter only. 
/// Because we are using a `HashMap` to represent a graph, we make
/// sure to symmetrically insert nodes, and then we only have to examine 
/// half of all possible pairs of words.
fn build_graph_from_dict<'a>(dict: &[&'a str]) -> Graph<'a> {
    let mut graph: Graph = HashMap::new();
    for i in 0..dict.len() {
        // because we will insert connections symmetrically, we only need
        // to check pairs from `i + 1` forward
        for j in i + 1..dict.len() {
            if is_one_char_diff(dict[i], dict[j]) {
                graph
                    .entry(dict[i])
                    .and_modify(|connections| connections.push(dict[j])) // if the entry already exists, we want to push the next match
                    .or_insert(vec![dict[j]]); // otherwise we make the entry
                
                graph
                    .entry(dict[j])
                    .and_modify(|connections| connections.push(dict[i]))
                    .or_insert(vec![dict[i]]);
            }
        }
    }

    graph
}

/// A general breadth-first search algorithm defined on our `Graph` type. 
/// The most significant deviation from this pseudocode is that we cannot easily
/// attach some notion of a parent to our graph nodes. Presumably, this implementation
/// assumes a more custom graph type that can hold this additional data in each node.
/// Instead, I simply make a new `HashMap` where each entry points to that word's parent
/// string. We are guaranteed to not overwrite this value at any point because a breadth-first
/// search such as this is constructing a tree where each node has exactly one parent.
/// 
/// Pseudocode to be translated into Rust code
/// 
/// ```
///     procedure BFS(G, root) is
///         let Q be a queue
///         label root as explored
///         Q.enqueue(root)
///         while Q is not empty do
///             v = Q.dequeue()
///             if v is the goal then
///                 return v
///             for all edges from v to w in G.adjacentEdges(v) do
///                 if w is n ot labeled as explored then
///                     label w as explored
///                     w.parent = v 
///                     Q.enqueue(w)
/// ```
fn bfs<'g>(graph: Graph<'g>, root: &'g str, goal: &'g str) -> (&'g str, HashMap<&'g str, &'g str>) {
    let mut q = Queue::new();
    let mut visited = HashSet::new();
    let mut parent_map = HashMap::new();
    visited.insert(root);
    q.enqueue(root);
    while !q.is_empty() {
        let v = q.dequeue().unwrap(); // `unwrap()` is safe here since we checked not empty
        if v == goal {
            return (v, parent_map)
        }
        if let None = graph.get(v) { 
            eprintln!("{} is not a valid word!", v); 
            std::process::exit(1); 
        }
        
        for &entry in graph[v].iter() {
            if let None = visited.get(entry) {
                visited.insert(entry);
                parent_map.insert(entry, v);
                q.enqueue(entry);
            }
        }
    }
    
    // Rust doesn't love something like a `while` loop that will eventually return from within
    // so we mark the end of the function here as `unreachable!()`
    unreachable!()
}

/// Use our `bfs` implementation to get the result we actually want: the solution path. 
/// This simply requires taking the parent map, the end word, and the start word, and
/// walking backward from there to construct the actual solution path. Then we simply reverse
/// the result of that to have the path in the order we want.
fn find_shortest_path<'g>(graph: Graph<'g>, start: &'g str, end: &'g str) -> Vec<&'g str> {
    let (sol, parent_map) = bfs(graph, start, end);
    let mut ptr = sol;
    let mut path = Vec::new();
    while ptr != start {
        path.push(ptr);
        ptr = parent_map[ptr];
    }
    path.push(start);
    let path: Vec<&str> = path.into_iter().rev().collect();
    
    path
}