pub mod button;
pub mod label;
pub mod window;

pub trait Widget {
    /// Ширина self.
    fn width(&self) -> usize;

    /// Прорисовка виджета в буфер.
    // Original
    //fn draw_into(&self, buffer: &mut dyn std::fmt::Write);
    // Modified
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error>;

    /// Прорисовка виджета.
    fn draw(&self) {
        let mut buffer = String::new();
        // Original
        //self.draw_into(&mut buffer);
        // Modified for avoid Warning
        self.draw_into(&mut buffer).expect("");
        println!("{buffer}");
    }
}