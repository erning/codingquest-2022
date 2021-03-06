fn main() {
    //
    let answer = codingquest::puzzle_10::solve("inputs/puzzle_10-input.png", true);
    println!("puzzle_10: {}", answer);
    println!();

    //
    let answer = codingquest::puzzle_09::solve(
        &std::fs::read_to_string("inputs/puzzle_09-input.txt").unwrap(),
    );
    println!("puzzle_09: {}", answer);

    //
    let answer = codingquest::puzzle_08::solve(
        &std::fs::read_to_string("inputs/puzzle_08-input.txt").unwrap(),
    );
    println!("puzzle_08: {}", answer);

    //
    let answer = codingquest::puzzle_07::solve(
        &std::fs::read_to_string("inputs/puzzle_07-input.txt").unwrap(),
    );
    println!("puzzle_07: {:?}", 20000 * 100000 - answer);

    //
    let answer = codingquest::puzzle_06::solve(
        &std::fs::read_to_string("inputs/puzzle_06-input.txt").unwrap(),
    );
    println!("puzzle_06: {:?}", answer);

    //
    println!("puzzle_05: mining, it takes minutes ...");
    let answer = codingquest::puzzle_05::solve(
        &std::fs::read_to_string("inputs/puzzle_05-input.txt").unwrap(),
    );
    println!("puzzle_05: {}", answer);

    //
    let answer = codingquest::puzzle_04::solve(
        &std::fs::read_to_string("inputs/puzzle_04-input.txt").unwrap(),
    );
    println!("puzzle_04: {}", answer);

    //
    let answer = codingquest::puzzle_03::solve(
        &std::fs::read_to_string("inputs/puzzle_03-input.txt").unwrap(),
    );
    println!("puzzle_03: {}", answer);

    //
    let answer = codingquest::puzzle_02::solve(
        &std::fs::read_to_string("inputs/puzzle_02-input.txt").unwrap(),
    );
    println!("puzzle_02: {}", answer);

    //
    let answer = codingquest::puzzle_01::solve(
        &std::fs::read_to_string("inputs/puzzle_01-input.txt").unwrap(),
    );
    println!("puzzle_01: {}", answer);

    //
    //
    //

    println!();
    println!();

    let answer = codingquest::practice_0304::solve(
        &std::fs::read_to_string("inputs/practice_0304-input.txt").unwrap(),
    );
    println!("practice_0304: {}", answer);

    let answer = codingquest::practice_0303::solve(
        &std::fs::read_to_string("inputs/practice_0303-input.txt").unwrap(),
    );
    println!("practice_0303: {}", answer);

    let answer = codingquest::practice_0302::solve(
        &std::fs::read_to_string("inputs/practice_0302-input.txt").unwrap(),
    );
    println!("practice_0302: {}", answer);

    let answer = codingquest::practice_0301::solve(
        &std::fs::read_to_string("inputs/practice_0301-input.txt").unwrap(),
    );
    println!("practice_0301: {}", answer);

    let answer = codingquest::practice_0228::solve(
        &std::fs::read_to_string("inputs/practice_0228-input.txt").unwrap(),
    );
    println!("practice_0228: {}", answer);
}
