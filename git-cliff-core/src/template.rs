use std::collections::{HashMap, HashSet};
use std::error::Error as ErrorImpl;

use regex::Regex;
use serde::Serialize;
use tera::{Context as TeraContext, Result as TeraResult, Tera, Value, ast};

use crate::config::TextProcessor;
use crate::error::{Error, Result};

/// Wrapper for [`Tera`].
#[derive(Debug)]
pub struct Template {
    /// Template name.
    name: String,
    /// Internal Tera instance.
    tera: Tera,
    /// Template variables.
    #[cfg_attr(not(feature = "github"), allow(dead_code))]
    pub variables: Vec<String>,
}

impl Template {
    /// Constructs a new instance.
    pub fn new(name: &str, mut content: String, trim: bool) -> Result<Self> {
        if trim {
            content = content
                .lines()
                .map(str::trim)
                .collect::<Vec<&str>>()
                .join("\n");
        }
        let mut tera = Tera::default();
        if let Err(e) = tera.add_raw_template(name, &content) {
            return if let Some(error_source) = e.source() {
                Err(Error::TemplateParseError(error_source.to_string()))
            } else {
                Err(Error::TemplateError(e))
            };
        }

        tera.register_filter("upper_first", Self::upper_first_filter);
        tera.register_filter("split_regex", Self::split_regex);
        tera.register_filter("replace_regex", Self::replace_regex);
        tera.register_filter("find_regex", Self::find_regex);

        Ok(Self {
            name: name.to_string(),
            variables: Self::get_template_variables(name, &tera)?,
            tera,
        })
    }

