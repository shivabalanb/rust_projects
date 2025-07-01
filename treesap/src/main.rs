use rand::Rng;
use std::io::Write;                                                                                                                                                                  
use std::fs::File;                                                                                                                                                                   
                          
fn main() {
    let t1 = [
        "Assignment 5 : LLM quantization",
        "Project (1 per group)",
        "Discrete Math Lecture Catch-up",
        "Sys for ML Lecture Catch-up",
    ];
    let t2 = ["Set up Calendar", "Work on TreeSap"];
    let habits = ["Coding"];
    let mut all_tasks: Vec<&str> = Vec::new();
    all_tasks.extend(t1);
    all_tasks.extend(t2);
    all_tasks.extend(habits);

    const TOTAL_PICKS: i32 = 4;
    let total_task_len= t1.len() + t2.len() + habits.len();
    let mut schedule: Vec<usize> = Vec::new();
    let mut picks = 0;
    'main: while picks < TOTAL_PICKS {
        let mut rng = rand::thread_rng();
        let n: usize = rng.gen_range(0..total_task_len);
        for &element in &schedule{
            if n == element{
                continue 'main;
            }
        }
        schedule.push(n);
        picks+=1;
    }

    let mut f = File::create("schedule.txt").expect("Unable to create file.");
    for &index in &schedule {
        if let Some(task) = all_tasks.get(index) {
            f.write_all(task.as_bytes()).expect("Unable to write data");
            f.write_all(b"\n").expect("Unable to write newline");  // Add a newline after each task
        }
    }
}
