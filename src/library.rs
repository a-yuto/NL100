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

