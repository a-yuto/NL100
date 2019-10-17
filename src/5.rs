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
  x.push(word);
  x
}

fn n_gram_word(_s: Vec<String>,n: i32) -> Vec<Vec<String>> {
  let mut gram: Vec<Vec<String>> = Vec::new();
  let mut temp: Vec<String> = Vec::new();
  let mut itr: i32  = 0;
  for word in _s { 
    temp.push(word);
    itr += 1;
    if itr == n {
      gram.push(temp);
      temp = Vec::new();
      itr = 0;
    }
  }
  gram.push(temp);
  gram
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

fn main(){
  let s:String = String::from("Paper is a thin material produced by pressing together moist fibres of cellulose pulp derived from wood, rags or grasses, and drying them into flexible sheets. It is a versatile material with many uses, including writing, printing, packaging, cleaning, decorating, and a number of industrial and construction processes. Papers are essential in legal or non-legal documentation.

The pulp papermaking process developed in China during the early 2nd century CE, possibly as early as the year 105 CE,[1] by the Han court eunuch Cai Lun, although the earliest archaeological fragments of paper derive from the 2nd century BCE in China.[2] The modern pulp and paper industry is global, with China leading its production and the United States right behind it. ");
  let words: Vec<String>  = resolve_word(&s);
  let ngramsword: Vec<Vec<String>> = n_gram_word(words,3);
  for words in ngramsword {
    for word in words {
      println!("{}",word);
    }
    println!("{}","--------------------------");
  }

  let ngram: Vec<String> = n_gram(s,4);
  for word in ngram {
    println!("{}",word);
  }


}
