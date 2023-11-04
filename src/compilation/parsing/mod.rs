use std::collections::{HashMap, HashSet};

use super::compilation_error::CompilationError;
use super::parsing::nodes::program_node::ProgramNode;
use super::tokenization::Tokens;

pub mod nodes;

#[derive(Debug)]
pub struct Variable {
    pub stack_position: i64,
}

#[derive(Debug)]
pub struct GlobalString {
    pub string: String,
    pub label: String,
}

#[derive(Debug)]
pub struct Scope {
    variables: HashMap<String, Variable>,
    parent: usize,
    pub stack_size_at_creation: usize,
}

#[derive(Debug)]
pub struct StackPointer {
    stack_size: usize,
    scopes: Vec<Scope>,
    current_scope: usize,
    pub loop_exit_labels: Vec<String>,
}

#[derive(Debug)]
pub struct ParsingContext {
    pub output: String,
    label_counter: usize,
    pub strings: Vec<GlobalString>,
    string_counter: usize,
    stack_pointers: Vec<StackPointer>,
    function_names: HashSet<String>,
}

impl Clone for Variable {
    fn clone(&self) -> Self {
        Self {
            stack_position: self.stack_position,
        }
    }
}

impl ParsingContext {
    pub fn push_on_stack(&mut self, register: &str) {
        self.push_line(format!("    push {}", register).as_str());
        if let Some(p) = self.stack_pointers.last_mut() {
            p.stack_size += 8;
        }
    }

    pub fn pop_from_stack(&mut self, register: &str) {
        self.push_line(format!("    pop {}", register).as_str());
        if let Some(p) = self.stack_pointers.last_mut() {
            p.stack_size -= 8;
        }
    }

    pub fn clear_current_stack(&mut self) {
        if let Some(p) = self.stack_pointers.last_mut() {
            while p.stack_size > 0 {
                self.output.push_str("    pop rsi\n");
                p.stack_size -= 8;
            }
        }
    }

    pub fn push_line(&mut self, string: &str) {
        self.output.push_str(string);
        self.output.push('\n');
    }

    pub fn get_var(&self, name: &String) -> Option<Variable> {
        if let Some(p) = self.stack_pointers.last() {
            let mut current_scope_index = p.current_scope;

            while let Some(scope) = p.scopes.get(current_scope_index) {
                if let Some(var) = scope.variables.get(name) {
                    return Some(var.clone());
                } else {
                    current_scope_index = scope.parent;
                }
            }
        }
        None
    }

    pub fn add_var(&mut self, name: String) -> Option<usize> {
        if let Some(p) = self.stack_pointers.last_mut() {
            if let Some(scope) = p.scopes.get_mut(p.current_scope) {
                println!("added var {} to {}", name, p.stack_size);
                if scope
                    .variables
                    .insert(
                        name,
                        Variable {
                            stack_position: p.stack_size as i64,
                        },
                    )
                    .is_none()
                {
                    return Some(p.stack_size);
                }
            }
        }
        None
    }

    pub fn add_offset_var(&mut self, name: String, offset: i64) {
        if let Some(p) = self.stack_pointers.last_mut() {
            if let Some(scope) = p.scopes.get_mut(p.current_scope) {
                scope.variables.insert(
                    name,
                    Variable {
                        stack_position: offset,
                    },
                );
            }
        }
    }

    pub fn push_scope(&mut self) {
        if let Some(p) = self.stack_pointers.last_mut() {
            p.scopes.push(Scope {
                variables: HashMap::new(),
                parent: p.current_scope,
                stack_size_at_creation: p.stack_size,
            });
            p.current_scope = p.scopes.len() - 1
        }
    }

    pub fn pop_scope(&mut self) {
        let mut popped_scope: Option<Scope> = None;
        let mut stack_size: usize = usize::MAX;
        if let Some(p) = self.stack_pointers.last_mut() {
            if let Some(scope) = p.scopes.pop() {
                p.current_scope = scope.parent;
                stack_size = p.stack_size;
                popped_scope = Some(scope);
            }
        }

        if let Some(scope) = popped_scope {
            while stack_size > scope.stack_size_at_creation {
                self.pop_from_stack("rsi");
                stack_size -= 8;
            }
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

    pub fn add_stack_pointer(&mut self) {
        self.stack_pointers.push(StackPointer {
            stack_size: 0,
            scopes: Vec::new(),
            current_scope: usize::MAX,
            loop_exit_labels: Vec::new(),
        })
    }

    pub fn pop_stack_pointer(&mut self) {
        self.stack_pointers.pop();
    }

    pub fn stack_pointers(&self) -> &Vec<StackPointer> {
        &self.stack_pointers
    }

    pub fn add_loop_exit_label(&mut self, label: String) {
        if let Some(p) = self.stack_pointers.last_mut() {
            p.loop_exit_labels.push(label);
        }
    }

    pub fn pop_loop_exit_label(&mut self) {
        if let Some(p) = self.stack_pointers.last_mut() {
            p.loop_exit_labels.pop();
        }
    }

    pub fn current_loop_exit_label(&mut self) -> Option<String> {
        if let Some(p) = self.stack_pointers.last() {
            p.loop_exit_labels.last().cloned()
        } else {
            None
        }
    }

    pub fn add_function_name(&mut self, name: String) -> bool {
        if self.function_names.contains(&name) {
            return false;
        }
        self.function_names.insert(name);
        true
    }

    pub fn function_exists(&self, name: &String) -> bool {
        self.function_names.contains(name)
    }
}

pub fn parse(tokens: &mut Tokens) -> Result<String, CompilationError> {
    let main_stack = StackPointer {
        stack_size: 0,
        scopes: Vec::new(),
        current_scope: usize::MAX,
        loop_exit_labels: Vec::new(),
    };

    let mut parsing_context: ParsingContext = ParsingContext {
        output: String::new(),
        label_counter: 0,
        strings: Vec::new(),
        string_counter: 0,
        stack_pointers: vec![main_stack],
        function_names: HashSet::new(),
    };

    parsing_context.push_scope();

    match ProgramNode::parse(tokens) {
        Ok(root)=>{
            root.to_asm(&mut parsing_context)?;
            Ok(parsing_context.output)
        }
        Err(mut e) => Err(e.add_line_num(tokens.get_line_num()).clone()),
    }

}
