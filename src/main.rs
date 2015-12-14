extern crate docopt;
extern crate rustc_serialize;
extern crate hyper;
extern crate xml;

mod helpers;

use std::io::Read;
use helpers::FormSite;

use docopt::Docopt;

const USAGE: &'static str = "
FORMSITE_TEST
Usage:
    formsite_test <api-key> <server> <account> [--form=<form>, --transaction=<trans>]
    formsite_test (-h | --help)
    formsite_test --version
Options:
    -h --help               Prints this message.
    --version               Prints version.
    --form FORM             Form name/directory (see Formsite API docs).
    --transaction TRANS     API transaction object (see Formsite API docs).
";

#[derive(Debug,RustcDecodable)]
struct Args {
    arg_api_key: String,
    arg_server: usize,
    arg_account: String,
    flag_form: Option<String>,
    flag_transaction: Option<String>,
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

fn main() {
    // parse Args
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    // test FormSite API
    let fs = FormSite::new(args.arg_api_key,
                           args.arg_server,
                           args.arg_account,
                           args.flag_form,
                           args.flag_transaction);
    let mut res = fs.get();
    println!("Status: {}", res.status);
    let mut body = String::new();
    let _ = res.read_to_string(&mut body);
    let parser = xml::reader::EventReader::from_str(&body);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(xml::reader::XmlEvent::StartElement { name, .. }) => {
                println!("{}+{}", indent(depth), name);
                depth += 1;
            }
            Ok(xml::reader::XmlEvent::Characters(s)) => {
                println!("{} {}", indent(depth), s);
            }
            Ok(xml::reader::XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{}-{}", indent(depth), name);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
