mod tpl_io;

use clickrs::command;
use handlebars::Handlebars;
use std::path::PathBuf;

#[command(
    name = "Dotfiles Sync",
    about = "A utility to easily sync configuration files easily."
)]
#[argument(
    "input",
    help = "Folder where config files are located.",
    default_value = ".",
    parse(from_os_str)
)]
#[argument(
    "output",
    help = "Folder where config files will be placed.",
    default_value = "~",
    parse(from_os_str)
)]
fn main(input: PathBuf, output: PathBuf) {
    let templates = tpl_io::load_tpl::load_templates(input.to_str().unwrap()).unwrap();
    let mut reg = Handlebars::new();

    for template in templates {
        println!("Template: {}", template.name);
        println!("Content: {}", template.content);
        println!("Vars: {}", template.vars);

        reg.register_template_string(&template.name, &template.content)
            .unwrap();
        println!("Render:");
        println!("{}", reg.render(&template.name, &template.vars).unwrap());
    }
}
