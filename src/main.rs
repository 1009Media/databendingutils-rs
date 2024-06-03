mod tosbr;
mod fromsbr;
mod tohbmp;
mod fromhbmp;

use qmetaobject::*;
use std::ffi::CString;

#[derive(Default, QObject)]
struct ImageConverter {
    base: qt_base_class!(trait QObject),
    input_file: qt_property!(QString; NOTIFY input_file_changed),
    output_file: qt_property!(QString; NOTIFY output_file_changed),
    header_file: qt_property!(QString; NOTIFY header_file_changed),

    convert_to_sbr: qt_method!(fn(&mut self)),
    convert_to_hbmp: qt_method!(fn(&mut self)),
    convert_from_sbr: qt_method!(fn(&mut self)),
    convert_from_hbmp: qt_method!(fn(&mut self)),
    show_instructions: qt_signal!(),
    show_info: qt_signal!(),
    input_file_changed: qt_signal!(),
    output_file_changed: qt_signal!(),
    header_file_changed: qt_signal!(),
}

impl ImageConverter {
    fn convert_to_sbr(&mut self) {
        let input = self.input_file.to_string();
        let output = self.output_file.to_string();
        let header = self.header_file.to_string();
        if let Err(e) = tosbr::convert_to_sbr(&input, &output, &header) {
            eprintln!("Error: {}", e);
        }
    }

    fn convert_to_hbmp(&mut self) {
        let input = self.input_file.to_string();
        let output = self.output_file.to_string();
        let header = self.header_file.to_string();
        if let Err(e) = tohbmp::convert_to_hbmp(&input, &output, &header) {
            eprintln!("Error: {}", e);
        }
    }

    fn convert_from_sbr(&mut self) {
        let input = self.input_file.to_string();
        let output = self.output_file.to_string();
        let header = self.header_file.to_string();
        if let Err(e) = fromsbr::convert_from_sbr(&input, &header, &output) {
            eprintln!("Error: {}", e);
        }
    }

    fn convert_from_hbmp(&mut self) {
        let input = self.input_file.to_string();
        let output = self.output_file.to_string();
        let header = self.header_file.to_string();
        if let Err(e) = fromhbmp::convert_from_hbmp(&input, &header, &output) {
            eprintln!("Error: {}", e);
        }
    }
}

fn main() {
    qmetaobject::qml_register_type::<ImageConverter>(
        CString::new("ImageConverter").unwrap().as_c_str(),
        1,
        0,
        CString::new("ImageConverter").unwrap().as_c_str(),
    );

    let mut engine = QmlEngine::new();
    engine.load_file("src/main.qml".into());
    engine.exec();
}
