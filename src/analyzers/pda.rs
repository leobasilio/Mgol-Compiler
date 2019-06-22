use std::collections::HashMap;
use analyzers::error::PdaError;

#[derive(Clone,Copy)]
pub enum ActionMethod {
    ACCEPT,
    SHIFT,
    REDUCE
}

#[derive(Clone,Copy)]
struct Action {
    method: ActionMethod,
    param: i8
}

#[derive(Clone)]
struct Rule {
    left_side: String,
    right_side: String
}

#[derive(Clone)]
struct Reduction {
    rule_nr: i8,
    pop_count: i8
}

pub struct PDA {

    stack: Vec<i8>,

    // (state,terminal) -> action
    actions: HashMap<(i8, String), Action>,

    // (state,non_terminal) -> new_state
    gotos: HashMap<(i8, String), i8>,

    // rule_nr -> rule
    rules: HashMap<i8, Rule>,

    // non_terminal -> follow_set
    follows: HashMap<String, Vec<String>>,

    // state -> Reduction
    reductions: HashMap<i8, Reduction>,

    panicking: Vec<String>

}

impl PDA {

    pub fn new() -> Self {

        PDA {
            stack: vec![0],
            actions: HashMap::new(),
            gotos: HashMap::new(),
            rules: HashMap::new(),
            follows: HashMap::new(),
            reductions: HashMap::new(),
            panicking: vec![]
        }

    }

    pub fn add_action(&mut self, state: i8, token: &str, action_method: ActionMethod, action_param: i8){

        self.actions.insert((state, String::from(token)), Action {
            method: action_method,
            param: action_param
        });

    }

    pub fn add_goto(&mut self, state: i8, symbol: &str, new_state: i8){

        self.gotos.insert((state, String::from(symbol)), new_state);

    }

    pub fn add_rule(&mut self, rule_nr: i8, left_side: &str, right_side: &str){

        self.rules.insert(rule_nr, Rule {
            left_side: left_side.to_string(),
            right_side: right_side.to_string()
        });

    }

    pub fn add_follow(&mut self, non_terminal: &str, follow_set: &[&str]){

        self.follows.insert(non_terminal.to_string(), follow_set.iter().map(|s| s.to_string()).collect());

    }

    pub fn add_reduction(&mut self, state: i8, rule_nr: i8, pop_count: i8){

        self.reductions.insert(state, Reduction {
            rule_nr,
            pop_count
        });

    }

    pub fn read(&mut self, lexeme: &str) -> Result<bool, PdaError> {

        if self.panic_discard(lexeme) {

            return Ok(false);

        }

        loop {

            if let Some(&current_state) = self.stack.last() {

                if let Some(&action) = self.actions.get(&(current_state, lexeme.to_string())) {

                    if let Some(accepted) = self.action_run(current_state, &action) {

                        return Ok(accepted);

                    }

                }else if let Some(e) = self.error(current_state) {

                    return Err(e);

                }

            }else{

                panic!("Pilha vazia: sem estado atual");

            }

        }

    }

    fn panic_discard(&mut self, lexeme: &str) -> bool {

        if !self.panicking.is_empty() {

            if !self.panicking.contains(&lexeme.to_string()) {

                return true;

            }

            self.panicking.clear();

        }

        return false;

    }

    fn panic_follow(&self, current_state: i8) -> Vec<String> {

        if let Some(reduction) = self.reductions.get(&current_state) {

            if let Some(rule) = self.rules.get(&reduction.rule_nr) {

                if let Some(follow) = self.follows.get(&rule.left_side){

                    return follow.clone();

                }

            }

        }

        panic!("Panic sem follow");

    }

    fn action_run(&mut self, current_state: i8, action: &Action) -> Option<bool> {

        match action.method {

            ActionMethod::SHIFT => {

                self.action_shift(action.param);

                Some(false)

            },

            ActionMethod::REDUCE => {

                self.action_reduce(current_state);

                None

            },

            ActionMethod::ACCEPT => Some(true)

        }

    }

    fn action_shift(&mut self, new_state: i8){

        self.stack.push(new_state);

    }

    fn action_reduce(&mut self, current_state: i8){

        if let Some(reduction) = self.reductions.get(&current_state) {

            if let Some(rule) = self.rules.get(&reduction.rule_nr) {

                for _ in 0..reduction.pop_count {

                    self.stack.pop();

                }

                if let Some(&current_state) = self.stack.last() {

                    if let Some(&new_state) = self.gotos.get(&(current_state, rule.left_side.to_string())) {

                        self.stack.push(new_state);

                    }else{

                        panic!("Desvio não encontrado [{},{}]", current_state, rule.left_side);

                    }

                }else{

                    panic!("Pilha vazia na redução [{},{}]", reduction.pop_count, rule.left_side);

                }

            }else{

                panic!("Regra não encontrada [{}]", reduction.rule_nr);

            }

        }else{

            panic!("Redução não encontrada [{}]", current_state);

        }

    }

    fn error(&mut self, current_state: i8) -> Option<PdaError> {

        let terminals: Vec<String> = self.actions.keys()
                                                 .filter(|(state, _)| *state == current_state)
                                                 .map(|(_, terminal)| terminal.clone())
                                                 .collect();

        if terminals.len() == 1 {

            let terminal = terminals.first().unwrap();
            let action = self.actions.get(&(current_state, terminal.to_string())).unwrap().clone();

            if self.action_run(current_state, &action) == None {

                return None;

            }

        }else{

            self.panicking = self.panic_follow(current_state);

            self.action_reduce(current_state);

        }

        return Some(PdaError::new(terminals));

    }

}