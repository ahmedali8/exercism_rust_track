/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    vec![]
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    // 1st way
    // let mut buffer = vec![];
    // for _i in 0..count {
    //     buffer.push(0)
    // }
    // buffer

    // 2nd way
    // (0..count).map(|_| 0).collect()

    // 3rd way
    vec![0; count]
}

pub fn fibonacci_of_length(count: usize) -> Vec<u8> {
    match count {
        0 => vec![],
        1 => vec![1],
        2 => vec![1,1],
        _ => {
            // index0 + index1 = index2
            
            // way 1
            // (2..count).fold(
            //     vec![1,1], 
            //     |mut acc, index| {
            //         acc.push(acc[index - 1] + acc[index - 2]);
            //         acc
            //     }
            // )

            // way 2
            let mut initial_vec: Vec<u8> = vec![1,1];
            for index in 2..count {
                initial_vec.push(
                    initial_vec[index - 1]
                    +
                    initial_vec[index - 2]
                )
            }
            initial_vec
        }
    }
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    fibonacci_of_length(5)
}
