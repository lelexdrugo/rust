/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    vec!()
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    // vec!(0;count)
    vec![0;count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    // vec!(1,1,2,3,5)
    let mut vet = Vec::with_capacity(5);
    for i in 0 .. 5{
        match i {
            0 | 1 => vet.push(1),
            _ => vet.push(vet[i-1]+vet[i-2]),
        }
    }
    return vet;
}

pub fn print_fib() -> Vec<u8> {
    let my_vec = fibonacci();
    //Questo muove
    //myVec.into_iter().for_each(|val| println!("{:?}", val));
    //Questo no, perché attiva in realtà iter, essendo &Container
    (&my_vec).into_iter().for_each(|val| println!("{:?}", val));
    my_vec.iter().for_each(|val| println!("{:?}", val));
    //Se non metto la &, muovo gli elementi e dopo non posso ritornare.
    //Il for usa sotto into_iter, quindi tipende dal tipo del contenitore
    for val in &my_vec {
        println!("{:?}", val);
    }
    return my_vec;
}
