    #![allow(dead_code)]
    use std::fs::File;
    use std::io::{ BufRead, BufReader };
    use std::collections::{VecDeque, HashMap};

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub enum Token {
        Empty,
        // Character and operators
        Undefined(Option<char>),
        Comma, 
        LCurly,
        RCurly,
        Equal,
        Equality,
        And,
        AndEq,
        Xor,
        XorEq,
        Or,
        OrEq,
        Mult,
        MultEq,
        Not,
        BooleanNot,
        BitNot,
        BitNotEq,
        Mod,
        ModEq,
        Div,
        DivEq,
        Plus,
        PlusEq, 
        PostPlusEq,
        Minus,
        MinusEq,
        PostMinusEq,
        Gt, 
        Lt,
        Gte,
        Lte,
        NotEq,
        LShift,
        LShiftEq,
        RShift,
        RShiftEq,
        LBracket,
        RBracket,
        LParen,
        RParen,
        BooleanAnd, 
        BooleanOr, 
        Num(u32),
        FieldSelect, 
        FieldDeref,
        TernIf,
        TernElse, 
        QuoteMark,
        DQuoteMark,
        PointerDeref,
        // Types
        Int, 
        Bool, 
        Char,
        String,
        Void,
        Struct,
        // TODO: how to represent structs
        // TODO: How to represent collection of arguments for function types 
        // Keywords
        If, 
        Else,
        While, 
        For, 
        Return, 
        Assert, 
        Error, 
        Alloc, 
        AllocArray,
        Typedef, 
        Break, 
        Continue, 
        SemiColon, 
        Use,
        True, 
        False,
        Null,
    }

    pub fn open_file(path:&mut String) -> BufReader<File>{
        let f = File::open(path).expect( "Unable to open file" );
        let buffered_reader = BufReader::new(f);
        buffered_reader
    }

    pub fn print_lines(path: &mut String){
        let file_reader = open_file(path);

        for line in file_reader.lines(){
        let line = line.expect( "Unable to read line" );
        println!( "Line: {}", line )
        }
    }

    #[derive(Clone)]
    pub struct Lexer {
        tokens: VecDeque<Token>,
        chars: VecDeque<char>
    }

    // TODO: Grammar extensions for annotations
    impl Lexer {
        pub fn new(mut file_path: &mut String) -> Lexer {
            let mut chars = chars(file_path);
            let tokens = VecDeque::new();
            Lexer {
                tokens,
                chars,
            }
        }

        // Unintialised lexer object returned, for testing
        pub fn empty() -> Lexer {
            Lexer{
                tokens: VecDeque::new(),
                chars: VecDeque::new(),
            }
        }

        pub fn tokens(self) -> VecDeque<Token> {
            self.tokens
        }

        pub fn next(&mut self) -> Option<Token> {
            println!("{:?}", self.chars);
            match self.chars.pop_front() {
                Some(c) => {
                    match c {
                        ' '|'\n'|'\t' => self.next(),
                        ';' => Some(Token::SemiColon),
                        '(' => Some(Token::LParen),
                        ')' => Some(Token::RParen),
                        '~' => Some(ops(Token::BitNot,   &mut self.chars)),
                        '=' => Some(ops(Token::Equal,    &mut self.chars)),
                        '!' => Some(ops(Token::Not  ,    &mut self.chars)),
                        '+' => Some(ops(Token::Plus ,    &mut self.chars)),
                        '-' => Some(ops(Token::Minus,    &mut self.chars)),
                        '&' => Some(ops(Token::And  ,    &mut self.chars)),
                        '%' => Some(ops(Token::Mod  ,    &mut self.chars)),
                        '/' => Some(ops(Token::Div  ,    &mut self.chars)),
                        '*' => Some(ops(Token::Mult ,    &mut self.chars)),
                        '<' => Some(ops(Token::Lt   ,    &mut self.chars)),
                        '>' => Some(ops(Token::Gt   ,    &mut self.chars)),
                        '^' => Some(ops(Token::Xor  ,    &mut self.chars)),
                        '|' => Some(ops(Token::Or   ,    &mut self.chars)),
                        '[' => Some(Token::LBracket),
                        ']' => Some(Token::RBracket),
                        '{' => Some(Token::LCurly),
                        '}' => Some(Token::RCurly),
                        ',' => Some(Token::Comma),
                        '.' => Some(Token::FieldSelect),
                        '?' => Some(Token::TernIf),
                        ':' => Some(Token::TernElse),
                        '"' => Some(Token::DQuoteMark),
                        '\''=> Some(Token::QuoteMark),
                        '0'|'1'|'2'|
                        '3'|'4'|'5'|
                        '6'|'7'|'8'|
                        '9' => Some(numeric(c, &mut self.chars)),

                        'i' | 'b' | 'c' | 
                        's' | 'v' | 'a' |
                        'e' | 'r' | 'w' |
                        'f' | '_' | 't' |
                        '#' | 'N' => Some(keyword(c, &mut self.chars)),
                        // ' ' => continue,
                        _   => Some(Token::Undefined(Some(c))),
                    }
                }
                _ => None,
            }
        }
    }

    // TODO: Lexing strings
    // TODO: spotting syntax errors
    fn lex_tokens(chars: &mut VecDeque<char>) -> VecDeque<Token> {
        VecDeque::new()
    }

    fn chars(file_path: &mut String) -> VecDeque<char>{
        let mut chars: VecDeque<char> = VecDeque::new();
        for line in open_file(file_path).lines() {
            match line {
                Ok(value) => for c in value.chars() {
                    chars.push_back(c);
                },
                Err(_) => break,
            }
        }
        println!("{:?}", chars);
        chars
    }

    fn re_insert(t1: char, t2: char, cs: &mut VecDeque<char>, ret: Token) -> Token {
        cs.push_front(t2); 
        cs.push_front(t1);
        ret
    }

    fn re_insert2 (t1: char, cs: &mut VecDeque<char>, ret: Token) -> Token {
        cs.push_front(t1);
        ret
    }

    fn ops(head: Token, chars: &mut VecDeque<char>) -> Token {
        let tail  = chars.pop_front().unwrap_or(' ');
        let tail2 = chars.pop_front().unwrap_or(' ');
        match head {
            Token::Equal => {
                match tail {
                    '=' => Token::Equality,
                    _   => re_insert(tail, tail2, chars, head),
                }
            }
            Token::Lt => {
                match tail {
                    '=' => Token::Lte,
                    '<' => match tail2 { 
                        '=' => Token::LShiftEq,
                        _  => re_insert2(tail2, chars, Token::LShift),
                        },
                    _ => re_insert(tail, tail2, chars, head),
                }
            }
            Token::Gt => {
                match tail {
                    '=' => Token::Gte,
                    '>' => match tail2 {
                        '=' => Token::RShiftEq,
                        _   => re_insert2(tail2, chars, Token::RShift), 
                    }, 
                    _   => re_insert(tail, tail2, chars, head),
                }
            }
            Token::Not => {
                match tail {
                    '=' => Token::NotEq,
                    '!' => Token::BooleanNot,
                    _   => re_insert(tail, tail2, chars, head),
                }
            }
            Token::Plus => {
                match tail {
                    '+' => Token::PostPlusEq, 
                    '=' => Token::PlusEq,
                    _   => re_insert(tail, tail2, chars, head),
                }
            }
            Token::Minus => {
                match tail {
                    '-' => Token::PostMinusEq,
                    '=' => Token::MinusEq,
                    '>' => Token::FieldDeref,
                    _         => re_insert(tail, tail2, chars, head)
                }
            }
            Token::Div => {
                match tail {
                    '=' => Token::DivEq,
                    _   => re_insert(tail, tail2, chars, head),
                }
            }
            Token::Mult => {
                match tail {
                    '=' => Token::MultEq, 
                    _   => re_insert(tail, tail2, chars, head),
                }
            }
            Token::Or => {
                match tail {
                    '=' => Token::OrEq, 
                    '|' => Token::BooleanOr,
                    _   => re_insert(tail, tail2, chars, head),
                }
            }
            Token::Mod => {
                match tail {
                    '=' => Token::ModEq,
                    _   => re_insert(tail, tail2, chars, head)
                }
            }
            Token::Xor => {
                match tail {
                    '=' => Token::XorEq,
                    _   => re_insert(tail, tail2, chars, head)
                }
            }
            Token::And => {
                match tail {
                    '=' => Token::AndEq,
                    '&' => Token::BooleanAnd, 
                    _   => re_insert(tail, tail2, chars, head),
                }
            }
            Token::BitNot => {
                match tail {
                    '=' => Token::BitNotEq,
                    _   => re_insert(tail, tail2, chars, head),
                }
            }
            _ => re_insert(tail, tail2, chars, head)
            // TODO: Finish integer ops
            // TODO: Boolean expressions
            // TODO: Pointers from Token::Mult
        }
    }

    fn numeric(head: char, chars: &mut VecDeque<char>) -> Token {
        let mut sum: u32 = head.to_digit(10).unwrap_or(0);
        loop {
            match chars.pop_front() {
                Some(c) => {
                    match c {
                        '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' =>{
                            if sum <= (4294967295/10){
                                sum = 10 * sum + c.to_digit(10).unwrap_or(0);
                            };
                        },
                        _        => {
                            chars.push_front(c);
                            return Token::Num(sum);
                        }
                    }
                },
                _   =>
                    return Token::Num(sum),
            }
        }
    }

    fn copy_n(n: usize, vec: &mut VecDeque<char>) -> Vec<char>{
        let mut ret: Vec<char> = Vec::new();

        for i in 0..n {
            match vec.get(i) {
                Some(c) => ret.push(*c),
                None    => return ret,
            }
        }

        ret
    }

    fn pattern_check(p: Vec<char>, chars: &mut VecDeque<char>) -> Option<Token> {

        let next_n: Vec<char> = copy_n(p.len(), chars);

        if next_n == p {
            return Some(Token::Undefined(None));
        }

        None
    }

    fn match_keyword(patterns: &HashMap<Token, Vec<char>>, chars: &mut VecDeque<char>) -> Option<Token>{

        for (t, p) in patterns.iter() {
            match pattern_check(p.to_vec(), chars) {
                Some(_) => {
                    chars.drain(0..p.len());
                    return Some(t.clone());
                }
                None    => (),
            }

        }

        return None;

    }

    fn keyword(head: char, chars: &mut VecDeque<char>) -> Token {

        let mut patterns = HashMap::new();
        let check_patterns = |patterns: &HashMap<Token, Vec<char>>, chars: &mut VecDeque<char>| -> Token {
            match match_keyword(patterns, chars) {
                Some(token) => token, 
                _           => Token::Undefined(Some(head)),
            }
        };

        match head { 
            'a' => {
                patterns.insert(Token::Assert, vec!['s', 's', 'e', 'r', 't']);
                patterns.insert(Token::Alloc, vec!['l', 'l', 'o', 'c']);
                check_patterns(&patterns, chars)
            }

            'b' => {
                patterns.insert(Token::Bool, vec!['o', 'o', 'l']);
                patterns.insert(Token::Break, vec!['r', 'e', 'a', 'k']);
                check_patterns(&patterns, chars)
            }

            'c' => {
                patterns.insert(Token::Char, vec!['h', 'a', 'r']);
                patterns.insert(Token::Continue, vec!['o', 'n', 't', 'i', 'n', 'u', 'e']);
                check_patterns(&patterns, chars)
            }

            'e' => {
                patterns.insert(Token::Error, vec!['r', 'r', 'o', 'r']);
                patterns.insert(Token::Else, vec!['l', 's', 'e']);
                check_patterns(&patterns, chars)
            }

            'f' => {
                patterns.insert(Token::For, vec!['o', 'r']);
                patterns.insert(Token::False, vec!['a','l','s','e']);
                check_patterns(&patterns, chars)
            }

            'i' => {
                patterns.insert(Token::If, vec!['f']);
                patterns.insert(Token::Int, vec!['n', 't']);
                check_patterns(&patterns, chars)

            }

            'N' => {
                patterns.insert(Token::Null, vec!['U', 'L', 'L']);
                check_patterns(&patterns, chars)
            }

            's' => {
                patterns.insert(Token::String, vec!['t', 'r', 'i', 'n', 'g']);
                patterns.insert(Token::Struct, vec!['t', 'r', 'u', 'c', 't']);
                check_patterns(&patterns, chars)
            }

            't' => {
                patterns.insert(Token::Typedef, vec!['y', 'p', 'e', 'd', 'e', 'f']);
                patterns.insert(Token::True, vec!['r', 'u', 'e']);
                check_patterns(&patterns, chars)
            }

            'r' => {
                patterns.insert(Token::Return, vec!['e', 't', 'u', 'r', 'n']);
                check_patterns(&patterns, chars) 
            }

            'w' => {
                patterns.insert(Token::While, vec!['h', 'i', 'l', 'e']);
                check_patterns(&patterns, chars)
            }

            'v' => {
                patterns.insert(Token::Void, vec!['o', 'i', 'd']);
                check_patterns(&patterns, chars)
            }

            '_' => {
                patterns.insert(Token::AllocArray, vec!['a', 'r', 'r', 'a', 'y']);
                check_patterns(&patterns, chars)
            }

            '#' => {
                patterns.insert(Token::Use, vec!['u', 's', 'e']);
                check_patterns(&patterns, chars)
            }

            _   => Token::Undefined(Some(head)), 
        }
    }
