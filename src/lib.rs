use mage_core::{
    image::{Image, Point},
    Colour, PresentInput,
};

pub struct Context {
    pub(crate) image: Image,

    current_pos: (u32, u32),
}

pub fn render_ui(present_input: &mut PresentInput, f: impl FnOnce(&mut Context)) {
    let mut ctx = Context {
        image: present_input.new_image(),
        current_pos: (0, 0),
    };

    f(&mut ctx);

    present_input.blit(
        present_input.rect(),
        ctx.image.rect(),
        &ctx.image,
        Colour::Black.into(),
    );
}

impl Context {
    pub fn print(&mut self, text: &str, ink: u32, paper: u32) {
        let x = self.current_pos.0 as i32;
        let y = self.current_pos.1 as i32;

        self.image.draw_string(Point::new(x, y), text, ink, paper);

        self.current_pos.0 = 0;
        self.current_pos.1 += 1;
    }
}
