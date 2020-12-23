extern crate gtk;

use gtk::prelude::*;

pub fn math_error_dialog(builder: &gtk::Builder) {
  let dialog: gtk::MessageDialog = builder.get_object("math_error").unwrap();
  
  let close: gtk::Button = builder.get_object("math_error_close").unwrap();
  let close_dialog = dialog.clone();
  close.connect_clicked(move |_| {
    close_dialog.close();
  });
  
  dialog.show_all();
}