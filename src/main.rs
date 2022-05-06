fn main() {
    let answer = codingquest::practice_0228::solve(
        &std::fs::read_to_string("inputs/practice_0228-input.txt").unwrap(),
    );
    println!("practice_0228: {}", answer);

    let answer = codingquest::practice_0301::solve(
        &std::fs::read_to_string("inputs/practice_0301-input.txt").unwrap(),
    );
    println!("practice_0301: {}", answer);

    let answer = codingquest::practice_0302::solve(
        &std::fs::read_to_string("inputs/practice_0302-input.txt").unwrap(),
    );
    println!("practice_0302: {}", answer);

    let answer = codingquest::practice_0303::solve(
        &std::fs::read_to_string("inputs/practice_0303-input.txt").unwrap(),
    );
    println!("practice_0303: {}", answer);

    let answer = codingquest::practice_0304::solve(
        &std::fs::read_to_string("inputs/practice_0304-input.txt").unwrap(),
    );
    println!("practice_0304: {}", answer);

    //
    println!();
    println!("##### day 1 #####");
    println!();
    let answer = codingquest::puzzle_01::solve(
        &std::fs::read_to_string("inputs/puzzle_01-input.txt").unwrap(),
    );
    println!("puzzle_01: {}", answer);

    println!();
    println!("##### day 5 #####");
    println!();
    let answer = codingquest::puzzle_05::solve(
        &std::fs::read_to_string("inputs/puzzle_05-input.txt").unwrap(),
    );
    println!();
    println!("puzzle_05: {}", answer);
}
