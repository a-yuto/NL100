//fn print_typename<T>(_: T) {
//    println!("{}", std::any::type_name::<T>());
//}


fn cir_count(_s: &str) -> Vec<i32>{
  let mut v: Vec<i32> = Vec::new();
  let mut count = 0;
  for ch in _s.chars(){
    if ch  == ' '{
      v.push(count);
      count = 0;
    }else{
      count += 1;
    }
  }
  v  
}
fn main(){
  let s = String::from("Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.");
  let _vv:Vec<i32> = cir_count(&s);
  for i in _vv{
    println!("{}",i);
  }
}
