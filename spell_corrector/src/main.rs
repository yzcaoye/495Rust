use std::io::{BufRead,BufReader};
use std::collections::HashMap;
use std::fs::File;
use std::env;

fn main() {
    // let file_corpus =String::from("corpus.txt");

    println!("Example type in: ");
    println!("cargo run corpus.txt input.txt");
    let file_corpus: String = env::args().nth(1).unwrap();

    // Get the corpus, save all correct words in HashMap
    let corpus = get_corpus(&file_corpus);

    // let mut file_input = String:: from("input.txt");
    let file_input: String =env::args().nth(2).unwrap();

    // println!("{}, {}",file_corpus, file_input);
    input_corrector(&file_input, &corpus);

}

/// Read the corpus and store all correct words in HashMap
fn get_corpus(filename: &String) -> HashMap<String, usize>{
    let mut corpus = HashMap::new();
    let file=File::open(filename).unwrap();
    let reader=BufReader::new(file);
    for line in reader.lines(){
        for word in line.unwrap().split_whitespace(){
            // println!("{}", word);
            let count = corpus.entry(word.to_owned()).or_insert(0);
            *count += 1;
            // input.push(String::from(word));
        }
    }
    corpus
}

/// Read the input words and give corrector to each word
fn input_corrector(filename: &String, corpus: &HashMap<String, usize>){
    let file=File::open(filename).unwrap();
    let reader=BufReader::new(file);

    for line in reader.lines(){
        for word in line.unwrap().split_whitespace(){
            // The word is correct
            if edit0(word.to_owned(),corpus){
                println!("{}", word);
                // println!("****");
            }

            else{
                let correctors_edit1=edit1(&mut String::from(word),corpus);
                if correctors_edit1.len()==0{
                    // println!("{} Need edit2!",word);
                    let correctors_edit2=edit2(&mut String::from(word),corpus);
                    // More than 2 edit distance
                    if correctors_edit2.len()==0 {
                        println!("{}, -",word );
                    }
                    else{
                        // 2 edit distance
                        let mut corrector = word.to_owned();
                        let mut max:usize=0;
                        for i in 0..correctors_edit2.len(){
                            // println!("{}", correctors[i]);
                            let num:usize = *corpus.get(&correctors_edit2[i]).unwrap();
                            if max < num{
                                max = num;
                                corrector=correctors_edit2[i].to_owned();
                            }
                        }
                        println!("{}, {}",word, corrector );
                    }

                }
                // println!("{}", correctors.len());
                else{
                    // 1 edit distance
                    let mut corrector = word.to_owned();
                    let mut max:usize=0;
                    for i in 0..correctors_edit1.len(){
                        // println!("{}", correctors[i]);
                        let num:usize = *corpus.get(&correctors_edit1[i]).unwrap();
                        if max < num{
                            max = num;
                            corrector=correctors_edit1[i].to_owned();
                        }
                    }
                    println!("{}, {}",word, corrector );
                }


            }
        }
    }
}

fn edit0(word: String, corpus: &HashMap<String, usize>) -> bool{
    if corpus.contains_key(&word){
        return true;
    }
    else {
        return false;
    }
}

fn edit1(word: &mut String, corpus: &HashMap<String, usize>) -> Vec<String>{
    let alphabet = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    // let mut new_word = word.to_owned();
    // let mut max:usize=0;
    let mut correctors=vec![];
    // delete a character
    for i in 0..word.len(){
        let temp = String::from(&word[0..i])+&String::from(&word[i+1..word.len()]);

        if edit0(temp.to_owned(), corpus){

             correctors.push(temp.to_owned());

        }
        // println!("{}", new_word);

    }

    //insert a character
    for i in 0..word.len()+1{
        for j in 0..26{
            word.insert(i,alphabet[j]);

            // word.remove(i);
            if edit0(word.to_owned(),corpus){
                correctors.push(word.to_owned());
            }
            word.remove(i);
        }
    }

    // replace a character
    for i in 0..word.len(){
        for j in 0..26{
            let temp=String::from(&word[0..i])+alphabet[j].to_string().as_str()+&String::from(&word[i+1..word.len()]);
            if word.to_owned()==temp.to_owned(){
                continue;
            }
            if edit0(temp.to_owned(), corpus) {
                correctors.push(temp.to_owned());
            }
        }
    }

    // tranpose two neighboring character
    if word.len()>1 {
        for i in 0..word.len()-1{
            let temp=String::from(&word[0..i])+&(String::from(&word[i+1..i+2])+&(String::from(&word[i..i+1])+&String::from(&word[i+1..word.len()-1])));
            // println!("tranpose {}", temp);
            if edit0(temp.to_owned(), corpus){
                correctors.push(temp.to_owned());

            }
        }
    }


    correctors
}

