use std::collections::HashMap;

use crate::compilation_error::CompilationError;
use crate::parsing_nodes::ProgramNode;
use crate::tokenization::Tokens;

pub struct Variable {
    pub stack_position: usize,
}

pub struct GlobalString {
    pub string: String,
    pub label: String,
}

pub struct Scope {
    variables: HashMap<String, Variable>,
    parent: usize,
}

pub struct ParsingContext {
    stack_size: usize,
    scopes: Vec<Scope>,
    pub output: String,
    current_scope: usize,
    label_counter: usize,
    pub loop_exit_labels: Vec<String>,
    pub strings: Vec<GlobalString>,
    string_counter: usize,
}

impl Clone for Variable {
    fn clone(&self) -> Self {
        Self {
            stack_position: self.stack_position,
        }
    }
}

impl Scope {
    fn _get_var_recursive(&self, scopes: &Vec<Scope>, name: &String) -> Option<Variable> {
        if let Some(var) = self.variables.get(name) {
            Some(var.clone())
        } else if let Some(scope) = scopes.get(self.parent) {
            scope._get_var_recursive(scopes, name)
        } else {
            None
        }
    }

    fn _get_var(&self, name: &String) -> Option<Variable> {
        self.variables.get(name).cloned()
    }

    fn _add_var_recursive(
        &mut self,
        scopes: &Vec<Scope>,
        name: &String,
        stack_position: usize,
    ) -> bool {
        if self._get_var_recursive(scopes, name).is_some() {
            false
        } else {
            self.variables
                .insert(name.clone(), Variable { stack_position });
            true
        }
    }
}

impl ParsingContext {
    pub fn push_on_stack(&mut self, register: &str) {
        self.push_line(format!("    push {}", register).as_str());
        self.stack_size += 8;
    }

    pub fn pop_from_stack(&mut self, register: &str) {
        self.push_line(format!("    pop {}", register).as_str());
        self.stack_size -= 8;
    }

    pub fn push_line(&mut self, string: &str) {
        self.output.push_str(string);
        self.output.push('\n');
    }

    pub fn get_var(&mut self, name: &String) -> Option<Variable> {
        let mut current_scope_index = self.current_scope;

        while let Some(scope) = self.scopes.get_mut(current_scope_index) {
            if let Some(var) = scope.variables.get(name) {
                return Some(var.clone());
            } else {
                current_scope_index = scope.parent;
            }
        }
        None
    }

    pub fn add_var(&mut self, name: String) -> Option<usize> {
        // let mut current_scope_index = self.current_scope;
        // while let Some(scope) = self.scopes.get_mut(current_scope_index) {
        //     if scope.variables.get(&name).is_some() {
        //         return false;
        //     } else {
        //         current_scope_index = scope.parent;
        //     }
        // }
        if let Some(scope) = self.scopes.get_mut(self.current_scope) {
            if scope
                .variables
                .insert(
                    name,
                    Variable {
                        stack_position: self.stack_size,
                    },
                )
                .is_some()
            {
                None
            } else {
                Some(self.stack_size)
            }
        } else {
            None
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Scope {
            variables: HashMap::new(),
            parent: self.current_scope,
        });
        self.current_scope = self.scopes.len() - 1
    }

    pub fn pop_scope(&mut self) {
        if let Some(scope) = self.scopes.pop() {
            self.current_scope = scope.parent;
        }
    }

    pub fn new_label(&mut self) -> String {
        self.label_counter += 1;
        format!("LABEL{}", self.label_counter)
    }

    pub fn add_string(&mut self, val: &str) -> String {
        let label = self.new_string_label();
        self.strings.push(GlobalString {
            string: val.to_string(),
            label: label.clone(),
        });
        label
    }

    fn new_string_label(&mut self) -> String {
        self.string_counter += 1;
        format!("STRING{}", self.string_counter)
    }
}

pub fn parse(tokens: &mut Tokens) -> Result<String, CompilationError> {
    let mut parsing_context: ParsingContext = ParsingContext {
        stack_size: 0,
        output: String::new(),
        scopes: Vec::new(),
        current_scope: usize::MAX,
        label_counter: 0,
        loop_exit_labels: Vec::new(),
        strings: Vec::new(),
        string_counter: 0,
    };

    parsing_context.push_scope();

    let root = ProgramNode::parse(tokens)?;

    root.to_asm(&mut parsing_context)?;

    Ok(parsing_context.output)
}
