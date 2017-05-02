/**!
 * ASSUMPTIONS
 *
 *- The edges in graph are undirected.
 *- Based on DFS, the result path may not be the shortest.
 *- Enter "exit" or just press enter button can break and stop the programme.
 *
 */


extern crate graph;
use graph::adjlist::*;
use std::env;
use std::fs::File;
use std::io::{Read,stdin};
use std::collections::HashSet;

fn main() {
    //initialize graph
    let graph = graph_builder(readin_file());
    // print the adjacent list for the graph
    // println!("{:?}", graph);
    input_node(&graph);
}

// find a path from start node to end node, using depth-first-search
fn find_path(graph: &Adjlist, start: String, end: String) -> Vec<String>{
    let mut path=Vec::new();
    path.push(start.clone());
    dfs(graph, start, end.clone(), &mut path);
    if path.len()==graph.node_num() && path[path.len()-1]!=end.clone(){
        return Vec::new();
    }
    else if path[path.len()-1] != end.clone() {
        return Vec::new();
    }
    else{
        return path;
    }
}
#[cfg(test)]
mod find_path_tests {
    use super::*;

    #[test]
    fn find_path_test() {
        let graph = graph_builder("a b d\nb a d\nc\nd c\n".to_string());

        assert_eq!(vec!["a", "b", "d", "c"], find_path(&graph, "a".to_owned(), "c".to_owned()));
    }

    #[test]
    fn none_find_path_test() {
        let graph = graph_builder("a b d\nb a d\nc\nd c\n".to_string());
        let test:Vec<String> = Vec::new();
        assert_eq!(test, find_path(&graph, "a".to_owned(), "s".to_owned()));
    }

    #[test]
    fn none_node_find_path_test() {
        let graph = graph_builder(" ".to_string());
        let test:Vec<String> = Vec::new();
        assert_eq!(test, find_path(&graph, "a".to_owned(), "d".to_owned()));
    }
}

// the dfs helper function for find_path
fn dfs(graph: &Adjlist, start: String, end: String, path: &mut Vec<String>){
    if start==end{
        return;
    }
    if path.len()==graph.node_num(){
        return;
    }

    let mut vec = HashSet::new();
    match graph.get_neighbors(&start){
        Some(v) => vec = v.clone(),
        None => println!("No such start node")
    }
    for i in vec.iter(){
        // if path.contains(&i)==false && ()
        if path.contains(&i.to_owned())==false{
            path.push(i.clone());
            // if vec[i]==end.to_owned(){
            //     return;
            // }
            dfs(graph, i.clone(), end.to_owned(), path);
        }
        if path[path.len()-1]==end.clone(){
            return;
        }
    }
}


fn readin_file() -> String{
    let mut filepath = String::new();
    for argument in env::args() {
        filepath = argument;
    }
    //let mut file;
    let mut lines = String::new();
    match File::open(filepath){
        Err(_) => panic!("No such files!"),
        Ok(mut f) => {
            match f.read_to_string(&mut lines){
                Err(_) => panic!("File is empty!"),
                Ok(_) => println!("Read file succeed!")
            }
        }
    }

    lines
}

fn graph_builder(lines: String) -> Adjlist{
    let mut graph=Adjlist::new();
    for line in lines.lines(){
        let mut iter = line.split_whitespace();
        let mut head = String::new();
        match iter.next(){
            Some(h) => {
                head = h.to_string();
                graph.add_edge(head.clone(), head.clone());
            },
            None =>{}
        }
        while let Some(next) = iter.next(){
            graph.add_edge(head.clone(), next.to_string());
        }
     }
    graph
}
#[cfg(test)]
mod graph_builder_tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn graph_builder_test() {
        let mut test = HashMap::new();
        test.insert("b".to_string(), vec!["a".to_string(), "d".to_string()]);
        test.insert("a".to_string(), vec!["b".to_string(), "d".to_string()]);
        test.insert("d".to_string(), vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        test.insert("c".to_string(), vec!["d".to_string()]);
        // assert_eq!(test, graph_builder("a b d\nb a d\nc\nd c\n".to_string()).adj_list);
        assert_eq!(test.len(),graph_builder("a b d\nb a d\nc\nd c\n".to_string()).node_num());
        assert_eq!(test.get(&String::from("a")),graph_builder("a b d\nb a d\nc\nd c\n".to_string()).get_neighbors(&String::from("a")));
        assert_eq!(test.get(&String::from("b")),graph_builder("a b d\nb a d\nc\nd c\n".to_string()).get_neighbors(&String::from("b")));
        assert_eq!(test.get(&String::from("c")),graph_builder("a b d\nb a d\nc\nd c\n".to_string()).get_neighbors(&String::from("c")));
        assert_eq!(test.get(&String::from("d")),graph_builder("a b d\nb a d\nc\nd c\n".to_string()).get_neighbors(&String::from("d")));

    }

}

fn input_node(graph: &Adjlist){
    loop {
        let mut input = String::new();
        let mut vec = Vec::new();
        stdin().read_line(&mut input).unwrap();
        for word in input.split_whitespace(){
            vec.push(word);
        }
        if vec.len()==0{
            break;
        }
        if vec[0].to_owned().to_lowercase() == "exit"{
            break;
        }
        else{
            if vec.len() != 2{
                println!("Please Input two Nodes");
            }else{
                let result = find_path(&graph, vec[0].to_string(), vec[1].to_string());
                if result.len() == 1 {
                    match graph.get_neighbors(&result[0]){
                        Some(_) => println!("{} {}", result[0], result[0]),
                        None => println!("No such path or node!")
                    }
                }else if result.len() == 0{
                    // no path, assume that when start node and end node are the same, there is no path
                    println!("No such path or node!");
                }else{
                    //print the path
                    for i in 0..result.len(){
                        print!("{} ", result[i]);
                    }
                    println!("");
                }
            }
        }
    }
}
