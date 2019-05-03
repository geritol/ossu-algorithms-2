use std::fs::File;
use std::io::{BufRead, BufReader};

mod job;
use job::Job;

fn main() {
    let mut jobs = get_jobs_from_file("jobs.small.txt");
    jobs.sort();
    jobs.reverse();

    let mut prev_task_length = 0;
    let mut total_completion_time:i64 = 0;
    for job in jobs.iter(){
        let curr_task_length = job.get_length();
        let curr_completion_time = job.calculate_weighted_completion_time(prev_task_length);
        prev_task_length += curr_task_length;
        total_completion_time += curr_completion_time;
    }

    println!("Total weighted completion time {}", total_completion_time);
}

fn get_jobs_from_file(file_name: &str) -> Vec<Job> {
    let file = File::open(file_name).expect("Unable to open");
    let mut jobs: Vec<Job>  = Vec::new();
    for (index, line) in BufReader::new(file).lines().enumerate() {
        if index == 0 {
            // TODO: allocate space for Jobs of count int(line)
        } else {
            let unwraped_line = line.unwrap();
            let split: Vec<&str> = unwraped_line.split(" ").collect();
            let weight = split[0].parse::<i64>().unwrap();
            let length = split[1].parse::<i64>().unwrap();
            jobs.push(Job{weight: weight, length: length});
        }
    }
    jobs
}