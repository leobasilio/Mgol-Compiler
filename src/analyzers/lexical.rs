use std::fs;
use std::io;
use symbols;
use symbols::tokens;
use analyzers::dfa::*;

pub struct Lexical {
    automaton: DFA
}

impl Lexical {

    pub fn new() -> Self {

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
            let c = b as char;
            dfa.add_transition(0, &c, 7);
            dfa.add_transition(7, &c, 7);
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
            automaton: dfa
        }

    }

}