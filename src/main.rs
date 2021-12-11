mod field;
use field::Field;

mod gameplay;
use gameplay::Gameplay;

fn main() {
    let mut game = Gameplay::new();
    let mut field = Field::default();
    field.draw();
    game.event_loop(&mut field);
}
