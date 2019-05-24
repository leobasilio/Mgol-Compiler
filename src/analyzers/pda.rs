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
    reductions: HashMap<i8,(i8,String)>
}

impl PDA {

    pub fn new() -> Self {

        PDA {
            stack: vec![0],
            actions: HashMap::new(),
            gotos: HashMap::new(),
            reductions: HashMap::new()
        }

    }

    pub fn add_action(&mut self, state: i8, token: &str, action_type: ActionType, action_param: i8){

        self.actions.insert((state, String::from(token)), (action_type, action_param));

    }

    pub fn add_goto(&mut self, state: i8, symbol: &str, new_state: i8){

        self.gotos.insert((state, String::from(symbol)), new_state);

    }

    pub fn add_reduction(&mut self, rule_nr: i8, pop_count: i8, symbol: &str){

        self.reductions.insert(rule_nr, (pop_count, String::from(symbol)));

    }

    pub fn read(&mut self, lexeme: &str) -> Result<bool, String>{

        loop {

            if let Some(&current_state) = self.stack.last() {

                if let Some(&(action_type, action_param)) =
                   self.actions.get(&(current_state, lexeme.to_string())) {

                    match action_type {

                        ActionType::SHIFT => {

                            self.shift(action_param);

                            return Ok(false);

                        },

                        ActionType::REDUCE => {

                            if let Err(error) = self.reduce(action_param) {

                                return Err(error);

                            }

                        },

                        ActionType::ACCEPT => return Ok(true)

                    }

                }else{

                    return Err(format!("Ação não encontrada: [{},{}]", current_state, lexeme));

                }

            }

        }

    }

    fn shift(&mut self, new_state: i8){

        self.stack.push(new_state);

    }

    fn reduce(&mut self, rule_nr: i8) -> Result<(), String> {

        let stack = &mut self.stack;

        if let Some((reduction_pop_count, reduction_symbol)) = self.reductions.get(&rule_nr) {

            for _ in 0..*reduction_pop_count {

                stack.pop();

            }

            if let Some(&current_state) = stack.last() {

                if let Some(&new_state) = self.gotos.get(&(current_state, reduction_symbol.to_string())) {

                    stack.push(new_state);

                    return Ok(())

                }else{

                    return Err(format!("Desvio não encontrado [{},{}]", current_state, reduction_symbol));

                }

            }else{

                return Err(format!("Pilha vazia na redução [{},{}]", reduction_pop_count, reduction_symbol));

            }

        }else{

            return Err(format!("Redução não encontrada [{}]", rule_nr));

        }

    }

}