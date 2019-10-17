fn add_string(x: &str,y: &str) -> String{
  let mut s = String::new(); 
  let num = x.chars().count();
  for _i in 0..num {
    s.push(x.chars().nth(_i).unwrap());
    s.push(y.chars().nth(_i).unwrap());
  }
  s.to_string()
}

fn main(){
  let s1 = String::from("パトカー");
  let s2 = String::from("タクシー");
  println!("{}",add_string(&s1,&s2));
}
