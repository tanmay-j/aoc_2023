use std::fs;

pub fn run() {
    let input_filepath = "src/data/1.txt";
    println!("First");
    unoptimized_silver(input_filepath)
}


fn unoptimized_silver(filepath: &str){
    println!("In file {}", filepath);

    let contents = fs::read_to_string(filepath)
        .expect("Should have been able to read the file");

    let mut sum: u32 = 0;

    let lines = contents.lines();

    let zero:u32 = '0' as u32;


    for line in lines {
        // println!("Lines: {line}");
        let mut first:u32 = 0;
        let mut last:u32 = 0;
        for c in line.chars() {
            // print!("{c}");
            if c >= '0' && c <= '9'{
                // println!("habibi! {c}");
                first = c as u32;
                first -= zero;
                break;
            }
        }

        for c in line.chars().rev() {
            if c >= '0' && c <= '9'{
                // println!("ibibah! {c}");
                last = c as u32;
                last -= zero;
                break;
            }
        }
        let result:u32 = first*10 + last;
        
        sum+= result;
        // println!("{result}");
    }

    println!("Result: {sum}")

    // println!("With text:\n{contents}");
}