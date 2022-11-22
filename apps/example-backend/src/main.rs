use typeshare::typeshare;

#[typeshare]
struct Player {
    name: String,
    score: u32,
}

fn main() {
    let player1 = Player {
        name: String::from("Player 1"),
        score: 42,
    };

    println!(
        "Player 1 name is {} and score is {}",
        player1.name, player1.score
    );
}
