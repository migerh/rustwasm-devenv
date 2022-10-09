pub fn sieves(n: usize) {
    ​​let numbers: Vec<_> = (1..(n + 1)).collect::<Vec<_>>();
    let mut marking: Vec<char> = vec!['?'; n];
    let mut open = n - 1;
    let mut first_open = 1;marking[0] = '+';


    while open > 0 {
        ​​while marking[first_open] != '?' {
             ​​first_open = first_open + 1;
        }

        ​​marking[first_open] = '+';
        open = open - 1;
        for i in first_open..n {
            ​​if marking[i] == '?' && numbers[i] % numbers[first_open] == 0 {
                ​​marking[i] = '-';
                open = open - 1;
            }​​
        }
        ​​first_open = first_open + 1;
    }
    
    ​​console_log(&numbers.into_iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "));
    console_log(&marking.into_iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "));
}​​

