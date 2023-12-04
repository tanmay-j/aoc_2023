use std::fs;

pub fn run() {
    let input_filepath = "src/data/1.txt";
    println!("First");

    let silver_answer =  unoptimized_silver(input_filepath);
    println!("Silver Solution: {silver_answer}");

    // Gold not working yet
    
    // let gold_answer =  unoptimized_gold(input_filepath);
    // println!("Gold Solution: {gold_answer}"); 
}


fn unoptimized_silver(filepath: &str) -> u32{

    let contents = fs::read_to_string(filepath)
        .expect("Should have been able to read the file");

    let mut sum: u32 = 0;

    let lines = contents.lines();

    let zero:u32 = '0' as u32;


    for line in lines {
        let mut first:u32 = 0;
        let mut last:u32 = 0;
        for c in line.chars() {
            if c >= '0' && c <= '9'{
                first = c as u32;
                first -= zero;
                break;
            }
        }

        for c in line.chars().rev() {
            if c >= '0' && c <= '9'{
                last = c as u32;
                last -= zero;
                break;
            }
        }
        let result:u32 = first*10 + last;
        
        sum+= result;
    }

    return sum;
}

// ##################################################################################
// The code below needs fixing 
// ##################################################################################

// fn advance_state_machine(string: &str, cur: char, state: [usize]) -> usize {
//     if string.chars().nth(state).expect("Index out of bounds!")==cur {
//         return state+1;
//     }
//     else{
//         return 0;
//     }
// }

// fn unoptimized_gold(filepath: &str) -> u32{
//     let strs = ["zero","one","two","three","four","five","six","seven","eight","nine"];
    
//     let contents = fs::read_to_string(filepath)
//     .expect("Should have been able to read the file");
    
//     let mut sum: u32 = 0;
    
//     let lines = contents.lines();
    
//     let zero:u32 = '0' as u32;
    
    
//     for line in lines {
//         let mut first:u32 = 0;
//         let mut last:u32 = 0;

//         let mut states: [usize; 10] = [0; 10] ;
//         for c in line.chars() {
//             if c >= '0' && c <= '9'{
//                 first = c as u32;
//                 first -= zero;
//                 break;
//             }
//             for i in [0..10] {
//                 states[i] = advance_state_machine(strs[i], c, states[i]);
//                 if states[i] == strs[i].len() {
//                     first = i;
//                     break;
//                 }
//             }
//         }

//         for c in line.chars().rev() {
//             if c >= '0' && c <= '9'{
//                 last = c as u32;
//                 last -= zero;
//                 break;
//             }
//         }
//         let result:u32 = first*10 + last;
        
//         sum+= result;
//     }

//     return sum;

// }