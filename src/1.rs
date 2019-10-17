fn step_by_2(s: &str) -> String{
  s.chars().step_by(2).collect()
}

fn main(){
  let s1 = String::from("パタトクカシーー");
  println!("{}",step_by_2(&s1));
}
