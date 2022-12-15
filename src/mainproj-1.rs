// import 
use rand::Rng;
use std::fs::File;
use std::sync::Arc;
use std::cmp::Reverse;
use std::io::BufRead;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::{ HashMap, HashSet};
use std::iter::successors;
use std::io::prelude::*;
use std::fs::File;
use std::io::{BufReader, BufRead};


fn main() {
    let file = File::open("my_text_file.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        println!("{}", line);
    }

    let mut n=1000;
    let mut edges = read_file("edges_data.txt");
    let mut rank = rank(&mut edges, n);

     //Graph with connection between 100 NODES
    let mut graph = Graph::new_undirected();
    for node in nodes {
        graph.add_node(node);
    }
    // Add the edges to the graph
    for (n1, n2) in edges {
        graph.add_edge(n1, n2, 1);
    }

    // Graph edges between the top100 nodes 
    let mut graph = Graph::create_directed(n,&edges);
    graph.outedges.retain(|edges| !edges.is_empty());

    // HashMap of top 100 nodes 
    let mut graph_hashmap = HashMap::new();
    for (i, 100) in top50.iter().enumerate() {
        graph_hashmap.insert(*num, &adj_list[i]);
    }


     // View the graph
    for (i, edges) in graph.add_edge {
        if edges.is_empty() {
            continue;
        }
        println!("({}) Node: {} - Number of Edges: {:?}",i, rank[i].0, edges);
    }
----------------------------------------------------------------------------------------------------------------------------------
    //Function DFS
    // Distance from the vector, return list of verticies, creat function that computes the algorithm
    fn dfs(graph: &Vec<Vec<usize>>, source: usize, visited: &mut Vec<bool>) {
        visited[source] = true;
        
        for neighbor in graph[source].iter() {
            if !visited[*neighbor] {
                dfs(graph, *neighbor, visited);
                }
            }
        }
        
        fn dfs2() {
            let mut graph: Vec<Vec<usize>> = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
            let mut visited = vec![false, false, false];
        
            dfs(&graph, 0, &mut visited);
    }
----------------------------------------------------------------------------------------------------------------------------------
// Adjacnecy list 

fn create_adj_list(edges: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut adj_list = vec![Vec::new(); edges.len()];

    for (u, v) in edges {
        adj_list[*u].push(*v);
        adj_list[*v].push(*u);
    }

    adj_list
}

// Read text file and return list of edges
fn read_edges(filename: &str) -> Vec<(usize, usize)> {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut edges = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut words = line.split_whitespace();
        let u: usize = words.next().expect("Failed to parse first node index").parse().expect("Failed to parse first node index as usize");
        let v: usize = words.next().expect("Failed to parse second node index").parse().expect("Failed to parse second node index as usize");
        edges.push((u, v));
    }
    edges
}
----------------------------------------------------------------------------------------------------------------------------------
//FUNCTION FOR BFS
// MARK COMPONENT BFS

fn bfs(graph: &Vec<Vec<usize>>, start: usize) {
    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    visited[start] = true;

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        // You can add code here to perform some action on the current node.

        // Add the current node's neighbors to the queue.
        for neighbor in &graph[current] {
            if !visited[*neighbor] {
                queue.push_back(*neighbor);
                visited[*neighbor] = true;
            }
        }
    }
}

fn bfs_distance(graph: &Vec<Vec<usize>>, start: usize, end: usize) -> i32 {
    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    visited[start] = true;

    let mut distance = 0;
    while !queue.is_empty() {
        let size = queue.len();
        for _ in 0..size {
            let current = queue.pop_front().unwrap();
            if current == end {
                return distance;
            }
            for neighbor in &graph[current] {
                if !visited[*neighbor] {
                    queue.push_back(*neighbor);
                    visited[*neighbor] = true;
                }
            }
        }
        distance += 1;
    }
    -1
}

fn mark_component_bfs(vertex:Vertex, graph:&Graph, component:&mut Vec<Option<Component>>, component_no:Component) {
    component[vertex] = Some(component_no);
    
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(vertex);
    
    while let Some(v) = queue.pop_front() {
        for w in graph.outedges[v].iter() {
            if let None = component[*w] {
                component[*w] = Some(component_no);
                queue.push_back(*w);
            }
        }
    }
}

----------------------------------------------------------------------------------------------------------------------------------
// CREATE A GRAPH TYPE
    type Vertex = usize;
    type List = Vec<(Vertex,Vertex)>;
    type Next = Vec<Vec<Vertex>>;
    type Component = usize;
    struct Graph {
        n: usize, // vertex labels in {0,...,n-1}
        outter: Next,
    }

    // Must change direction of edges
    fn edges(list:&List)
            -> List {
        let mut list = vec![];
        for (u,v) in list {
            list.push((*v,*u));
        }
        list
    }

    // Executre graph
impl Graph {
    fn directed(&mut self,edges:&List) {
        for (u,v) in edges {
            self.outter[*u].push(*v);
        }
    }
    fn lists(&mut self) {
        for l in self.outter.iter_mut() {
            l.sort();
        }
    }

    fn create_directed(n:usize,edges:&List)  -> Graph {
        let mut g = Graph{n,outter:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }
    
    fn no_directed(n:usize,edges:&List)-> Graph {
        let mut g = Self::create_directed(n,edges);
        g.directed_edges(&edges(edges));
        g.lists();
        g                                        
    }


// Page Rank for 100 verticies 
fn pagerank(data: &mut Vec<(Vertex, Vertex)>, n_vertices:usize) -> Vec<(Vertex, f64)> { 
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(0..=(data.len()));
    let random = random as usize;
    let mut leaf = data[random].0;
    let mut pagerank: Vec<(Vertex, Vertex)> = Vec::new();

    for i in 0..n_vertices {
        pagerank.push((i, 0));
    }
    let mut outter = Vec::new();
    let mut outteredges = 0;

    for _i in 0..100 {
        for j in 0..data.len() {
            if data[j].0 == leaf {
                outteredges += 1;
                outter.push(data[j].1);
            }
        }

        while outteredges == 0 {
            let mut rng = rand::thread_rng();
            leaf = rng.gen_range(0..=(data.len()));
            leaf = data[leaf as usize].0;
        }

        for j in 0..data.len() {
            if data[j].0 == leaf {
                outteredges += 1;
                outter.push(data[j].1);
            }
        }

        let rand = rng.gen_range(0..10);
        if rand == 0 {
            leaf = rng.gen_range(0..=(data.len()));
            leaf = data[leaf as usize].0;
        }

        else {
            
            let mut rng = rand::thread_rng();
            leaf = rng.gen_range(0..outteredges);
            leaf = outter[leaf as usize];
        }

        for i in 0..pagerank.len() {
            if pagerank[i].0 == leaf {
                pagerank[i].1 += 1;
            }
        }

        out = Vec::new();
        out_edges = 0;
    }

    // Sort pagerank vector by the second element of the tuple
    pagerank.sort_by(|a, b| b.1.cmp(&a.1));

    // This vector contains the page rank of the top 100 nodes
    let mut t100: Vec<(Vertex, f64)> = Vec::new();
    for i in 0..100 {
        t100.push((pagerank[i].0 as Vertex, pagerank[i].1 as f64 / 100.0));
    }
    return t100;
}
