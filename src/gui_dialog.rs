extern crate iui;
use iui::controls::{Label, ProgressBar, ProgressBarValue, VerticalBox};
use iui::prelude::*;

#[derive(Clone)]
pub struct DialogUI {
    pub ui: UI,
    pub progress_bar: ProgressBar,
    pub label1: Label,
    pub label2: Label,
    pub label3: Label,
    pub label4: Label,
    pub label5: Label,
    pub label6: Label,
}

impl DialogUI {
    pub fn new(_ctx: &UI, progress_bar: ProgressBar, label1: Label, label2: Label, label3: Label, label4: Label, label5: Label, label6: Label) -> Self {
        DialogUI {
            ui: _ctx.clone(),
            progress_bar: progress_bar.clone(),
            label1: label1.clone(),
            label2: label2.clone(),
            label3: label3.clone(),
            label4: label4.clone(),
            label5: label5.clone(),
            label6: label6.clone(),
        }
    }

    pub fn set_value<V>(&mut self, value: V)
    where
        V: Into<ProgressBarValue>,
    {
        self.progress_bar.set_value(&self.ui, value);
    }

    pub fn set_text(&mut self, text: &mut std::str::Split<&str>) {
        self.label1.set_text(&self.ui, text.next().unwrap());
        self.label2.set_text(&self.ui, text.next().unwrap());
        self.label3.set_text(&self.ui, text.next().unwrap());
        self.label4.set_text(&self.ui, text.next().unwrap());
        self.label5.set_text(&self.ui, text.next().unwrap());
        self.label6.set_text(&self.ui, text.next().unwrap());
    }

    pub fn set_text_label1(&mut self, text: &str) {
        self.label1.set_text(&self.ui, text);
    }
}

unsafe impl Send for DialogUI {}

pub fn build_ui() -> DialogUI {
    let ui = UI::init().expect("Couldn't initialize UI library");
    let mut win = Window::new(&ui, "cpb", 200, 160, WindowType::NoMenubar);

    let mut vbox = VerticalBox::new(&ui);
    vbox.set_padded(&ui, true);

    let progress_bar = ProgressBar::indeterminate(&ui);
    vbox.append(&ui, progress_bar.clone(), LayoutStrategy::Compact);

    let label1 = Label::new(&ui, "");
    let label2 = Label::new(&ui, "");
    let label3 = Label::new(&ui, "");
    let label4 = Label::new(&ui, "");
    let label5 = Label::new(&ui, "");
    let label6 = Label::new(&ui, "");

    vbox.append(&ui, label1.clone(), LayoutStrategy::Compact);
    vbox.append(&ui, label2.clone(), LayoutStrategy::Compact);
    vbox.append(&ui, label3.clone(), LayoutStrategy::Compact);
    vbox.append(&ui, label4.clone(), LayoutStrategy::Compact);
    vbox.append(&ui, label5.clone(), LayoutStrategy::Compact);
    vbox.append(&ui, label6.clone(), LayoutStrategy::Compact);

    win.set_child(&ui, vbox);
    win.show(&ui);

    DialogUI::new(&ui, progress_bar, label1, label2, label3, label4, label5, label6)
}
