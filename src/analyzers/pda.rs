use std::collections::HashMap;

#[derive(Clone,Copy)]
pub enum ActionType {
    ACCEPT,
    SHIFT,
    REDUCE
}

pub struct PDA {
    stack: Vec<i8>,
    actions: HashMap<(i8,String),(ActionType,i8)>,
    gotos: HashMap<(i8,String),i8>,
    reductions: HashMap<i8,(String, String,i8)>,
    follows: HashMap<i8,Vec<String>>
}

impl PDA {

    pub fn new() -> Self {

        PDA {
            stack: vec![0],
            actions: HashMap::new(),
            gotos: HashMap::new(),
            reductions: HashMap::new(),
            follows: HashMap::new()
        }

    }

    pub fn add_action(&mut self, state: i8, token: &str, action_type: ActionType, action_param: i8){

        self.actions.insert((state, String::from(token)), (action_type, action_param));

    }

    pub fn add_goto(&mut self, state: i8, symbol: &str, new_state: i8){

        self.gotos.insert((state, String::from(symbol)), new_state);

    }

    pub fn add_reduction(&mut self, rule_nr: i8, left_side: &str, right_side: &str){

        self.reductions.insert(rule_nr, (String::from(left_side),
                                         String::from(right_side),
                                         right_side.split(' ').count() as i8));

    }

    pub fn add_follow(&mut self, state: i8, follow_set: &[&str]){

        self.follows.insert(state, follow_set.iter().map(|s| s.to_string()).collect());

    }

    pub fn read(&mut self, lexeme: &String) -> Result<bool, String>{

        loop {

            if let Some(&current_state) = self.stack.last() {

                if let Some(&(action_type, action_param)) =
                   self.actions.get(&(current_state, lexeme.clone())) {

                    match action_type {

                        ActionType::SHIFT => {

                            self.shift(action_param);

                            return Ok(false);

                        },

                        ActionType::REDUCE => {

                            if let Err(e) = self.reduce(action_param) {

                                return Err(e);

                            }

                        },

                        ActionType::ACCEPT => return Ok(true)

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

    fn reduce(&mut self, rule_nr: i8) -> Result<(), String> {

        if let Some((rule_left_side, _rule_right_side, rule_pop_count)) = self.reductions.get(&rule_nr) {

            for _ in 0..*rule_pop_count {

                self.stack.pop();

            }

            if let Some(&current_state) = self.stack.last() {

                if let Some(&new_state) = self.gotos.get(&(current_state, rule_left_side.to_string())) {

                    self.stack.push(new_state);

                    return Ok(())

                }else{

                    return Err(format!("Desvio não encontrado [{},{}]", current_state, rule_left_side));

                }

            }else{

                return Err(format!("Pilha vazia na redução [{},{}]", rule_pop_count, rule_left_side));

            }

        }else{

            return Err(format!("Redução não encontrada [{}]", rule_nr));

        }

    }

    fn error(&mut self, current_state: i8, lexeme: &String){

        let transitions_count = self.actions.keys().filter(|k| k.0 == current_state).count();

        if transitions_count == 1 {



        }else{

        }

    }

}