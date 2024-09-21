pub enum WindowType {
    FontSizeControl,
    FontFamilyMenu,
    MainMenu,
    FontInfo,
    FontList,
}

pub struct CustomWindowConfig<'a> {
    pub title: &'a str,
    pub height: usize,
    pub width: usize,
    pub content: Option<&'a str>,
    pub keymaps: bool,
    pub window_type: WindowType,
}

impl<'a> CustomWindowConfig<'a> {
    pub fn new(title: &'a str, height: usize, width: usize, window_type: WindowType) -> Self {
        Self {
            title,
            height,
            width,
            content: None,
            keymaps: false,
            window_type,
        }
    }

    pub fn with_keymaps(mut self, keymaps: bool) -> Self {
        self.keymaps = keymaps;
        self
    }

    pub fn with_content(mut self, content: Option<&'a str>) -> Self {
        self.content = content;
        self
    }
}
