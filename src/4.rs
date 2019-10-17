use std::collections::HashMap;


fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}


fn resolve_word(s: &str) -> Vec<String> {
  let mut x: Vec<String> = Vec::new();
  let mut word: String = String::new();
  for ch in s.chars() {
    if ch == ' ' {
      x.push(word);
      word = String::new();
    }else{
      word = format!("{}{}",word,ch.to_string());
    }
  }
  x
}

fn element(s: Vec<String>) -> HashMap<i32,String>{
  let mut map = HashMap::new();
  let mut itr = 1;
  let mut symbol = String::new();
  for word in s {
    if itr == 1 || itr == 5 || itr == 6 || itr == 7 || itr == 8 || itr == 9 || itr == 15 || itr == 16 || itr == 19 {
      symbol = String::from(&word[0..1]);  
    } else {
      symbol = String::from(&word[0..2]);
    }
    map.insert(itr,symbol);
    itr += 1;
  }
  map
}

fn main(){
  let s = String::from("Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.");  
  let ans: Vec<String> = resolve_word(&s);
  for (i,j) in  element(ans){
    println!("{}{}",i,j);
  }
}
