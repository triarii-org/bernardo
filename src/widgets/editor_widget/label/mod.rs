mod label;
pub use label::{Label, LabelPos, LabelStyle};

mod labels_provider;
pub use labels_provider::{LabelsProvider, LabelsProviderRef};

mod rustc_output_parser_label_provider;
pub use rustc_output_parser_label_provider::RustcOutputParserLabelProvider;
