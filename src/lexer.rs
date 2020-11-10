#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    ADD,
    SUB,
    INT(String),
    ALPHA(String),
    LBRACKET,
    RBRACKET,
    END,

}

pub struct Scanner<'a> {
    source: std::str::Chars<'a>,
    current: char,
    line: u8
}

impl Scanner<'_> {
    pub fn new(source: &str) -> Scanner {
        let mut source_iter = source.chars();
        let ch = source_iter.next().unwrap();
        Scanner{
            source: source_iter,
            current: ch,
            line: 1
        }
    }

    fn peek(&self) -> char {
        self.current
    }

    fn consume(&mut self) -> char {
        let old = self.current;
        self.current = match self.source.next() {
            Some(c) => c,
            None => '\0'
        };
        old
    }

    fn is_digit(&mut self) -> bool {
        self.current.is_digit(10)
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_whitespace() {
            match self.peek() {
                '\n' => self.line += 1,
                '\0' => break, // EOF
                _ => ()
            }
            self.consume();
        }
    }

    fn get(&mut self) -> Token {
        // Get the string of the next token
        let mut out = String::new() ;
        
        self.skip_whitespace();

        if self.is_digit() {
            while self.is_digit() {
                out.push(self.consume());
            }
            return Token::INT(out)
        } else {
            match self.consume() {
                '+' => Token::ADD,
                '-' => Token::SUB,
                '(' => Token::LBRACKET,
                ')' => Token::RBRACKET,
                '\0' => Token::END,
                _ => panic!("Character not handled.")

            }
        }

    }


}

impl Iterator for Scanner<'_> {
    type Item=Token;

    fn next(&mut self) -> Option<Token> {
        match self.get() {
            Token::END => None,
            t => Some(t)
        }
    }
}


#[test]
fn test_add() {
    let mut scan = Scanner::new("2+2");
    assert_eq!(scan.get(), Token::INT(String::from("2")));
    assert_eq!(scan.get(), Token::ADD);
    assert_eq!(scan.get(), Token::INT(String::from("2")));
}

#[test]
fn test_long_int() {
    let mut scan = Scanner::new("23+27");
    assert_eq!(scan.get(), Token::INT(String::from("23")));
    assert_eq!(scan.get(), Token::ADD);
    assert_eq!(scan.get(), Token::INT(String::from("27")));
}

#[test]
fn test_whitespace() {
    let mut scan = Scanner::new("     2 +  2  ");
    assert_eq!(scan.get(), Token::INT(String::from("2")));
    assert_eq!(scan.get(), Token::ADD);
    assert_eq!(scan.get(), Token::INT(String::from("2")));
}
