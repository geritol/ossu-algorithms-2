# Job Optimiser

The input file describes a set of jobs with positive and integral weights and lengths. It has the format:

```
[number_of_jobs]

[job_1_weight] [job_1_length]

[job_2_weight] [job_2_length]

```

For example, the third line of the file is "74 59", indicating that the second job has weight 74 and length 59.

`main.rs` 'schedules' jobs in decreasing order of the difference (weight - length) and outputs the sum of weighted completion times of the resulting schedule.
