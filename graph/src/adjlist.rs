use std::collections::HashSet;
use std::collections::hash_map::{HashMap, Entry};
#[derive(Debug)]
pub struct Adjlist{
    // use an adjacent list to represent graph
    // in the HashMap, key is the node, and Vec is his neighbors
    adj_list: HashMap<String, HashSet<String>>,
}

impl Adjlist{
    pub fn new() -> Adjlist{
        Adjlist{
            adj_list:HashMap::new(),
        }
    }

    pub fn get_neighbors(&self, start: &String) -> Option<&HashSet<String>>{
        self.adj_list.get(start)
    }

    pub fn node_num(&self) ->usize{
        self.adj_list.len()
    }
    // add an edge between two neighbors
    pub fn add_edge(&mut self, node1: String, node2: String){

        match self.adj_list.entry(node1.clone()) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().insert(node2.clone());
            },
            Entry::Vacant(entry) => {
                let mut set = HashSet::new();
                set.insert(node2.clone());
                entry.insert(set);
            }
        }

        match self.adj_list.entry(node2.clone()) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().insert(node1.clone());
            },
            Entry::Vacant(entry) => {
                let mut set = HashSet::new();
                set.insert(node1.clone());
                entry.insert(set);
                //entry.insert(HashSet::new());
            }
        }
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
