use crate::task::Task;
use chrono::Utc;
use std::path::Path;

pub struct Database;

impl Database {
    pub fn list_tasks() {
        let tasks = Self::get_tasks();
        match tasks {
            Some(t) => {
                let width = 16;
                println!(
                    "{:^4} | {:<width$} | {:<width$} | {:<60}",
                    "ID", "Название", "Начата", "Завершена"
                );
                println!(
                    "{:-<4} | {:-<width$} | {:-<width$} | {:-<60}",
                    "", "", "", ""
                );
                t.iter().for_each(|task| {
                    println!(
                        "{:>4} | {:<width$} | {:<width$} | {:<50}",
                        task.id,
                        task.created.format("%d-%m-%Y %H:%M").to_string(),
                        task.ended.format("%d-%m-%Y %H:%M").to_string(),
                        task.name,
                    );
                });
            }
            None => println!("Нет задач."),
        }
    }

    pub fn add_task(name: String) {
        let tasks = Self::get_tasks();
        match tasks {
            Some(mut t) => {
                if t.iter().any(|task| task.name == name) {
                    println!("Задача с именем {name} уже есть.");
                    std::process::exit(0);
                } else {
                    let task = Task::new(t.last().unwrap().id + 1, &name);
                    println!("Задача {} создана.", task.name);
                    t.push(task);
                    Self::save_tasks(&t);
                }
            }
            None => {
                let task = Task::new(0, &name);
                let tasks = vec![task];
                Self::save_tasks(&tasks);
            }
        }
    }

    pub fn end_task(id: u32) {
        let mut tasks = Self::get_tasks();
        match tasks {
            Some(ref mut t) => {
                if let Some(task) = t.iter_mut().find(|task| task.id == id) {
                    task.ended = Utc::now();
                    println!("Задача с id {} завершена.", id);
                    Self::save_tasks(&t);
                } else {
                    println!("{} - задачи не существует.", id);
                }
            }
            None => {
                println!("Задач нет.");
            }
        }
    }

    pub fn get_tasks() -> Option<Vec<Task>> {
        let filepath = Self::get_filepath();
        let tasks_path = Path::new(filepath.as_str());
        if tasks_path.exists() {
            let tasks_json = std::fs::read_to_string(tasks_path).unwrap();
            let tasks: Vec<Task> = serde_json::from_str(&tasks_json).unwrap();
            Some(tasks)
        } else {
            None
        }
    }

    fn get_filepath() -> String {
        let mut filepath = std::env::var("HOME").unwrap();
        filepath.push_str("/tasks.json");
        filepath
    }

    pub fn save_tasks(tasks: &Vec<Task>) {
        println!("Сохраняю задачи.");
        let filepath = Self::get_filepath();
        let tasks_path = Path::new(filepath.as_str());
        println!("Десереализация Vec<Task>.");
        let tasks_json = serde_json::to_string_pretty(&tasks).unwrap();
        std::fs::write(tasks_path, tasks_json).unwrap();
    }
}
