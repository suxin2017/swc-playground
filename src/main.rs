use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use swc::{Compiler, HandlerOpts};
use once_cell::sync::Lazy;
use serde_json::json;
use swc::config::{Config, ExperimentalOptions, JscConfig, JscExperimental, Options, PluginConfig};
pub use swc_common::{
    comments::{self, SingleThreadedComments},
    errors::Handler,
    FileName, Mark, GLOBALS,
};
pub use swc_ecma_visit::VisitMutWith;
use swc_common::{sync::Lrc, FilePathMapping, SourceMap};
pub use swc_ecma_transforms::{pass::noop, resolver};

/// Get global sourcemap
pub fn compiler() -> Arc<Compiler> {
    // console_error_panic_hook::set_once();

    static C: Lazy<Arc<Compiler>> = Lazy::new(|| {
        let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));

        Arc::new(Compiler::new(cm))
    });

    C.clone()
}

#[tokio::main]
async fn main() {
    let c = compiler();
    let cm = c.cm.clone();

    let config = HandlerOpts::default();
    GLOBALS.set(&Default::default(), || {
        swc::try_with_handler(cm, config, |handler|{

            let fm = c.cm.new_source_file(
                   FileName::Anon,
                "foo === bar;".into(),
            );
            let cm = c.cm.clone();
            let file = fm.clone();
            let comments = SingleThreadedComments::default();
            dbg!(env::current_dir());
            let current = env::current_dir().unwrap();
            let plugin_path  = current.join("my_first_plugin.wasm");
            dbg!(plugin_path.to_string_lossy().to_string());

           let out =  c.process_js_file(
               fm,
               handler,
               &Options{
               config:Config{
                   jsc:JscConfig{
                       experimental:JscExperimental{
                           plugins:Some(
                               vec![PluginConfig(plugin_path.to_string_lossy().to_string(),json!(null))]
                           ),
                           ..Default::default()
                       },
                     ..Default::default()
                   },
                   ..Default::default()
               },
               ..Default::default()
           },
           )?;
            dbg!(out.code);
            Ok(())
        }).expect("TODO:")
    })

}
