fn main() {
    let my_scores = vec![
        Test { score: 90 },
        Test { score: 78 },
        Test { score: 87 },
        Test { score: 23 },
    ];
    for score in my_scores {
        score.print_score();
        Test::print_score(&score);
    }
}

struct Test {
    score: i32,
}

impl Test {
    fn print_score(&self) {
        println!("Score: {:?}", self.score)
    }
}

