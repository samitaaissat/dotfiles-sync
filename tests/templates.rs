use std::error::Error;

use handlebars::Handlebars;
use serde_json::json;

#[test]
fn alone() -> Result<(), Box<dyn Error>> {
    let mut reg = Handlebars::new();
    // render without register
    println!(
        "{}",
        reg.render_template("Hello {{name}}", &json!({"name": "foo"}))?
    );

    // register template using given name
    reg.register_template_string("tpl_1", "Good afternoon, {{name}}")?;
    assert_eq!(
        reg.render("tpl_1", &json!({"name": "foo"}))?,
        "Good afternoon, foo"
    );
    Ok(())
}

#[test]
fn gitconfig() -> Result<(), Box<dyn Error>> {
    let mut reg = Handlebars::new();
    reg.register_template_string("gitconfig", include_str!("assets/gitconfig/gitconfig.hbs"))?;
    assert_eq!(
        reg.render(
            "gitconfig",
            &serde_json::from_str::<serde_json::Value>(include_str!("assets/gitconfig/gitconfig.json"))?
        )?,
        "[includeif \"gitdir:~/src/\"]\n\tpath = ~/src/.gitconfig\n[includeif \"gitdir:~/test/\"]\n\tpath = ~/test/.gitconfig\n"
    );
    Ok(())
}
