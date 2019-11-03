use pandoc;
use std::fs;
use std::env;
use regex::Regex;
// use std::path;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1{
        println!("No file included.");
        return
    }

    let old_file = args[1].clone();
    let data = match fs::read_to_string(&old_file) {
        Ok(x) => x,
        Err(e) => {
            println!("{:?}", e);
            return
        }
    };
    // let re = Regex::new("\\$\\$\n\\begin\\{align\\*\\}").unwrap();
    // let re = Regex::new(r"(?P<dolla>\$\$)").unwrap();
    let re1 = Regex::new(r"(?P<dolla>\$\$)\n\\begin\{align\*\}").unwrap();
    let re2 = Regex::new(r"\\end\{align\*\}\n\$\$").unwrap();
    
    let p1 = re1.replace_all(&data, "\n\\begin{align*}").to_string();
    let val =  re2.replace_all(&p1, "\\end{align*}\n").to_string();

    let filename = old_file.replace(".md", ".pdf");

    // let old_file = path::PathBuf::from(old_file);
    // let filename= path::PathBuf::from(filename);
    let mut p = pandoc::new();
    p.set_input(pandoc::InputKind::Pipe(val));
    p.set_output(pandoc::OutputKind::File(filename));
    let r = p.execute();

    match r {
        Ok(pandoc::PandocOutput::ToBuffer(z)) => println!("{:?}", z),
        Ok(pandoc::PandocOutput::ToFile(z)) => println!("{:?}", z),
        Err(x) => println!("{:?}", x)
    }
    // let filename = &args[1].replace_range((&args[1].len()-3)..&args[1].len(), ".pdf");
    // println!("{:?}", filename);
    return 
}
