
// Problem Statement:
// You are managing a fleet of $N$ servers. You receive a list of requested maintenance windows, where each request is represented by a tuple [start_time, end_time].
// A single maintenance engineer can only work on one server at a time. However, to minimize downtime, you want to identify the maximum number of non-overlapping maintenance requests that a single engineer can fulfill from the provided list.
// Write a function that accepts a list of time intervals and returns the count of the maximum compatible subset of intervals.
// **Input/Output**:

// - **Input**: A list of integer pairs `[[s1, e1], [s2, e2], ...]`. (Time is represented as integers, e.g., 900 for 9:00 AM).
// - **Output**: An integer representing the count of tasks.

// **Constraints**:

// - `1 <= N <= 10^5`
// - `0 <= start_time < end_time <= 2400`
// - If a task ends at time `T`, the engineer can immediately start a new task at time `T`.

// **Example**:

// - *Input*: `[[900, 1030], [1000, 1100], [1030, 1130], [1100, 1200]]`
// - *Logic*:
//   - Task A: 900-1030
//   - Task B: 1000-1100 (Overlaps A)
//   - Task C: 1030-1130 (Compatible with A)
//   - Task D: 1100-1200 (Compatible with B, but B overlaps A. Check A->C... C overlaps D? Yes, 1130 vs 1100.)
//   - *Optimal Set*: Task A (900-1030) + Task D (1100-1200). Count = 2.
//   - *Alternative Optimal*: Task B (1000-1100) + Task D (1100-1200). Count = 2.
// - *Output*: `2`


fn max_non_overlapping_task( mut intervals:Vec<[i32;2]>)->i32{

    if intervals.is_empty(){
        return 0;
    }
    intervals.sort_unstable_by_key(|iv| iv[1]);
    
    let mut count = 1;
    let mut last_end_time= intervals[0][1];

    for task in &intervals[1..]{
       
        if task[0]>= last_end_time{
            count+=1;
            last_end_time= task[1];
        }
    }
    count

}



fn main() {
    let tasks = vec![[900, 1030], [1000, 1100], [1030, 1130], [1100, 1200]];
    let max_tasks = max_non_overlapping_task(tasks);

    println!("Maximum completed task:{}",max_tasks);
}
