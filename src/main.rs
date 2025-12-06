mod game;

fn main() {
    let secret_number = game::initialize();
    game::run_main_loop(secret_number);
}
