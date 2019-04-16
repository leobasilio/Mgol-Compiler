use std::fs;
use std::io;
use std::io::Read;
use symbols;
use symbols::tokens;
use analyzers::dfa::DFA;

pub struct Lexical<'a> {
    source_code: Vec<char>,
    current_position: usize,
    current_column: usize,
    current_line: usize,
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

        for b in b'\0'..=b'\xFF' {

            let c = &(b as char);

            if *c != '"' {
                dfa.add_transition(8, c, 8);
            }

            if *c != '}' {
                dfa.add_transition(10, c, 10);
            }

        }

        Lexical {
            source_code: vec![],
            current_position: 0,
            current_line: 1,
            current_column: 1,
            automaton: dfa,
            table: table
        }

    }

    pub fn load(&mut self, filename: &str) -> io::Result<()> {

        let mut content = String::new();

        fs::File::open(filename)?.read_to_string(&mut content)?;

        self.source_code = content.chars().collect();
        self.current_position = 0;
        self.current_line = 1;
        self.current_column = 1;

        Ok(())

    }

    pub fn next_token(&mut self) -> symbols::Symbol {

        loop {

            self.automaton.reset();

            let mut count_read = 0;
            let mut count_accepted = 0;
            let mut final_state = 0;

            for &next_char in &self.source_code[self.current_position..] {

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

                return symbols::Table::make_symbol("", tokens::EOF);

            }else if count_accepted == 0 {

                return symbols::Table::make_symbol(&self.extract_lexeme(count_read), tokens::ERROR);

            } else {

                let class = Lexical::find_class(final_state);
                let lexeme = self.extract_lexeme(count_accepted);

                if class != tokens::WHITESPACE && class != tokens::COMMENT {

                    if class == tokens::IDENTIFIER {

                        return self.table.insert(&lexeme, class);

                    }else{

                        return symbols::Table::make_symbol(&lexeme, class);

                    }

                }

            }

        }

    }

    fn extract_lexeme(&mut self, length: usize) -> String {

        let i = self.current_position;
        let j = i + length;
        let lexeme: String = self.source_code[i..j].iter().collect();

        self.current_position += length;

        for c in lexeme.chars() {

            if c == '\n' {

                self.current_line += 1;
                self.current_column = 1;

            }else{

                self.current_column += 1;

            }

        }

        return lexeme;

    }

    fn find_class(state: i8) -> &'static str {

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

    pub fn current_line(&self) -> usize {
        self.current_line
    }

    pub fn current_column(&self) -> usize {
        self.current_column
    }

}