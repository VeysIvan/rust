use crate::widgets::Widget;

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    // fn new(title: &str) -> Window {
    // pub(crate) - метод будет доступен при подключении модулей только внутри текущего крейта
    pub(crate) fn new(title: &str) -> Window {
        Window { title: title.to_owned(), widgets: Vec::new() }
    }

    //fn add_widget(&mut self, widget: Box<dyn Widget>) {
    // pub(crate) - метод будет доступен при подключении модулей только внутри текущего крейта
    pub(crate) fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }

    fn inner_width(&self) -> usize {
        std::cmp::max(
            self.title.chars().count(),
            self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
        )
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        // Добавляем 4 пиксела для отступа от границ
        self.inner_width() + 4
    }

    // Original
    //fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
    // Modified
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) -> Result<(), std::fmt::Error> {
        let mut inner = String::new();
        for widget in &self.widgets {
            // Original
            //widget.draw_into(&mut inner);
            // Modified for avoid Warning
            widget.draw_into(&mut inner).expect("");
        }

        let inner_width = self.inner_width();

        /* Original
        // TODO: Измените draw_into так чтобы этот метод возвращал Result<(), std::fmt::Error>.
        // Затем используйте оператор ? вместо .unwrap().
        writeln!(buffer, "+-{:-<inner_width$}-+", "").unwrap();
        writeln!(buffer, "| {:^inner_width$} |", &self.title).unwrap();
        writeln!(buffer, "+={:=<inner_width$}=+", "").unwrap();
        for line in inner.lines() {
            writeln!(buffer, "| {:inner_width$} |", line).unwrap();
        }
        writeln!(buffer, "+-{:-<inner_width$}-+", "").unwrap();
        */

        writeln!(buffer, "+-{:-<inner_width$}-+", "")?;
        writeln!(buffer, "| {:^inner_width$} |", &self.title)?;
        writeln!(buffer, "+={:=<inner_width$}=+", "")?;
        for line in inner.lines() {
            writeln!(buffer, "| {:inner_width$} |", line)?;
        }
        writeln!(buffer, "+-{:-<inner_width$}-+", "")?;

        // Чтобы возвращало Result<(), std::fmt::Error>
        Ok(())
    }
}