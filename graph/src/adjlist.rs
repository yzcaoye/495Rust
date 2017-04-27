use std::collections::HashMap;
#[derive(Debug)]
pub struct Adjlist{
    // use an adjacent list to represent graph
    // in the HashMap, key is the node, and Vec is his neighbors
    pub adj_list: HashMap<String, Vec<String>>,
}

impl Adjlist{
    pub fn new() -> Adjlist{
        Adjlist{
            adj_list:HashMap::new(),
        }
    }

    // add an edge between two neighbors
    pub fn add_edge(&mut self, node1: String, node2: String){

        if self.adj_list.contains_key(&node1)==false{
            let vec:Vec<String>=Vec::new();
            self.adj_list.insert(node1.clone(),vec);
        }
        if self.adj_list.contains_key(&node2)==false{
            let vec:Vec<String>=Vec::new();
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

#[cfg(test)]
mod add_edge_tests {
    use super::*;

    #[test]
    fn single_add_edge_test() {
        let mut graph = Adjlist::new();
        graph.add_edge("b".to_string(), "a".to_string());
        let mut test = HashMap::new();
        test.insert("b".to_string(), vec!["a".to_string()]);
        test.insert("a".to_string(), vec!["b".to_string()]);
        assert_eq!(test, graph.adj_list);
    }
    #[test]
    fn multiple_add_edge_test() {
        let mut graph = Adjlist::new();
        graph.add_edge("b".to_string(), "a".to_string());
        graph.add_edge("b".to_string(), "c".to_string());
        let mut test = HashMap::new();
        test.insert("b".to_string(), vec!["a".to_string(),"c".to_string()]);
        test.insert("a".to_string(), vec!["b".to_string()]);
        test.insert("c".to_string(), vec!["b".to_string()]);
        assert_eq!(test, graph.adj_list);
    }
}