fn edit2(word: &mut String, corpus: &HashMap<String, usize>) -> Vec<String>{
    // let correctors:Vec<String>=vec![];
    let mut correctors_edit2:Vec<String>=vec![];
    let alphabet = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    // let mut new_word = word.to_owned();
    // let mut max:usize=0;
    let mut correctors=vec![];
    // delete a character
    for i in 0..word.len(){
        let temp = String::from(&word[0..i])+&String::from(&word[i+1..word.len()]);
        correctors.push(temp.to_owned());

    }
    for i in 0..word.len()+1{
        for j in 0..26{
            word.insert(i,alphabet[j]);
            correctors.push(word.to_owned());
            word.remove(i);
        }
    }


    for i in 0..word.len(){
        for j in 0..26{
            // println!("{}", String::from(&word[0..i])+alphabet[j].to_string().as_str()+&String::from(&word[i+1..word.len()]));

            let temp=String::from(&word[0..i])+alphabet[j].to_string().as_str()+&String::from(&word[i+1..word.len()]);
            if word.to_owned()==temp.to_owned(){
                continue;
            }

            correctors.push(temp.to_owned());
        }
    }

    for i in 0..word.len()-1{
        let temp=String::from(&word[0..i])+&(String::from(&word[i+1..i+2])+&(String::from(&word[i..i+1])+&String::from(&word[i+1..word.len()-1])));
        // println!("tranpose {}", temp);
        correctors.push(temp.to_owned());
    }
    // println!("edit1 of edit2 {}", correctors.len());
    for i in 0..correctors.len(){

        // println!("{}", correctors[i]);

        let temp=edit1(&mut correctors[i],corpus);
        // println!("temp:{:?}", temp);
        if temp.len()>0 {
            for j in 0..temp.len(){
                correctors_edit2.push(temp[j].to_owned());

            }
        }

    }
    correctors_edit2
}

#[cfg(test)]
mod edit0_tests{
    use super::edit0;
    use std::collections::HashMap;
    #[test]
    fn test_in_corpus(){
        let mut corpus=HashMap::new();
        corpus.insert(String::from("hello"),2 as usize);
        corpus.insert(String::from("world"),1 as usize);
        assert_eq!(edit0(String::from("hello"),&corpus),true);
        assert_eq!(edit0(String::from("world"),&corpus),true);
    }

    #[test]
    fn test_not_in_corpus(){
        let corpus=HashMap::new();
        assert_eq!(edit0(String::from("hello"),&corpus),false);
        assert_eq!(edit0(String::from("world"),&corpus),false);
    }

}

#[cfg(test)]
mod edit1_tests{
    use super::edit1;
    use std::collections::HashMap;

    #[test]
    fn test_hell(){
        let mut corpus=HashMap::new();
        corpus.insert(String::from("hello"),3 as usize);
        corpus.insert(String::from("world"),2 as usize);
        corpus.insert(String::from("word"),1 as usize);
        let edit1_vec=edit1(&mut String::from("hell"),&corpus);
        assert_eq!(edit1_vec.len(),1);
        assert_eq!(edit1_vec[0],"hello");
    }

    #[test]
    fn test_wordl(){
        let mut corpus=HashMap::new();
        corpus.insert(String::from("hello"),3 as usize);
        corpus.insert(String::from("world"),2 as usize);
        corpus.insert(String::from("word"),1 as usize);
        let edit1_vec=edit1(&mut String::from("wordl"),&corpus);
        assert_eq!(edit1_vec.len(),2);
        assert_eq!(edit1_vec[0],"word");
        assert_eq!(edit1_vec[1],"world");
    }

    #[test]
    fn test_hello(){
        let mut corpus=HashMap::new();
        corpus.insert(String::from("hello"),3 as usize);
        corpus.insert(String::from("world"),2 as usize);
        corpus.insert(String::from("word"),1 as usize);
        let edit1_vec=edit1(&mut String::from("hello"),&corpus);
        assert_eq!(edit1_vec.len(),0);
    }
}

#[cfg(test)]
mod edit2_tests{
    use super::edit2;
    use std::collections::HashMap;

    #[test]
    fn test_wo(){
        let mut corpus=HashMap::new();
        corpus.insert(String::from("hello"),3 as usize);
        corpus.insert(String::from("world"),2 as usize);
        corpus.insert(String::from("word"),1 as usize);
        let edit2_vec=edit2(&mut String::from("wo"),&corpus);
        assert_eq!(edit2_vec.len(),2);
        assert_eq!(edit2_vec[0],"word");
        assert_eq!(edit2_vec[1],"word");
    }

    #[test]
    fn test_w(){
        let mut corpus=HashMap::new();
        corpus.insert(String::from("hello"),3 as usize);
        corpus.insert(String::from("world"),2 as usize);
        corpus.insert(String::from("word"),1 as usize);
        let edit2_vec=edit2(&mut String::from("w"),&corpus);
        assert_eq!(edit2_vec.len(),0);

    }

}
