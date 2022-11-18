use serde::ser::Serialize;

static TEMPLATES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates");

/// Render a template.
/// 
/// # Arguments
/// * `template` - The name of the template to render.
/// * `context` - The context to expose to the template.
pub fn render_template< S: AsRef<str>, T: Serialize>(template: S, context: T) {

}