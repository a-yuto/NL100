fn rev(s: &str) -> String {
  s.chars().rev().collect()
}
fn main(){
  let s1 = String::from("stressed");
  println!("{}",rev(&s1));
}
