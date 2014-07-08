
fn main() {
    for number in range(1i, 101) {

        let output = 

        if div_fifeteen(number) {
            "FizzBuzz".to_str()
        }

        else if div_five(number) {
            "Fizz".to_str()
        }

        else if div_three(number) {
            "Buzz".to_str()
        }

        else {
            number.to_str()
        };

        println!("{:s}", output);

    }
}

fn div_fifeteen(number: int) -> bool {
    number % 15 == 0
}

fn div_five(number: int) -> bool {
    number % 5 == 0
}

fn div_three(number: int) -> bool {
    number % 3 == 0
}