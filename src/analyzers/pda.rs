use symbols;
use std::collections::HashMap;
use analyzers::error::SyntacticError;
use analyzers::error::SemanticError;
use analyzers::Semantic;
use analyzers::semantic::ReductionHandler;

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
    right_side: String,
    handler: Option<ReductionHandler>
}

#[derive(Clone)]
struct Reduction {
    rule_nr: i8,
    pop_count: i8
}

pub struct PDA {

    stack: Vec<i8>,

    // pilha de tokens
    t_stack: Vec<symbols::SharedSymbol>,

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

    panicking: Vec<String>,

    semantic: Semantic

}

impl<'a> PDA {

    pub fn new(semantic: Semantic) -> Self {

        PDA {
            stack: vec![0],
            t_stack: vec![],
            actions: HashMap::new(),
            gotos: HashMap::new(),
            rules: HashMap::new(),
            follows: HashMap::new(),
            reductions: HashMap::new(),
            panicking: vec![],
            semantic
        }

    }

    pub fn reset(&mut self){
        self.stack = vec![0];
        self.t_stack = vec![];
        self.panicking = vec![];
        self.semantic.reset();
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

    pub fn add_rule(&mut self, rule_nr: i8, left_side: &str, right_side: &str, handler: Option<ReductionHandler>){

        self.rules.insert(rule_nr, Rule {
            left_side: left_side.to_string(),
            right_side: right_side.to_string(),
            handler
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

    pub fn read(&mut self, token: &symbols::SharedSymbol) -> (Result<bool, SyntacticError>, Vec<SemanticError>) {

        let lexeme = PDA::get_lexeme(token);

        if self.panic_discard(&lexeme) {

            return (Ok(false), vec![]);

        }

        let mut semantic_errors: Vec<SemanticError> = vec![];

        loop {

            if let Some(&current_state) = self.stack.last() {

                if let Some(&action) = self.actions.get(&(current_state, lexeme.to_string())) {

                    match self.action_run(current_state, &action, token) {

                        Ok(Some(accepted)) => return (Ok(accepted), semantic_errors),

                        Err(e) => semantic_errors.push(e),

                        _ => ()

                    }

                }else {

                    let (syntactic_error, semantic_error) = self.error(current_state);

                    if let Some(e) = semantic_error {

                        semantic_errors.push(e);

                    }

                    if let Some(e) = syntactic_error {

                        return (Err(e), semantic_errors);

                    }

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

    fn action_run(&mut self, current_state: i8, action: &Action, token: &symbols::SharedSymbol) -> Result<Option<bool>, SemanticError> {

        match action.method {

            ActionMethod::SHIFT => {

                self.action_shift(action.param, token);

                Ok(Some(false))

            },

            ActionMethod::REDUCE => {

                if let Err(e) = self.action_reduce(current_state) {

                    Err(e)

                }else{

                    Ok(None)

                }

            },

            ActionMethod::ACCEPT => {

                Ok(Some(true))

            }

        }

    }

    fn action_shift(&mut self, new_state: i8, token: &symbols::SharedSymbol){

        self.stack.push(new_state);
        self.t_stack.push(token.clone());

    }

    fn action_reduce(&mut self, current_state: i8) -> Result<(), SemanticError> {

        if let Some(reduction) = self.reductions.get(&current_state) {

            if let Some(rule) = self.rules.get(&reduction.rule_nr) {

                let mut tokens: Vec<symbols::SharedSymbol> = vec![];

                for _ in 0..reduction.pop_count {

                    if self.stack.pop().is_some() {

                        tokens.insert(0, self.t_stack.pop().unwrap());

                    }

                }

                if let Some(&current_state) = self.stack.last() {

                    if let Some(&new_state) = self.gotos.get(&(current_state, rule.left_side.to_string())) {

                        self.stack.push(new_state);

                        let handler = rule.handler.unwrap_or(Semantic::handle_null);

                        match handler(&mut self.semantic, &tokens) {

                            Ok(new_token) => {

                                self.t_stack.push(new_token);

                                return Ok(());

                            },

                            Err(e) => {

                                self.t_stack.push(Semantic::null());

                                return Err(e);

                            }

                        }

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

    fn error(&mut self, current_state: i8) -> (Option<SyntacticError>, Option<SemanticError>) {

        let terminals: Vec<String> = self.actions.keys()
                                                 .filter(|(state, _)| *state == current_state)
                                                 .map(|(_, terminal)| terminal.clone())
                                                 .collect();

        if terminals.len() == 1 {

            let terminal = terminals.first().unwrap().to_string();
            let action = self.actions.get(&(current_state, terminal)).unwrap().clone();

            match self.action_run(current_state, &action, &Semantic::null()) {

                Ok(None) => (None, None),

                Ok(Some(_)) => (Some(SyntacticError::new(terminals)), None),

                Err(e) => (None, Some(e))

            }

        }else{

            self.panicking = self.panic_follow(current_state);

            (Some(SyntacticError::new(terminals)), self.action_reduce(current_state).err())

        }

    }

    pub fn semantic(&'a self) -> &'a Semantic {
        return &self.semantic;
    }

    fn get_lexeme(item: &symbols::SharedSymbol) -> String {

        let item_ref = item.borrow();

        match item_ref.token {

            symbols::tokens::EOF => String::from(""),

            symbols::tokens::IDENTIFIER => String::from("id"),

            symbols::tokens::LITERAL => String::from("literal"),

            symbols::tokens::ARITHMETIC => String::from("opm"),

            symbols::tokens::RELATIONAL => String::from("opr"),

            symbols::tokens::NUMBER => String::from("num"),

            symbols::tokens::ATTRIBUTION => String::from("rcb"),

            "inteiro" => String::from("int"),

            _ => item_ref.lexeme.clone()

        }

    }

}