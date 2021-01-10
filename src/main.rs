use select::document::Document;
use select::predicate::Name;
use regex::Regex;

fn main() -> Result<(), ureq::Error> {
    const COUNTRY_CODES: &str = "https://developer.paypal.com/docs/api/reference/country-codes/";

    let body: String = ureq::get(COUNTRY_CODES)
        .call()?
        .into_string()?;

    let doc = Document::from(body.as_str());

    let re = Regex::new(r"\n{2,}").unwrap();

    let mut names = vec![];
    let mut codes = vec![];

    for table in doc.find(Name("table")) {
        let text = table.find(Name("tbody")).next().unwrap().text();
        let text2 = text.replace("Required", "");
        let rep = String::from(re.replace_all(text2.as_str(), "\n"));

        for (i, l) in rep.lines().enumerate() {
            if l.is_empty() {
                continue;
            }
            if i % 2 == 0 {
                codes.push(l.to_owned());
            } else {
                names.push(l.to_owned());
            }

        }
        break;
    }

    assert_eq!(codes.len(), names.len());

    let pairs: Vec<(&String, &String)> = codes.iter().zip(names.iter()).collect();

    println!("//! Generated using https://github.com/edg-l/payhelper");
    println!();
    println!("use serde::{{Deserialize, Serialize}};");
    println!("use crate::errors::InvalidCountryError;");
    println!();
    println!("/// IS0-3166-1 country codes");
    println!("#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]");
    println!("pub enum Country {{");
    for (code, name) in &pairs {
        println!("    /// {}", name);
        println!("    {},", code);
    }
    println!("}}");
    println!();
    println!(r#"impl Default for Country {{
    fn default() -> Self {{
        Self::US
    }}
}}"#);
    println!();
    println!(r#"impl std::fmt::Display for Country {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        std::fmt::Debug::fmt(&self, f)
    }}
}}"#);

    println!("impl FromStr for Country {{");
    println!("    type Err = InvalidCountryError;");
    println!();
    println!("    fn from_str(s: &str) -> Result<Self, Self::Err> {{");
    println!("        match s {{");
    for (code, _) in &pairs {
        println!("            \"{}\" => Ok(Self::{}),", code, code);
    }
    println!("            country => Err(InvalidCountryError(country.to_owned()))");
    println!("        }}");
    println!("    }}");
    println!("}}");

    Ok(())
}
