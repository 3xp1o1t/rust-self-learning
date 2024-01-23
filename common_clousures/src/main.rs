fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // filter even numbers
    let even_numbers: Vec<i32> = numbers.into_iter().filter(|&x| x % 2 == 0).collect();

    println!("Even numbers (Filter): {:?}", even_numbers);

    // pow every number
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let squared_numbers: Vec<i32> = numbers.into_iter().map(|x| x * x).collect();

    println!("Pow numbers: {:?}", squared_numbers);

    // Accumulator
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum = numbers.into_iter().fold(0, |acc, x| acc + x);

    println!("1 to 10 sum: {}", sum);

    // Any verify if any value meets condition
    let numbers = vec![1, 2, 3, 4, 5, 6, 8, 9, 10];
    let any_is_seven = numbers.iter().any(|&x| x == 7);
    println!("Is 7 overthere? {}", any_is_seven);

    // All verify if all items meets conditons
    let numbers = vec![1, 2, 3, 4, 5, 6, 8, 9, 10];
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("All positive? {}", all_positive);

    // Enumerate used for index and values
    let numbers = vec![10, 20, 30];

    for (index, value) in numbers.iter().enumerate() {
        println!("Index: {} - Value: {}", index, value);
    }
}
