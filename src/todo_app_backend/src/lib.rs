use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::BTreeMap;

thread_local! {
    static STATE: RefCell<Todos> = RefCell::new(Todos::default());
}

#[derive(Default)]
struct Todos {
    todos: BTreeMap<u32, Todo>,
}

#[derive(Clone, CandidType, Deserialize, Serialize)]
struct Todo {
    id: u32,
    text: String,
    completed: bool,
}

impl Todos {
    fn add_todo(&mut self, text: String) -> u32 {
        let id = match self.todos.last_key_value() {
            Some(todo) => todo.0 + 1,
            None => 1,
        };
        self.todos.insert(
            id,
            Todo {
                id: id,
                text,
                completed: false,
            },
        );
        id
    }

    fn remove_todo_by_id(&mut self, id: u32) -> Option<Todo> {
        self.todos.remove(&id)
    }

    fn get_todo_by_id(&self, id: u32) -> Option<Todo> {
        self.todos.get(&id).cloned()
    }

    fn get_todos_paginates(&self, offset: u32, limit: u32) -> Vec<Todo> {
        self.todos
            .iter()
            .skip(offset as usize)
            .take(limit as usize)
            .map(|todo| todo.1.clone())
            .collect()
    }

    fn update_todo_by_id(
        &mut self,
        id: u32,
        text: Option<String>,
        completed: Option<bool>,
    ) -> Option<Todo> {
        let todo = self.todos.get_mut(&id);
        match todo {
            Some(todo) => {
                if let Some(text) = text {
                    todo.text = text;
                }
                if let Some(completed) = completed {
                    todo.completed = completed;
                }
                Some(todo.clone())
            }
            None => {
                ic_cdk::println!("Todo not found");
                None
            }
        }
        // let index = self.todos.iter().position(|todo| todo.id == id);
        // match index {
        //     Some(index) => {
        //         if let Some(text) = text {
        //             self.todos[index].text = text;
        //         }
        //         if let Some(completed) = completed {
        //             self.todos[index].completed = completed;
        //         }
        //         Some(self.todos[index].clone())
        //     }
        //     None => {
        //         ic_cdk::println!("Todo not found");
        //         None
        //     }
        // }
    }
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::update]
fn add(text: String) -> Todo {
    let id = STATE.with(|state| state.borrow_mut().add_todo(text.clone()));
    Todo {
        id: id,
        text: text,
        completed: false,
    }
}

#[ic_cdk::update]
fn remove(id: u32) -> Option<Todo> {
    STATE.with(|state| state.borrow_mut().remove_todo_by_id(id))
}

#[ic_cdk::query]
fn get(id: u32) -> Option<Todo> {
    STATE.with(|state| state.borrow().get_todo_by_id(id))
}

#[ic_cdk::query]
fn paginate(offset: u32, limit: u32) -> Vec<Todo> {
    STATE.with(|state| state.borrow().get_todos_paginates(offset, limit))
}

#[ic_cdk::update]
fn update(id: u32, text: Option<String>, completed: Option<bool>) -> Option<Todo> {
    STATE.with(|state| state.borrow_mut().update_todo_by_id(id, text, completed))
}
