
/*
mod command_pattern;
use command_pattern::{add_brackets, insert_char};
*/
mod builder;

/*
mod factory;
use crate::factory::base_source::BaseSource;
use crate::factory::csv_document::CsvDocument;
use crate::factory::csv_source::CsvSource;
use crate::factory::txt_source::TxtSource;
use std::sync::{Arc, Mutex};
*/


use builder::{
    window::Window,
    builders::{
        ui_builder, relaxed_ui_builder, presentation_ui_builder, compact_ui_builder
    }
};
use crate::builder::builders::ui_builder::UiBuilder;

fn main() {
    //run_command_pattern();

    //run_factory_pattern();

    run_builder_pattern();
}
/*
fn run_command_pattern () {
    let mut  my_text_file = command_pattern::text_file::TextFile::new();

    let add_char_cmd1 = insert_char::InsertChar::new(my_text_file.text.clone(),'a');
    let add_char_cmd2 = insert_char::InsertChar::new(my_text_file.text.clone(),'b');


    let add_brackets = add_brackets::AddBrackets::new(my_text_file.text.clone(), add_brackets::BracketsCase::Quotes);

    let add_char_cmd3 = insert_char::InsertChar::new(my_text_file.text.clone(),'c');

    my_text_file.add_command(Box::new(add_char_cmd1));
    my_text_file.add_command(Box::new(add_char_cmd2));

    my_text_file.add_command(Box::new(add_brackets));

    my_text_file.add_command(Box::new(add_char_cmd3));

    for mut cmd in my_text_file.commands.iter_mut() {
        cmd.execute();

        println!("the value is {}", my_text_file.text.lock().unwrap());
    }

    my_text_file.commands[0].rollback();
    println!("the value is {}", my_text_file.text.lock().unwrap());
}
*/

//Factory pattern -----------------------------------------------------------------------------------------------------------------------
/*
fn run_factory_pattern () {
    let source = initialize(false);

    let document = source.initialize();
    let exported = source.export_document();
    let content = document.read_content();
    println!("{}",content.borrow());
    document.write_content(String::from(" Cadena al final"));
    println!("{}",content.borrow());
}

fn initialize(case: bool)->  Box<dyn BaseSource>{
    if case {
        Box::new(TxtSource::new(String::from("source1")))
    }else{
        Box::new(CsvSource::new(',', '.', String::from("source2")))
    }
}
*/

//Builder pattern -----------------------------------------------------------------------------------

fn run_builder_pattern(){
    let mut relaxed_ui_builder = Box::new(relaxed_ui_builder::RelaxedUiBuilder::default());
    Window::configure_relaxed_view(&mut relaxed_ui_builder);
    let relaxed_ui = relaxed_ui_builder.build();

    let relaxed_definition = relaxed_ui.get_ui_definition();
    println!("{:?}", relaxed_definition);

    let mut compact_ui_builder = Box::new(compact_ui_builder::CompactUiBuilder::default());
    Window::configure_compact_view(&mut compact_ui_builder);
    let compact_ui = compact_ui_builder.build();

    let compact_definition = compact_ui.get_ui_definition();
    println!("{:?}", compact_definition);

    let mut presentation_ui_builder = Box::new(presentation_ui_builder::PresentationUiBuilder::default());
    Window::configure_compact_view(&mut presentation_ui_builder);
    let presentation_ui = presentation_ui_builder.build();

    let presentation_definition = presentation_ui.get_ui_definition();
    println!("{:?}", presentation_definition);
}