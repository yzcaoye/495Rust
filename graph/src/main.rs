use std::collections::HashMap;

fn main() {

    //initialize graph
    let mut graph=Adjlist::new();

    // add edges
    graph.add_edge(String::from("a"),String::from("b"));
    graph.add_edge(String::from("a"),String::from("d"));
    graph.add_edge(String::from("b"),String::from("a"));
    graph.add_edge(String::from("b"),String::from("d"));
    graph.add_edge(String::from("d"),String::from("c"));

    // print the adjacent list for the graph
    println!("{:?}", graph);

    // test: a->d, a->c, a->b, a->a
    // let mut result=find_path(&graph, String::from("a"), String::from("d"));
    // let mut result=find_path(&graph, String::from("a"), String::from("c"));
    let mut result=find_path(&graph, String::from("a"), String::from("b"));
    // let mut result=find_path(&graph, String::from("a"), String::from("a"));

    // print the path
    if result.len()<=1{
        // no path, assume that when start node and end node are the same, there is no path
        println!("no such path!");
    }
    else{
        //print the path
        for i in 0..result.len(){
            print!("{} ", result[i]);
        }
        println!("");
    }

}

#[derive(Debug,Clone)]
struct Node{
    node_name: String,

}

impl Node{
    fn new(nodename:String) -> Node{
        Node{
            node_name: nodename,
        }
    }
}

#[derive(Debug)]
struct Adjlist{
    // use an adjacent list to represent graph
    // in the HashMap, key is the node, and Vec is his neighbors
    adj_list: HashMap<String, Vec<String>>,
}

impl Adjlist{
    fn new() -> Adjlist{
        Adjlist{
            adj_list:HashMap::new(),
        }
    }

    // add an edge between two neighbors
    fn add_edge(&mut self, node1: String, node2: String){

        if self.adj_list.contains_key(&node1)==false{
            let mut vec:Vec<String>=Vec::new();
            self.adj_list.insert(node1.clone(),vec);
        }
        if self.adj_list.contains_key(&node2)==false{
            let mut vec:Vec<String>=Vec::new();
            self.adj_list.insert(node2.clone(),vec);
        }
        let mut vec1 = self.adj_list.get(&node1).unwrap().to_owned();
        if vec1.contains(&node2)==false{
            vec1.push(node2.to_owned());
        }
        self.adj_list.insert(node1.to_owned(),vec1);

        let mut vec2 = self.adj_list.get(&node2).unwrap().to_owned();
        if vec2.contains(&node1)==false{
            vec2.push(node1.to_owned());
        }
        self.adj_list.insert(node2.to_owned(),vec2);
    }

}

// find a path from start node to end node, using depth-first-search
fn find_path(graph: &Adjlist, start: String, end: String) -> Vec<String>{
    let mut path=Vec::new();
    path.push(start.clone());
    dfs(graph, start, end.clone(), &mut path);
    if path.len()==graph.adj_list.len() && path[path.len()-1]!=end.clone(){
        return Vec::new();
    }
    else if path[path.len()-1] != end.clone() {
        return Vec::new();
    }
    else{
        return path;
    }
}

// the dfs helper function for find_path
fn dfs(graph: &Adjlist, start: String, end: String, path: &mut Vec<String>){
    if start.to_owned()==end.to_owned(){
        return;
    }
    if path.len()==graph.adj_list.len(){
        return;
    }

    let mut vec = graph.adj_list.get(&start).unwrap();
    for i in 0..vec.len(){
        // if path.contains(&i)==false && ()
        if path.contains(&vec[i].clone())==false{
            path.push(vec[i].clone());
            if vec[i]==end.to_owned(){
                return;
            }
            dfs(graph, vec[i].clone(), end.to_owned(), path);
        }
    }

}
