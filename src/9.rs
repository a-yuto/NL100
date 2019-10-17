
use rand::seq::SliceRandom;
use std::iter::FromIterator;
fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn resolve_word(s: String) -> Vec<String> {
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

fn typoglycemia (original: &String ) -> String {
  let sep_word: Vec<String> = resolve_word(original);
  let unreadable = String::new();
  for word in sep_word {
    if word.chars().count > 4 {
      let word_char: Vec<char>   = word.chars().collect();
      let mut rng = rand::thread_rng();
      word_char.shuffle(&mut rng);
      word = String::from_iter(word_char);
    }
    format!("{}{}",unreadable,word);
  }
  unreadable
}

fn main(){
  let s: String = String::from("I couldn't believe that I could actually understand what I was reading : the phenomenal power of the human mind .");
  println!("{}",typoglucemia(&s));
}
