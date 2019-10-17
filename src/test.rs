use rand::seq::SliceRandom;
use std::iter::FromIterator;
fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
  let mut s:String = String::from("absbsba");
  let mut ss:Vec<char> = s.chars().collect();
  let mut rng = rand::thread_rng();
  ss.shuffle(&mut rng);
  let sss = String::from_iter(ss);
  println!("{}",sss);
}
