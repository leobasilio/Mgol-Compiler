use std::collections::HashMap;

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

    // state -> State
    reductions: HashMap<i8, Reduction>

}

impl PDA {

    pub fn new() -> Self {

        PDA {
            stack: vec![0],
            actions: HashMap::new(),
            gotos: HashMap::new(),
            rules: HashMap::new(),
            follows: HashMap::new(),
            reductions: HashMap::new()
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

    pub fn read(&mut self, lexeme: &String) -> Result<bool, String>{

        loop {

            if let Some(&current_state) = self.stack.last() {

                if let Some(&action) = self.actions.get(&(current_state, lexeme.clone())) {

                    match action.method {

                        ActionMethod::SHIFT => {

                            self.shift(action.param);

                            return Ok(false);

                        },

                        ActionMethod::REDUCE => {

                            if let Err(e) = self.reduce(current_state) {

                                return Err(e);

                            }

                        },

                        ActionMethod::ACCEPT => return Ok(true)

                    }

                }else{

                    self.error(current_state, lexeme);

                }

            }

        }

    }

    fn shift(&mut self, new_state: i8){

        self.stack.push(new_state);

    }

    fn reduce(&mut self, current_state: i8) -> Result<(), String> {

        if let Some(reduction) = self.reductions.get(&current_state) {

            if let Some(rule) = self.rules.get(&reduction.rule_nr) {

                for _ in 0..reduction.pop_count {

                    self.stack.pop();

                }

                if let Some(&current_state) = self.stack.last() {

                    if let Some(&new_state) = self.gotos.get(&(current_state, rule.left_side.to_string())) {

                        self.stack.push(new_state);

                        return Ok(())

                    }else{

                        return Err(format!("Desvio não encontrado [{},{}]", current_state, rule.left_side));

                    }

                }else{

                    return Err(format!("Pilha vazia na redução [{},{}]", reduction.pop_count, rule.left_side));

                }

            }else{

                return Err(format!("Regra não encontrada [{}]", reduction.rule_nr));

            }

        }else{

            return Err(format!("Redução não encontrada [{}]", current_state));

        }

    }

    fn error(&mut self, current_state: i8, lexeme: &String){

        let actions : Vec<&String> = self.actions.keys()
                                                 .filter(|(state, _)| *state == current_state)
                                                 .map(|(_, terminal)| terminal)
                                                 .collect();

        if actions.len() == 1 {



        }else{

        }

    }

}