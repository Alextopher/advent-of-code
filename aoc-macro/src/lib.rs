//! This modules creates a macro `get_input!(directory, day)` that will download the input
//! file if needed, and then read the input file. Returns a `&'static str` containing the contents of the file.

/// This macro will download the input file if needed, and then read the input file.
/// Returns a `&'static str` containing the contents of the file.
///
/// # Arguments
///
/// * `year` - the year of the advent of code challenge
/// * `day` - the day of the advent of code challenge
extern crate proc_macro;
use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};

struct InputArgs {
    year: u32,
    day: u32,
}

impl Parse for InputArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let year: syn::LitInt = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let day: syn::LitInt = input.parse()?;

        Ok(InputArgs {
            year: year.base10_parse()?,
            day: day.base10_parse()?,
        })
    }
}

#[proc_macro]
pub fn input_str(input: TokenStream) -> TokenStream {
    // Step 1: parse the input using the syn crate
    let args = syn::parse_macro_input!(input as InputArgs);

    // Step 2: get the input file if it exists
    let input_file = format!("inputs/{}-{:02}.txt", args.year, args.day);

    let input_file = std::path::Path::new(&input_file);

    if !input_file.exists() {
        // Step 3: get the session from the cookie file, or from an environment variable
        let mut cookie = std::env::var("AOC_COOKIE").ok();

        if cookie.is_none() {
            cookie = std::fs::read_to_string("inputs/cookie.txt").ok();
        }

        let cookie = cookie.expect("Failed to get cookie");

        // Step 4: download the input file
        let client = reqwest::blocking::Client::new();
        let url = format!(
            "https://adventofcode.com/{}/day/{}/input",
            args.year, args.day
        );

        let resp = client
            .get(url)
            .header("Cookie", cookie)
            .send()
            .expect("Failed to send request");

        // Verify that the code is 200
        if resp.status() != reqwest::StatusCode::OK {
            panic!("Failed to get input file");
        }

        // Step 5: write the input file
        std::fs::write(input_file, resp.text().expect("Failed to get text"))
            .expect("Failed to write input file");
    }

    // Step 6: read the input file
    let input = std::fs::read_to_string(input_file).expect("Failed to read input file");

    // Step 7: return the input file as a `&'static str`
    let input = format!("r#\"{}\"#", input);
    input.parse().expect("Failed to parse input")
}
