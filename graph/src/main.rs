fn main() {
    let mut graph=Adjmatrix::new(4);
    let nodea=Node::new(0);
    let nodeb=Node::new(1);
    let nodec=Node::new(2);
    let noded=Node::new(3);
    graph.add_edge(nodea,nodeb);
    graph.add_edge(nodea,noded);
    graph.add_edge(nodeb,nodea);
    graph.add_edge(nodeb,noded);
    graph.add_edge(noded,nodec);
    for i in 0..4{
        for j in 0..4{
            print!("{}", graph.adj_matrix[i][j]);
        }
        println!("");
    }
    let mut result=find_path(&graph, nodea, noded);
    if result.len()<=1{
        println!("no such path!");
    }
    else{
        for i in 0..result.len(){
            print!("{} ", result[i]);
        }
        println!("");
    }
}

#[derive(Debug,Copy,Clone)]
struct Node{
    node_id: usize,

}

impl Node{
    fn new(nodeid:usize) -> Node{
        Node{
            node_id: nodeid,
        }
    }
}

#[derive(Debug)]
struct Adjmatrix{
    node_num: usize,
    adj_matrix: Vec<Vec<usize>>,
}

impl Adjmatrix{
    fn new(nodenum: usize) -> Adjmatrix{
        Adjmatrix{
            node_num: nodenum,
            adj_matrix: vec![vec![0;nodenum];nodenum],
        }
    }

    fn add_edge(&mut self, node1: Node, node2: Node){
        self.adj_matrix[node1.node_id][node2.node_id]=1;
        self.adj_matrix[node2.node_id][node1.node_id]=1;
    }

}

fn find_path(graph: &Adjmatrix, start: Node, end: Node) -> Vec<usize>{
    let mut path=Vec::new();
    path.push(start.node_id);
    dfs(graph, start, end, &mut path);
    if path.len()==graph.node_num && path[path.len()-1]!=end.node_id{
        return Vec::new();
    }
    else if(path[path.len()-1]!=end.node_id){
        return Vec::new();
    }
    else{
        return path;
    }
}

fn dfs(graph: &Adjmatrix, start: Node, end: Node, path: &mut Vec<usize>){
    if start.node_id==end.node_id{
        return;
    }
    if path.len()==graph.node_num{
        return;
    }
    for i in 0..graph.node_num{
        if path.contains(&i)==false && (graph.adj_matrix[start.node_id][i]==1 || graph.adj_matrix[i][start.node_id]==1) {
            path.push(i);
            println!("current node: {}", i);
            if end.node_id==i{
                return;
            }

            dfs(graph, Node::new(i), end, path);
            // path.pop();
        }

    }

}
