mod extensions;
use extensions::player; 
use extensions::count; 

fn main() {
    let player: player::Player = player::Player::new(String::from("Alex"), 11);
    let mut count : count::Count = count::Count::new(player.number);
    println!("{}", player.to_string());
    println!("\n");
    count.count_down();
    println!("\n");
    count.count_up();
}
