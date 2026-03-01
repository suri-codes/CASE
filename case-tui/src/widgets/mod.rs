use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph, Wrap},
};
use shared::ViewModel;

impl From<ViewModel> for TuiViewModel {
    fn from(value: ViewModel) -> Self {
        Self(value)
    }
}

pub struct TuiViewModel(shared::ViewModel);

impl Widget for TuiViewModel {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let view_model = self.0;

        Paragraph::new(Text::from(view_model.text))
            .block(Block::bordered().title_top(Line::from("CASE").centered()))
            .style(Style::new().white().on_black())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}
