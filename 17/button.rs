use crate::widgets::label::Label;
use crate::widgets::Widget;


pub struct Button {
    label: Label,
}

impl Button {
    //fn new(label: &str) -> Button {
    // pub(crate) - метод будет доступен при подключении модулей только внутри текущего крейта
    pub(crate) fn new(label: &str) -> Button {
        Button {
            label: Label::new(label)
        }
    }
}



impl Widget for Button {
    fn width(&self) -> usize {
        self.label.width() + 8 // Добавление отступа от границ
    }

    // Original
    //fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
    // Modified
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error> {
        let width = self.width();
        let mut label = String::new();
        // Original
        //self.label.draw_into(&mut label);
        // Modified for avoid Warning
        self.label.draw_into(&mut label).expect("");

        /* Original
        writeln!(buffer, "+{:-<width$}+", "").unwrap();
        for line in label.lines() {
            writeln!(buffer, "|{:^width$}|", &line).unwrap();
        }
        writeln!(buffer, "+{:-<width$}+", "").unwrap();
        */

        writeln!(buffer, "+{:-<width$}+", "")?;
        for line in label.lines() {
            writeln!(buffer, "|{:^width$}|", &line)?;
        }
        writeln!(buffer, "+{:-<width$}+", "")?;

        Ok(())
    }
}
