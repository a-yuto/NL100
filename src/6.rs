fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn union (_a: &Vec<String>,mut _b: &Vec<String>) -> Vec<String> {
  let mut _c: Vec<String> = Vec::new();
  for ele in _a {
    if !_c.contains(&ele) {
      _c.push(ele.to_string());
    }
  }
  for ele in _b {
    if !_c.contains(&ele) {
      _c.push(ele.to_string());
    }
  }
  _c
}

fn intersection(_a: &Vec<String>,mut _b: &Vec<String>) -> Vec<String> {
  let mut _c: Vec<String> = Vec::new();
  for ele in _a {
    if _b.contains(&ele) & !_c.contains(&ele){
      _c.push(ele.to_string());
    }
  }
  _c
}

fn n_gram(_s: String,n: i32) -> Vec<String> {
  let mut gram: Vec<String> = Vec::new();
  let mut temp: String = String::new();
  let mut itr: i32 = 0;
  for ch in _s.chars() {
    temp.push(ch);
    itr += 1;
    if itr == n {
      gram.push(temp);
      temp = String::new();
      itr = 0
    }
  }
  gram.push(temp);
  gram
}

fn strvecptr(_a: &Vec<String>) {
  for word in _a {
    println!("{}",word);
  }
}
fn main(){
  let n = 2;
  let sx = String::from("paraparaparadise");
  let sy = String::from("paragraph");
  let _x: Vec<String> = n_gram(sx,n);
  let _y: Vec<String> = n_gram(sy,n);
  let _z: Vec<String> = union(&_x,&_y);
  let _w: Vec<String> = intersection(&_x,&_y);
  println!("{}","union");
  strvecptr(&_z);
  println!("{}","intersection");
  strvecptr(&_w);
}
