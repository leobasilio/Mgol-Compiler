use std::collections::HashMap;

const REJECTION_STATE: i8 = -1;

pub struct DFA {
    final_states: Vec<i8>,
    current_state: i8,
    transitions: HashMap<(i8,char),i8>
}

impl DFA {

    pub fn new(final_states: &[i8]) -> Self {
        DFA {
            final_states: final_states.to_vec(),
            current_state: 0,
            transitions: HashMap::new()
        }
    }

    pub fn add_transition(&mut self, current_state: i8, symbol: &char, next_state: i8){
        self.transitions.insert((current_state, *symbol), next_state);
    }

    pub fn read_symbol(&mut self, symbol: char) -> bool {

        if let Some(next_state) = self.transitions.get(&(self.current_state, symbol)) {

            self.current_state = *next_state;

            return true;

        }else{

            self.current_state = REJECTION_STATE;

            return false;

        }

    }

    pub fn reset(&mut self){
        self.current_state = 0;
    }

    pub fn current_state(&self) -> i8 {
        self.current_state
    }

    pub fn is_accepted(&self) -> bool {
        self.final_states.contains(&self.current_state)
    }

}