use crate::app::Task;

pub fn content_to_task(content: &Vec<u8>) -> Vec<Vec<Task>> {
    let mut tasks: Vec<Vec<Task>> = vec![Vec::new(), Vec::new(), Vec::new()];

    content.split(|&data| data == b'\n')
        .map(|s| s.to_vec())
        .enumerate()
        .take(3)
        .for_each(|(i, d)| tasks[i] = d.split(|&data| data == b'\t').map(|s| Task { content: s.to_vec().iter().map(|x| *x as char).collect::<Vec<char>>() }).collect());

    tasks
}

pub fn tasks_to_chars(tasks: &Vec<Vec<Task>>) -> Vec<u8> {
    let mut r: Vec<u8> = Vec::new();
    for task in tasks {
        task.iter().for_each(|t| {
            t.content.iter().for_each(|c| r.push(*c as u8));
            r.extend_from_slice("\t".as_bytes());
        });
        r.pop();
        r.extend_from_slice("\n".as_bytes());
    }
    r.pop();
    
    r
}