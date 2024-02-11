use std::fs::File;
use std::io::Read;
use walkdir::WalkDir;

/// Template represents a template file with its name, content and vars.
pub struct Template {
    pub name: String,
    pub content: String,
    pub vars: serde_json::Value,
}

/// This function loads all templates from a given directory.
///
/// The function receives a directory path as a string and returns a vector of Template.
/// The directory should contain .hbs and .json files with the same name.
/// The .hbs file contains the template content and the .json file contains the vars that will be used for render.
///
/// # Example
///
/// ```
/// use tpl_io::load_tpl::load_templates;
/// use handlebars::Handlebars;
///
/// fn main() {
///     let templates = tpl_io::load_tpl::load_templates("tests/assets").unwrap();
///     let mut reg = Handlebars::new();
///     for template in templates {
///         println!("Template: {}", template.name);
///         println!("Content: {}", template.content);
///         println!("Vars: {}", template.vars);
///
///         reg.register_template_string(&template.name, &template.content)
///             .unwrap();
///         println!("Render:");
///         println!("{}", reg.render(&template.name, &template.vars).unwrap());
///     }
/// }
pub fn load_templates(directory: &str) -> Result<Vec<Template>, Box<dyn std::error::Error>> {
    let mut templates = Vec::new();

    for entry in WalkDir::new(directory) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("hbs") {
            let mut file = File::open(path)?;
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap()
                .to_string();
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let json_path = path.with_extension("json");
            let mut json_file = File::open(&json_path)?;
            let mut vars_str = String::new();
            json_file.read_to_string(&mut vars_str)?;

            let vars = serde_json::from_str(&vars_str)?;

            templates.push(Template {
                name,
                content: contents,
                vars,
            });
        }
    }

    Ok(templates)
}

#[test]
fn load_templates_test() -> Result<(), Box<dyn std::error::Error>> {
    let templates = load_templates("tests/assets")?;
    let mut reg = handlebars::Handlebars::new();

    for template in templates {
        reg.register_template_string(&template.name, &template.content)?;

        assert_eq!(
            reg.render(&template.name, &template.vars)?,
            "[includeif \"gitdir:~/src/\"]\n\tpath = ~/src/.gitconfig\n[includeif \"gitdir:~/test/\"]\n\tpath = ~/test/.gitconfig\n"
        );
    }
    Ok(())
}
