const PREPOSITIONS: [&str; 11] = [
    "nad",
    "v",
    "pod",
    "před",
    "za",
    "na",
    "ve",
    "s",
    "se",
    "z",
    "ze"
];

const EXCEPTIONS: [&str; 7] = [
    "se",
    "si",
    "já",
    "ty",
    "on",
    "my",
    "vy",
    // etc
];

fn lex(text: &String) -> Vec<String> {
    let mut result = vec![String::new()];
    for character in text.chars() {
        let last_index = &result.len() - 1; 
        match character {
            ','|';'|'.' => {},
            ' ' => { 
                result[last_index] = String::from(&result[last_index]).to_lowercase();
                result.push(String::new()) 
            },
            _ => { 
                result[last_index].push(character) 
            }
        }
    }
    return result;
}

struct Stream<T> {
    position: usize,
    words: Vec<T>
}

impl<T> Stream<T> {
    pub fn new(words: Vec<T>) -> Stream<T> {
        Stream {
            position: 0,
            words
        }
    }

    fn next(&mut self) {
        self.position += 1;
    }

    fn on_previous(&mut self) -> &T {
        if self.position == 0 {
            return &self.words[self.position]
        }
        &self.words[self.position - 1]
    }

    fn on_next(&mut self) -> &T {
        if self.position + 1 == self.words.len() {
            return &self.words[self.position]
        }
        &self.words[self.position + 1]
    }

    fn curent(&mut self) -> &T {
        &self.words[self.position]
    }

    fn ended(&mut self) -> bool {
        self.position == self.words.len()
    }
}

fn parse(words: Vec<String>) -> String {
    let mut result = String::new();
    let mut stream = Stream::new(words);
    while !stream.ended() {
        if !PREPOSITIONS.contains(&stream.on_previous().as_str()) && !EXCEPTIONS.contains(&stream.curent().as_str()) {
            result.push('\'');
        }
        result.push_str(&parse_word(stream.curent()));
        result.push(' ');
        stream.next();
    }
    return format!("[{}]", result.trim());
}

fn parse_word(word: &String) -> String {
    let mut result = String::new();
    let mut stream = Stream::new(Vec::from_iter(word.chars()));
    while !stream.ended() {
        let part =  match stream.curent() {
            'c' => {
                if stream.on_next() == &'h' {
                    stream.next();
                    String::from("x")
                } else {
                    String::from("c")
                }
            },
            'm' => {
                if stream.on_next() == &'ě' {
                    stream.next();
                    String::from("mňe")
                } else {
                    String::from("m")
                }
            },
            'n' => {
                match stream.on_next() {
                    &'ě' =>  {
                        stream.next();
                        String::from("ňe")
                    },
                    &'i' => {
                        stream.next();
                        String::from("ňi")
                    }
                    _ => String::from("n")
                }
            },
            'b' => {
                if stream.on_next() == &'ě' {
                    stream.next();
                    String::from("bje")
                } else {
                    String::from("b")
                }
            },
            'd' => {
                match stream.on_next() {
                    &'ě' =>  {
                        stream.next();
                        String::from("ďe")
                    },
                    &'i' => {
                        stream.next();
                        String::from("ďi")
                    }
                    _ => String::from("d")
                }
            },
            'y' => String::from("i"),
            'x' => String::from("ks"),
            character => character.to_string()
        };
        result.push_str(&part);
        stream.next();
    }

    return result;
}

fn main() {
    println!("{}", parse(lex(&"Já jsem nikdo ty jsi někdo dítě je xylofoník. Je mě zle".to_string())));   
}
