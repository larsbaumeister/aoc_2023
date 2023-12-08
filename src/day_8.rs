use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use std::io::BufRead;


pub fn solve() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("input/day_8.txt")?);

    let mut instructions: String = String::new();
    let mut prev_node: String = String::from("AAA");
    
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    for (idx, line) in reader.lines().enumerate() {
        let line = line?;
        
        if line.is_empty() {
            continue;
        } else if idx == 0 {
            instructions = line;
        } else {
            let mut split = line.split(" = ");
            let node_name = split.next().unwrap();
            let node_value = split.last().unwrap();

            let child_nodes = node_value.chars().skip(1).take(8).collect::<String>();
            let mut split = child_nodes.split(", ");
            let left_child = split.next().unwrap();
            let right_child = split.last().unwrap();
            
            nodes.insert(node_name.to_string(), (left_child.to_string(), right_child.to_string()));
        }
    }

    
    let mut num_steps = 0;
    for (idx, instruction) in instructions.chars().cycle().enumerate() {
        if prev_node == "ZZZ" {
            num_steps = idx;
            break;
        }
        prev_node = match instruction {
            'L' => nodes.get(&prev_node).unwrap().0.clone(),
            'R' => nodes.get(&prev_node).unwrap().1.clone(),
            _ => panic!("Invalid instruction: {}", instruction),
        };
    }

    println!("Node: {}", prev_node);
    println!("Number of steps: {}", num_steps);

    Ok(())
}



pub fn solve_v2() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("input/day_8.txt")?);

    let mut instructions: String = String::new();
    
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    for (idx, line) in reader.lines().enumerate() {
        let line = line?;
        
        if line.is_empty() {
            continue;
        } else if idx == 0 {
            instructions = line;
        } else {
            let mut split = line.split(" = ");
            let node_name = split.next().unwrap();
            let node_value = split.last().unwrap();

            let child_nodes = node_value.chars().skip(1).take(8).collect::<String>();
            let mut split = child_nodes.split(", ");
            let left_child = split.next().unwrap();
            let right_child = split.last().unwrap();
            
            nodes.insert(node_name.to_string(), (left_child.to_string(), right_child.to_string()));
        }
    }

    let mut prev_nodes = nodes.iter()
        .filter(|(k, _)| k.ends_with("A"))
        .map(|k| k.0.to_owned())
        .collect::<Vec<_>>();

    let mut steps: Vec<u64> = vec![];
    for i in 0..prev_nodes.len() {
        for (num_steps, instruction) in instructions.chars().cycle().enumerate() {
            if prev_nodes[i].ends_with("Z") {
                steps.push(num_steps as u64);
                break;
            }
            prev_nodes[i] = match instruction {
                'L' => nodes.get(&prev_nodes[i]).unwrap().0.clone(),
                'R' => nodes.get(&prev_nodes[i]).unwrap().1.clone(),
                _ => panic!("Invalid instruction: {}", instruction),
            };
        }
    }

    println!("Number of steps: {}", lcm(steps.as_slice()));

    Ok(())
}


fn lcm(nums: &[u64]) -> u64 {
    let mut result = nums[0];
    for i in 1..nums.len() {
        result = result * nums[i] / gcd(result, nums[i]);
    }
    result
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}