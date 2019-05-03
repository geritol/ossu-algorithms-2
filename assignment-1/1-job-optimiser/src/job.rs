use std::cmp::Ordering;

#[derive(Debug)]
pub struct Job{
    pub weight: i64,
    pub length: i64,
}

impl Job {
    pub fn calculate_weight(&self) -> i64{
        self.weight - self.length
    }
    pub fn get_length(&self) -> i64{
        self.length
    }
    pub fn calculate_weighted_completion_time(&self, previous_task_length: i64) -> i64{
        self.weight * (self.length + previous_task_length)
    }
}

impl PartialEq for Job {
    fn eq(&self, other: &Job) -> bool {
        let self_score = self.calculate_weight();
        let other_score = other.calculate_weight();

        self_score == other_score && self.weight == other.weight
    }
}
impl Eq for Job {}

impl Ord for Job {
    fn cmp(&self, other: &Job) -> Ordering {
        let self_score = self.calculate_weight();
        let other_score = other.calculate_weight();

        if self_score < other_score {
            Ordering::Less
        } else if self_score > other_score {
            Ordering::Greater
        // Equal score
        } else if self.weight < other.weight {
            Ordering::Less
        } else if self.weight > other.weight {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Job) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Eq
    #[test]
    fn calculate_weigth_simple_test(){
        let job = Job{weight: 1, length: 3};
        assert_eq!(job.calculate_weight(), -2);
    }

    #[test]
    fn job_equality_smaller_vs_bigger(){
        let job_small = Job{weight: 1, length: 3};
        let job_big = Job{weight: 10, length: 30};
        assert_eq!(job_small == job_big, false);
    }

    #[test]
    fn job_equality_same_weigth(){
        let job_small = Job{weight: 1, length: 3};
        let job_big = Job{weight: 2, length: 4};
        assert_eq!(job_small == job_big, false);
    }

    #[test]
    fn job_equality_same_weigth_and_length(){
        let job_small = Job{weight: 1, length: 3};
        let job_big = Job{weight: 1, length: 3};
        assert_eq!(job_small == job_big, true);
    }

    // Ord 
    #[test]
    fn job_order_greater_true(){
        let job_small = Job{weight: 1, length: 3};
        let job_big = Job{weight: 10, length: 3};
        assert_eq!(job_big > job_small, true);
    }

    #[test]
    fn job_order_greater_false(){
        let job_small = Job{weight: 1, length: 3};
        let job_big = Job{weight: 10, length: 3};
        assert_eq!(job_small > job_big, false);
    }

    #[test]
    fn job_order_smaller_weight_true(){
        let job_small = Job{weight: 1, length: 3};
        let job_big = Job{weight: 2, length: 4};
        assert_eq!(job_small < job_big, true);
    }

    #[test]
    fn job_order_smaller_weight_false(){
        let job_small = Job{weight: 1, length: 3};
        let job_big = Job{weight: 2, length: 4};
        assert_eq!(job_big < job_small, false);
    }

    //Completion time
    #[test]
    fn job_weighted_completion_time(){
        let job = Job{weight: 1, length: 3};
        assert_eq!(job.calculate_weighted_completion_time(0), 3);
    }

    #[test]
    fn job_weighted_completion_time_with_prev_task_length(){
        let job = Job{weight: 1, length: 3};
        assert_eq!(job.calculate_weighted_completion_time(11), 14);
    }

    //Sort
    #[test]
    fn job_sort_test(){
        let job_small = Job{weight: 4, length: 4};
        let job_big = Job{weight: 3, length: 2};

        let mut jobs = [job_big, job_small];
        jobs.sort();
        jobs.reverse();
        
        assert_eq!(jobs[0].weight, 3);
    }
}
