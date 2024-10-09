use std::sync::{Arc, Mutex};
use factory::*;

mod command_pattern;
mod factory;

use command_pattern::{add_brackets, insert_char};
use crate::factory::base_source::BaseSource;
use crate::factory::csv_document::CsvDocument;
use crate::factory::csv_source::CsvSource;
use crate::factory::txt_source::TxtSource;

fn main() {
    //run_command_pattern();

    run_factory_pattern();
}

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