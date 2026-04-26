use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Task {
    description: String,
    done: bool,
}

#[derive(Debug)]
struct TaskList {
    tasks: Vec<Task>,
}

struct User {
    name: String,
    shared_list: Rc<RefCell<TaskList>>,
}

impl User {
    fn add_task(&self, description: String) {
        let mut list = self.shared_list.borrow_mut();
        list.tasks.push(Task {
            description,
            done: false,
        });
        println!("{} added a task.", self.name);
    }

    fn mark_task_done(&self, index: usize) {
        let mut list = self.shared_list.borrow_mut();

        if let Some(task) = list.tasks.get_mut(index) {
            task.done = true;
            println!("{} completed task {}", self.name, index);
        } else {
            println!("Invalid task index.");
        }
    }

    fn print_tasks(&self) {
        let list = self.shared_list.borrow();

        println!("\nShared Task List:");
        for (i, task) in list.tasks.iter().enumerate() {
            let status = if task.done { "Done" } else { "Pending" };
            println!("{}: {} [{}]", i, task.description, status);
        }
    }
}

fn main() {
    let shared_tasks = Rc::new(RefCell::new(TaskList { tasks: Vec::new() }));

    let user1 = User {
        name: String::from("Alice"),
        shared_list: Rc::clone(&shared_tasks),
    };

    let user2 = User {
        name: String::from("Bob"),
        shared_list: Rc::clone(&shared_tasks),
    };

    user1.add_task(String::from("Finish Rust homework"));
    user2.add_task(String::from("Review ownership rules"));

    user1.mark_task_done(0);

    user1.print_tasks();
}