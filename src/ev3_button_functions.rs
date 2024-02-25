/// Helper macro to create all necessary functions for a button
#[macro_export]
macro_rules! ev3_button_functions {
    ($button_name:ident) => {
        paste! {
            #[doc = "Check if the `" $button_name "` button is pressed."]
            #[doc = "```no_run"]
            #[doc = "use ev3dev_lang_rust::Button;"]
            #[doc = "use std::thread;"]
            #[doc = "use std::time::Duration;"]
            #[doc = ""]
            #[doc = "# fn main() -> ev3dev_lang_rust::Ev3Result<()> {"]
            #[doc = "let button = Button::new()?;"]
            #[doc = ""]
            #[doc = "loop {"]
            #[doc = "    button.process();"]
            #[doc = "    println!(\"Is '" $button_name "' pressed: {}\", button.is_" $button_name "());"]
            #[doc = "    thread::sleep(Duration::from_millis(100));"]
            #[doc = "}"]
            #[doc = "# }"]
            #[doc = "```"]
            pub fn [<is_ $button_name>] (&self) -> bool {
                self.button_handler.borrow().get_button_state(stringify!($button_name))
            }

            #[doc = "Set an event handler, that is called by `process()` if the pressed state of the `" $button_name "` button changes."]
            #[doc = "```no_run"]
            #[doc = "use ev3dev_lang_rust::Button;"]
            #[doc = "use std::thread;"]
            #[doc = "use std::time::Duration;"]
            #[doc = ""]
            #[doc = "# fn main() -> ev3dev_lang_rust::Ev3Result<()> {"]
            #[doc = "let mut button = Button::new()?;"]
            #[doc = ""]
            #[doc = "button.set_" $button_name "_handler(|is_pressed| {"]
            #[doc = "    println!(\"Is '" $button_name "' pressed: {}\", is_pressed);"]
            #[doc = "});"]
            #[doc = ""]
            #[doc = "loop {"]
            #[doc = "    button.process();"]
            #[doc = "    thread::sleep(Duration::from_millis(100));"]
            #[doc = "}"]
            #[doc = "# }"]
            #[doc = "```"]
            pub fn [<set_ $button_name _handler>](&mut self, handler: impl Fn(bool) + 'static) {
                self.button_handler
                    .borrow_mut()
                    .set_button_handler(stringify!($button_name), Some(Box::new(handler)));
            }

            #[doc = "Removes the event handler of the `" $button_name "` button."]
            pub fn [<remove_ $button_name _handler>](&mut self) {
                self.button_handler
                    .borrow_mut()
                    .set_button_handler(stringify!($button_name), None);
            }
        }
    };
}
