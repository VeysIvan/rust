mod widgets;

use crate::widgets::button::Button;
use crate::widgets::label::Label;
use crate::widgets::Widget;
use crate::widgets::window::Window;

fn main() {
    let mut window = Window::new("Демонстрация графического интерфейса Rust 1.23");
    window.add_widget(Box::new(Label::new("Это маленькая демонстрация графического интерфейса.")));
    window.add_widget(Box::new(Button::new("Нажми меня!")));
    window.draw();
}



//fn main() {
//    println!("Hello, world!");
//}
