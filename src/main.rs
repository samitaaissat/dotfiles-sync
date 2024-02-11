mod tpl_io;

use handlebars::Handlebars;

fn main() {
    let templates = tpl_io::load_tpl::load_templates("tests/assets").unwrap();
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
