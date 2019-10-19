use rand::seq::SliceRandom;
use std::iter::FromIterator;
fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn resolve_word(s: &String) -> Vec<String> {
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

fn prankstring(s: &String ) -> String {
  let mut word_char: Vec<char>   = s.chars().collect();
  let mut rng = rand::thread_rng();
  word_char.shuffle(&mut rng);
  let baka:String  = String::from_iter(word_char);
  baka
}

fn typoglycemia (original: &String ) -> String {
  let sep_word: Vec<String> = resolve_word(&original);
  let mut unreadable = String::new();
  for mut word in sep_word {
    let size = word.chars().count();
    if size > 4 {
      let head = word.chars().nth(0).unwrap();
      let last = word.chars().nth(size - 1).unwrap();
      word.remove(size - 1);
      word.remove(0);
      word = prankstring(&word);
      word = head.to_string() + &word + &last.to_string();
    }
    word.push_str(&" ".to_string());
    unreadable.push_str(&word);
  }         
  unreadable
}         
        
fn main(){
  let s: String = String::from("I couldn't believe that I could actually understand what I was reading : the phenomenal power of the human mind .");
  println!("{}",typoglycemia(&s));
}