    /// Filter for making the first character of a string uppercase.
    fn upper_first_filter(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
        let mut s = tera::try_get_value!("upper_first_filter", "value", String, value);
        let mut c = s.chars();
        s = match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        };
        Ok(tera::to_value(&s)?)
    }

    /// Replaces all occurrences of a regex pattern with a string.
    fn replace_regex(value: &Value, args: &HashMap<String, Value>) -> TeraResult<Value> {
        let s = tera::try_get_value!("replace_regex", "value", String, value);
        let from = match args.get("from") {
            Some(val) => tera::try_get_value!("replace_regex", "from", String, val),
            None => {
                return Err(tera::Error::msg(
                    "Filter `replace_regex` expected an arg called `from`",
                ));
            }
        };

        let to = match args.get("to") {
            Some(val) => tera::try_get_value!("replace_regex", "to", String, val),
            None => {
                return Err(tera::Error::msg(
                    "Filter `replace_regex` expected an arg called `to`",
                ));
            }
        };

        let re = Regex::new(&from).map_err(|e| {
            tera::Error::msg(format!(
                "Filter `replace_regex` received an invalid regex pattern: {e}"
            ))
        })?;
        Ok(tera::to_value(re.replace_all(&s, &to))?)
    }

    /// Finds all occurrences of a regex pattern in a string.
    fn find_regex(value: &Value, args: &HashMap<String, Value>) -> TeraResult<Value> {
        let s = tera::try_get_value!("find_regex", "value", String, value);

        let pat = match args.get("pat") {
            Some(p) => {
                let p = tera::try_get_value!("find_regex", "pat", String, p);
                p.replace("\\n", "\n").replace("\\t", "\t")
            }
            None => {
                return Err(tera::Error::msg(
                    "Filter `find_regex` expected an arg called `pat`",
                ));
            }
        };
        let re = Regex::new(&pat).map_err(|e| {
            tera::Error::msg(format!(
                "Filter `find_regex` received an invalid regex pattern: {e}"
            ))
        })?;
        let result: Vec<&str> = re.find_iter(&s).map(|mat| mat.as_str()).collect();
        Ok(tera::to_value(result)?)
    }

    /// Splits a string by a regex pattern.
    fn split_regex(value: &Value, args: &HashMap<String, Value>) -> TeraResult<Value> {
        let s = tera::try_get_value!("split_regex", "value", String, value);
        let pat = match args.get("pat") {
            Some(p) => {
                let p = tera::try_get_value!("split_regex", "pat", String, p);
                p.replace("\\n", "\n").replace("\\t", "\t")
            }
            None => {
                return Err(tera::Error::msg(
                    "Filter `split_regex` expected an arg called `pat`",
                ));
            }
        };
        let re = Regex::new(&pat).map_err(|e| {
            tera::Error::msg(format!(
                "Filter `split_regex` received an invalid regex pattern: {e}"
            ))
        })?;
        let result: Vec<&str> = re.split(&s).collect();
        Ok(tera::to_value(result)?)
    }

    /// Recursively finds the identifiers from the AST.
    fn find_identifiers(node: &ast::Node, names: &mut HashSet<String>) {
        match node {
            ast::Node::Block(_, block, _) => {
                for node in &block.body {
                    Self::find_identifiers(node, names);
                }
            }
            ast::Node::VariableBlock(_, expr) => {
                if let ast::ExprVal::Ident(v) = &expr.val {
                    names.insert(v.clone());
                }
            }
            ast::Node::MacroDefinition(_, def, _) => {
                for node in &def.body {
                    Self::find_identifiers(node, names);
                }
            }
            ast::Node::FilterSection(_, section, _) => {
                for node in &section.body {
                    Self::find_identifiers(node, names);
                }
            }
            ast::Node::Forloop(_, forloop, _) => {
                if let ast::ExprVal::Ident(v) = &forloop.container.val {
                    names.insert(v.clone());
                }
                for node in &forloop.body {
                    Self::find_identifiers(node, names);
                }
                for node in &forloop.empty_body.clone().unwrap_or_default() {
                    Self::find_identifiers(node, names);
                }
                for (_, expr) in forloop.container.filters.iter().flat_map(|v| v.args.iter()) {
                    if let ast::ExprVal::String(ref v) = expr.val {
                        names.insert(v.clone());
                    }
                }
            }
            ast::Node::If(cond, _) => {
                for (_, expr, nodes) in &cond.conditions {
                    if let ast::ExprVal::Ident(v) = &expr.val {
                        names.insert(v.clone());
                    }
                    for node in nodes {
                        Self::find_identifiers(node, names);
                    }
                }
                if let Some((_, nodes)) = &cond.otherwise {
                    for node in nodes {
                        Self::find_identifiers(node, names);
                    }
                }
            }
            _ => {}
        }
    }

    /// Returns the variable names that are used in the template.
    fn get_template_variables(name: &str, tera: &Tera) -> Result<Vec<String>> {
        let mut variables = HashSet::new();
        let ast = &tera.get_template(name)?.ast;
        for node in ast {
            Self::find_identifiers(node, &mut variables);
        }
        log::trace!("Template variables for {name}: {variables:?}");
        Ok(variables.into_iter().collect())
    }

    /// Returns `true` if the template contains one of the given variables.
    pub(crate) fn contains_variable(&self, variables: &[&str]) -> bool {
        variables
            .iter()
            .any(|var| self.variables.iter().any(|v| v.starts_with(var)))
    }

    /// Renders the template.
    pub fn render<C: Serialize, T: Serialize, S: Into<String> + Clone>(
        &self,
        context: &C,
        additional_context: Option<&HashMap<S, T>>,
        postprocessors: &[TextProcessor],
    ) -> Result<String> {
        let mut context = TeraContext::from_serialize(context)?;
        if let Some(additional_context) = additional_context {
            for (key, value) in additional_context {
                context.insert(key.clone(), &value);
            }
        }
        match self.tera.render(&self.name, &context) {
            Ok(mut v) => {
                for postprocessor in postprocessors {
                    postprocessor.replace(&mut v, vec![])?;
                }
                Ok(v)
            }
            Err(e) => {
                if let Some(source1) = e.source() {
                    if let Some(source2) = source1.source() {
                        Err(Error::TemplateRenderDetailedError(
                            source1.to_string(),
                            source2.to_string(),
                        ))
                    } else {
                        Err(Error::TemplateRenderError(source1.to_string()))
                    }
                } else {
                    Err(Error::TemplateError(e))
                }
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::commit::Commit;
    use crate::release::Release;

    fn get_fake_release_data() -> Release<'static> {
        Release {
            version: Some(String::from("1.0")),
            message: None,
            extra: None,
            commits: vec![
                Commit::new(String::from("123123"), String::from("feat(xyz): add xyz")),
                Commit::new(String::from("124124"), String::from("fix(abc): fix abc")),
            ]
            .into_iter()
            .filter_map(|c| c.into_conventional().ok())
            .collect(),
            commit_range: None,
            commit_id: None,
            timestamp: None,
            previous: None,
            repository: Some(String::from("/root/repo")),
            submodule_commits: HashMap::new(),
            statistics: None,
            #[cfg(feature = "github")]
            github: crate::remote::RemoteReleaseMetadata {
                contributors: vec![],
            },
            #[cfg(feature = "gitlab")]
            gitlab: crate::remote::RemoteReleaseMetadata {
                contributors: vec![],
            },
            #[cfg(feature = "gitea")]
            gitea: crate::remote::RemoteReleaseMetadata {
                contributors: vec![],
            },
            #[cfg(feature = "bitbucket")]
            bitbucket: crate::remote::RemoteReleaseMetadata {
                contributors: vec![],
            },
            #[cfg(feature = "azure_devops")]
            azure_devops: crate::remote::RemoteReleaseMetadata {
                contributors: vec![],
            },
        }
    }

    #[test]
    fn render_template() -> Result<()> {
        let template = r"
		## {{ version }} - <DATE>
		{% for commit in commits %}
		### {{ commit.group }}
		- {{ commit.message | upper_first }}
		{% endfor %}";
        let mut template = Template::new("test", template.to_string(), false)?;
        let release = get_fake_release_data();
        assert_eq!(
            "\n\t\t## 1.0 - 2023\n\t\t\n\t\t### feat\n\t\t- Add xyz\n\t\t\n\t\t### fix\n\t\t- Fix \
             abc\n\t\t",
            template.render(&release, Option::<HashMap<&str, String>>::None.as_ref(), &[
                TextProcessor {
                    pattern: Regex::new("<DATE>").expect("failed to compile regex"),
                    replace: Some(String::from("2023")),
                    replace_command: None,
                }
            ],)?
        );
        template.variables.sort();
        assert_eq!(
            vec![
                String::from("commit.group"),
                String::from("commit.message"),
                String::from("commits"),
                String::from("version"),
            ],
            template.variables
        );
        #[cfg(feature = "github")]
        {
            assert!(!template.contains_variable(&["commit.github"]));
            assert!(template.contains_variable(&["commit.group"]));
        }
        Ok(())
    }

    #[test]
    fn render_trimmed_template() -> Result<()> {
        let template = r"
		##  {{ version }}
		";
        let template = Template::new("test", template.to_string(), true)?;
        let release = get_fake_release_data();
        assert_eq!(
            "\n##  1.0\n",
            template.render(&release, Option::<HashMap<&str, String>>::None.as_ref(), &[
            ],)?
        );
        assert_eq!(vec![String::from("version"),], template.variables);
        Ok(())
    }

    #[test]
    fn test_upper_first_filter() -> Result<()> {
        let template = "{% set hello_variable = 'hello' %}{{ hello_variable | upper_first }}";
        let release = get_fake_release_data();
        let template = Template::new("test", template.to_string(), true)?;
        let r = template.render(&release, Option::<HashMap<&str, String>>::None.as_ref(), &[
        ])?;
        assert_eq!("Hello", r);
        Ok(())
    }

    #[test]
    fn test_replace_regex_filter() -> Result<()> {
        let template = "{% set hello_variable = 'hello world' %}{{ hello_variable | \
                        replace_regex(from='o', to='a') }}";
        let release = get_fake_release_data();
        let template = Template::new("test", template.to_string(), true)?;
        let r = template.render(&release, Option::<HashMap<&str, String>>::None.as_ref(), &[
        ])?;
        assert_eq!("hella warld", r);
        Ok(())
    }

    #[test]
    fn test_find_regex_filter() -> Result<()> {
        let template = "{% set hello_variable = 'hello world, hello universe' %}{{ hello_variable \
                        | find_regex(pat='hello') }}";
        let release = get_fake_release_data();
        let template = Template::new("test", template.to_string(), true)?;
        let r = template.render(&release, Option::<HashMap<&str, String>>::None.as_ref(), &[
        ])?;
        assert_eq!("[hello, hello]", r);
        Ok(())
    }

    #[test]
    fn test_split_regex_filter() -> Result<()> {
        let template = "{% set hello_variable = 'hello world, hello universe' %}{{ hello_variable \
                        | split_regex(pat=' ') }}";
        let release = get_fake_release_data();
        let template = Template::new("test", template.to_string(), true)?;
        let r = template.render(&release, Option::<HashMap<&str, String>>::None.as_ref(), &[
        ])?;

        assert_eq!("[hello, world,, hello, universe]", r);
        Ok(())
    }
}
