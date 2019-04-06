use std::fs;
use std::io;
use std::io::Read;
use symbols;
use symbols::tokens;
use analyzers::dfa::*;

pub struct Lexical<'a> {
    source_code: String,
    current_position: usize,
    automaton: DFA,
    table: &'a mut symbols::Table
}

impl<'a> Lexical<'a> {

    pub fn new(table : &'a mut symbols::Table) -> Self {

        let mut dfa = DFA::new(&[1, 3, 6, 9, 7, 12, 18, 17, 13, 15, 16, 14, 21, 20, 19, 11]);

        dfa.add_transition(1, &'.', 2);
        dfa.add_transition(0, &'"', 8);
        dfa.add_transition(0, &'{', 10);
        dfa.add_transition(0, &'<', 13);
        dfa.add_transition(0, &'>', 14);
        dfa.add_transition(0, &'=', 16);
        dfa.add_transition(0, &'(', 19);
        dfa.add_transition(0, &')', 20);
        dfa.add_transition(0, &';', 21);
        dfa.add_transition(7, &'_', 7);
        dfa.add_transition(8, &'"', 9);
        dfa.add_transition(10, &'}', 11);
        dfa.add_transition(13, &'=', 16);
        dfa.add_transition(13, &'-', 15);
        dfa.add_transition(13, &'>', 17);
        dfa.add_transition(14, &'=', 16);

        for c in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].iter() {
            dfa.add_transition(0, c, 1);
            dfa.add_transition(1, c, 1);
            dfa.add_transition(2, c, 3);
            dfa.add_transition(3, c, 3);
            dfa.add_transition(4, c, 6);
            dfa.add_transition(5, c, 6);
            dfa.add_transition(6, c, 6);
            dfa.add_transition(7, c, 7);
        }

        for c in ['e', 'E'].iter() {
            dfa.add_transition(1, c, 4);
            dfa.add_transition(3, c, 4);
        }

        for c in ['+', '-'].iter() {
            dfa.add_transition(4, c, 5);
        }

        for c in ['+', '-', '*', '/'].iter() {
            dfa.add_transition(0, c, 18);
        }

        for c in ['\t', '\r', '\n', ' '].iter() {
            dfa.add_transition(0, c, 12);
            dfa.add_transition(12, c, 12);
        }

        for b in b'a'..=b'z' {
            let c = &(b as char);
            dfa.add_transition(0, c, 7);
            dfa.add_transition(7, c, 7);
        }

        for b in b'A'..=b'Z' {
            let c = &(b as char);
            dfa.add_transition(0, c, 7);
            dfa.add_transition(7, c, 7);
        }

        for b in 0..=255 {

            if let Some(c) = std::char::from_u32(b) {

                if c != '"' {
                    dfa.add_transition(8, &c, 8);
                }

                if c != '}' {
                    dfa.add_transition(10, &c, 10);
                }

            }

        }

        Lexical {
            source_code: String::new(),
            current_position: 0,
            automaton: dfa,
            table: table
        }

    }

    pub fn load(&mut self, filename: &str) -> io::Result<()> {

        fs::File::open(filename)?.read_to_string(&mut self.source_code)?;

        self.current_position = 0;

        Ok(())

    }

    pub fn next_token(&mut self) -> symbols::Symbol {

        loop {

            self.automaton.reset();

            let mut reader = self.source_code.chars().skip(self.current_position);
            let mut count_read = 0;
            let mut count_accepted = 0;
            let mut final_state = 0;

            while let Some(next_char) = reader.next() {

                count_read += 1;

                if self.automaton.read_symbol(next_char) {

                    if self.automaton.is_accepted() {

                        count_accepted = count_read;

                        final_state = self.automaton.current_state();

                    }

                }else{

                    break;

                }

            }

            if count_read == 0 {

                return symbols::Symbol {
                    lexeme: String::new(),
                    token: String::from(tokens::EOF),
                    data_type: None
                };

            }else if count_accepted == 0 {

                let i = self.current_position;
                let j = i + count_read;

                self.current_position += count_read;

                return symbols::Symbol {
                    lexeme: String::from(&self.source_code[i..j]),
                    token: String::from(tokens::ERROR),
                    data_type: None
                };

            } else {

                let class = self.find_class(final_state);

                let i = self.current_position;
                let j = i + count_accepted;

                self.current_position += count_accepted;

                if class != tokens::WHITESPACE {

                    return self.table.insert(&self.source_code[i..j], class).clone();

                }

            }

        }

    }

    fn find_class(&self, state: i8) -> &'static str {

        match state {
            0 => tokens::EOF,
            1 | 3 | 6 => tokens::NUMBER,
            9 => tokens::LITERAL,
            7 => tokens::IDENTIFIER,
            11 => tokens::COMMENT,
            12 => tokens::WHITESPACE,
            13 | 14 | 16 | 17 => tokens::RELATIONAL,
            15 => tokens::ATTRIBUTION,
            18 => tokens::ARITHMETIC,
            19 => tokens::OPEN_PARENTHESIS,
            20 => tokens::CLOSE_PARENTHESIS,
            21 => tokens::SEMICOLON,
            _ => tokens::ERROR
        }

    }

}