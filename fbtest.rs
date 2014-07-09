
fn div_by_three(num: int) -> bool {
    num % 3 == 0
}

fn div_by_five(num: int) -> bool {
    num % 5 == 0
}

fn div_by_fifeteen(num: int) -> bool {
    num % 15 == 0
}

#[test]
fn test_div_by_three() {
    if div_by_three(1) {
        fail!("One is not divisible by three.");
    }
}

#[test]
fn test_div_by_three_with_three() {
    if !div_by_three(3) {
        fail!("Three should be three.");
    }
}

#[test] 
fn test_div_by_five() {
    if div_by_five(3) {
        fail!("Three is not divisible by five.");
    }
}

#[test]
fn test_div_by_five_with_five() {
    if !div_by_five(5) {
        fail!("Five should be five.");
    }
}

#[test]
fn test_div_by_fifeteen() {
    if div_by_fifeteen(1) {
        fail!("One is not fifeteen");
    }
}

#[test]
fn test_div_by_fifeteen_with_fifeteen() {
    if !div_by_fifeteen(15) {
        fail!("Fifeteen should be fifeteen");
    }
}