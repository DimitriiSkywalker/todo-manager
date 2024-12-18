use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;
use std::path::Path;




fn main() {
    #[derive(Serialize, Deserialize, Debug)]
    struct Task {
        id: u32,
        description: String,
        completed: bool,
    }

    impl Task {
        fn new(id: u32, description: String) -> Self {
            Self {
                id,
                description,
                completed: false,
            }
        }
    }

    #[derive(Parser)]
    #[command(author, version, about, long_about = None)]
    struct Cli {
        #[command(subcommand)]
        command: Commands,
    }

    #[derive(Subcommand)]
    enum Commands {
        Add { description: String },
        List,
        Complete { id: u32 },
        Remove { id: u32 },
    }

    const FILE_PATH: &str= "tasks.json";

    fn load_tasks() -> Vec<Task> {
        if Path::new(FILE_PATH).exists() {
            let data = fs::read_to_string(FILE_PATH).expect("Не удалось прочитать файл");
            serde_json::from_str(&data).expect("ошибка десериализации")
        } else {
            Vec::new()
        }
    }

    fn save_tasks(tasks: &Vec<Task>) {
        let data = serde_json::to_string(tasks).expect("Ошибка сериализации");
        fs::write(FILE_PATH, data).expect("Не удалось записать в файл");
    }

    let cli = Cli::parse();
    let mut tasks = load_tasks();

    match cli.command {
        Commands::Add { description } => {
            let id = tasks.len() as u32 +1;
            tasks.push(Task::new(id, description));
            save_tasks(&tasks);
            println!("Задача добавлена!");
        }
        Commands::List => {
            for task in &tasks {
                println!(
                    "[{}] {} - {}",
                    task.id,
                    task.description,
                    if task.completed {"Выполнено"} else {"Не выполнено"}
                );
            }
        }
        Commands::Complete {id} => {
            if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
                task.completed = true;
                save_tasks(&tasks);
                println!("Задача выполнена!");
            } else {
                println!("Задача с ID {} не найдена", id);
            }
        }
        Commands::Remove { id } => {
            tasks.retain(|task| task.id != id);
            save_tasks(&tasks);
            println!("Задача удалена!");
        }
    }
}
