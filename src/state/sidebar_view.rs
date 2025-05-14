#[derive(Debug, Default)]
pub struct SidebarView {
    pub visible: bool,
}

impl SidebarView {
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }
}
