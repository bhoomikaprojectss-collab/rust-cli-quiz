use std::io;

struct Question {
    prompt: String,
    answer: String,
}

impl Question {
    fn new(prompt: &str, answer: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            answer: answer.to_string(),
        }
    }

    fn ask(&self) -> bool {
        println!("{}", self.prompt);
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read input");
        user_input.trim().eq_ignore_ascii_case(&self.answer)
    }
}

fn main() {
    println!(" Welcome to the Rust CLI Quiz! \n");

    let quiz = vec![
        Question::new("What is Rust known for?", "memory safety"),
        Question::new("Ownership in Rust is checked at?", "compile time"),
        Question::new("Rust uses which type of memory management?", "borrowing"),
    ];

    let mut score = 0;

    for (i, question) in quiz.iter().enumerate() {
        println!("Q{}:", i + 1);
        if question.ask() {
            println!(" Correct!\n");
            score += 1;
        } else {
            println!("Wrong! Correct answer: {}\n", question.answer);
        }
    }

    println!("🎉 Your total score: {}/{}", score, quiz.len());
}
