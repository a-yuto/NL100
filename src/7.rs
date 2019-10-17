fn output(x: String,y: String,z: String ){
  println!("{}時の{}は{}",x,y,z);
}

fn main(){
  let x: String = String::from("12");
  let y: String = String::from("temp");
  let z: String = String::from("22.4");
  output(x,y,z);
}
