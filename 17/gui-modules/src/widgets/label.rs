use crate::widgets::Widget;

pub struct Label {
    label: String,
}

impl Label {
    //fn new(label: &str) -> Label {
    // pub(crate) - метод будет доступен при подключении модулей только внутри текущего крейта
    pub(crate) fn new(label: &str) -> Label {
        Label {
            label: label.to_owned()
        }
    }
}


impl Widget for Label {
    fn width(&self) -> usize {
        self.label.lines().map(|line| line.chars().count()).max().unwrap_or(0)
    }

    // Original
    //fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
    // Modified
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error> {
        // Original
        //writeln!(buffer, "{}", &self.label).unwrap();
        // Modified
        writeln!(buffer, "{}", self.label)?;
        Ok(())
    }
}
