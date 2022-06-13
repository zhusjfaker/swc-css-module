mod file;
mod css_module;
mod hash;

#[cfg(test)]
mod tests {
  use swc_common::input::SourceFileInput;
  use swc_common::{FileName, SourceMap};
  use swc_common::sync::Lrc;
  use swc_css_codegen::{CodegenConfig, CodeGenerator, Emit};
  use swc_css_codegen::writer::basic::{BasicCssWriter, BasicCssWriterConfig};
  use swc_css_parser::lexer::Lexer;
  use swc_css_parser::parser::{Parser, ParserConfig};
  use crate::css_module::CssModuleComponent;
  use crate::file::{path_resolve, readfile};
  use swc_css_visit::FoldWith;
  
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
  
  #[test]
  fn test_css_swc_module() {
    let filepath = path_resolve("assets/css_modules/index.module.css");
    let content = readfile(filepath.as_str()).unwrap();
    let config = ParserConfig {
      ..Default::default()
    };
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(FileName::Custom(filepath.clone()), content.clone().into());
    let lexer = Lexer::new(SourceFileInput::from(&*fm), config);
    let mut parser = Parser::new(lexer, config);
    let stylesheet = parser.parse_all().unwrap();
    let _json = serde_json::to_string_pretty(&stylesheet).unwrap();
    let mut visitor = CssModuleComponent::new(filepath.as_str(), content.as_str());
    let new_stylesheet = stylesheet.fold_with(&mut visitor);
    let mut css_str = String::new();
    {
      let wr = BasicCssWriter::new(
        &mut css_str,
        None, // Some(&mut src_map_buf),
        BasicCssWriterConfig::default(),
      );
      let mut gen = CodeGenerator::new(wr, CodegenConfig { minify: false });
      gen.emit(&new_stylesheet).unwrap();
    }
    println!("css output -> \n {}", css_str);
  }
}
