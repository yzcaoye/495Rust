#[doc="
Consumes text from standard input, and Print a list of word-frequency counts in descending order of frequency to the standard output.

Assumptions:
1. Words in a line are seperated by whitespace.
2. Do not differentiate between upppercase and lowercase. For example, 'HELLO' and 'hello' are the same word.
3. The left-most and right-most punctuations in a word are trimmed.
"]


use std::io::{BufRead,BufReader};
use std::collections::HashMap;
use std::fs::File;
use std::env;

fn main() {
    //read input data from txt file
    // let filename =String::from("test.txt");
    let filename =String::from("input_inputs.txt");
    // let filename =String::from("input_larger.txt");
    // let filename =String::from("input_some.txt");

    //type filename in commandline
    // let filename: String = env::args().nth(1).unwrap();

    let words = read_input(&filename);
    let mut words_frequencies: HashMap<String, u32> = HashMap:: new();

    //calculate words_frequencies
    cal_frequency(&words, &mut words_frequencies);

    //print frequencies out in descending order
    print_in_descend(&words_frequencies);



}

//read input data from "filename", return a vector of all the words in file
fn read_input(filename: &String) -> Vec<String>{
    let file=File::open(filename).unwrap();
    let reader=BufReader::new(file);
    let mut input: Vec<String> = vec![];
    for line in reader.lines(){
        for word in line.unwrap().split_whitespace(){
            // println!("{}", word);
            input.push(String::from(word));
        }
    }

    input

}

//count frequency for each word
fn cal_frequency(words: &Vec<String>, words_frequencies: &mut HashMap<String, u32> ){
    for word in words{
        let new_word = remove_punctuation(word.to_owned());
        let count = words_frequencies.entry(new_word).or_insert(0);
        *count += 1;
    }
    //  println!("{:?}", words_frequencies);
}

//remove punctuation in word
fn remove_punctuation(word: String) -> String{
    let punctuations: &[_] = &[',', '.', '#', '!', '@', '$', '%', '&', '*', '(', ')', ':', ';', '\'', '\"', '<', '>', '?', '/','^','{','}','[',']'];
    let new_word=word.trim_matches(punctuations).to_lowercase();
    new_word
}

fn print_in_descend(words_frequencies: &HashMap<String,u32>){
    let mut sort_vec: Vec<_> = words_frequencies.iter().collect();
    sort_vec.sort_by(|a, b| b.1.cmp(a.1));
    // println!("{:?}", sort_vec);
    for item in sort_vec{
        println!("{}: {}", item.0, item.1);
    }
}

#[cfg(test)]
mod read_input_test{
    use super::read_input;

    #[test]
    fn empty_inputfile(){
        let input = read_input(&String::from("empty.txt"));
        let expected: Vec<String>= vec![];
        assert_eq!(expected, input);
    }

    #[test]
    fn hello_world_test(){
        let input = read_input(&String::from("test.txt"));
        let expected = vec!["hello","world,","bye","world"];
        assert_eq!(expected,input);
    }
}

#[cfg(test)]
mod remove_punctuation_tests{
    use super::remove_punctuation;

    #[test]
    fn trim_punctuation(){
        assert_eq!("test".to_owned(),remove_punctuation("$%test,.'.\"".to_owned()));
    }

    #[test]
    fn not_need_trim(){
        assert_eq!("test".to_owned(),remove_punctuation("test".to_owned()));
    }

    #[test]
    fn trim_to_lowercase(){
        assert_eq!("test".to_owned(),remove_punctuation("^)/TeSt?{}[]".to_owned()));
    }
}

#[cfg(test)]
mod cal_frequency_tests{
    use super::cal_frequency;
    use std::collections::HashMap;

    #[test]
    fn test_one_two_three(){
        let mut vec_input: Vec<String> = vec![];
        vec_input.push("one".to_owned());
        vec_input.push("two".to_owned());
        vec_input.push("two".to_owned());
        vec_input.push("three".to_owned());
        vec_input.push("three".to_owned());
        vec_input.push("three".to_owned());

        let mut words_frequencies: HashMap<String, u32> = HashMap:: new();

        cal_frequency(&vec_input, &mut words_frequencies);

        assert_eq!(words_frequencies.get("one"),Some(&(1 as u32)));
        assert_eq!(words_frequencies.get("two"),Some(&(2 as u32)));
        assert_eq!(words_frequencies.get("three"),Some(&(3 as u32)));
    }

    #[test]
    fn test_zero(){
        let vec_input: Vec<String> = vec![];
        let mut words_frequencies: HashMap<String, u32> = HashMap:: new();

        cal_frequency(&vec_input, &mut words_frequencies);

        assert_eq!(words_frequencies.get("one"),None);
        assert_eq!(words_frequencies.get("two"),None);
        assert_eq!(words_frequencies.get("three"),None);

    }


}
