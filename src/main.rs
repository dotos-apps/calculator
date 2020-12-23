extern crate gtk;
extern crate meval;

use gtk::prelude::*;

static LAYOUT_GLADE: &str = include_str!("layout.glade");

static INPUT_BINDINGS: [(&'static str, &'static str); 15] = [
    ("num_pad_0", "0"),
    ("num_pad_1", "1"),
    ("num_pad_2", "2"),
    ("num_pad_3", "3"),
    ("num_pad_4", "4"),
    ("num_pad_5", "5"),
    ("num_pad_6", "6"),
    ("num_pad_7", "7"),
    ("num_pad_8", "8"),
    ("num_pad_9", "9"),
    ("num_pad_dot", "."),
    ("num_pad_div", " / "),
    ("num_pad_mult", " * "),
    ("num_pad_sub", " - "),
    ("num_pad_add", " + ")
];

mod errors;

struct Calc {
    builder: gtk::Builder,
}

impl Calc {
    fn new() -> Self {
        Calc {
            builder: gtk::Builder::from_string(LAYOUT_GLADE),
        }
    }

    fn build_ui(&mut self) {
        let window: gtk::Window = self.builder.get_object("calculator")
                                              .unwrap();
        
        let calculation: gtk::Entry = self.builder.get_object("calculation").unwrap();

        for &(id, text) in &INPUT_BINDINGS {
            let calculation = calculation.clone();
            let el: gtk::Button = self.builder.get_object(id).unwrap();
            el.connect_clicked(move |_| {
                let pos = calculation.get_position();
                let buffer = calculation.get_buffer();

                buffer.insert_text(pos as u16, text);
                
                calculation.grab_focus();
                calculation.set_position(pos + text.len() as i32);
            });
        }

        let history: gtk::Label = self.builder.get_object("history").unwrap();
        history.set_lines(3);

        let equals: gtk::Button = self.builder.get_object("equals").unwrap();
        let builder = self.builder.clone();
        equals.connect_clicked(move |_| {
            let expression = calculation.get_text().to_string();
            let result = meval::eval_str(&expression);
            
            match result {
                Ok(result) => {
                    let result = result.to_string();

                    calculation.set_text(&result);
                    calculation.set_position(result.len() as i32);

                    history.set_text(&format!("{}\n{}", history.get_text(), expression));
                    history.set_lines(3);
                }
                Err(_) => errors::math_error_dialog(&builder)
            }
        });

        window.show_all();
    }
}

fn main() {
    gtk::init().expect("Failed to initialize GTK");

    let mut calc = Calc::new();
    calc.build_ui();

    gtk::main();
}