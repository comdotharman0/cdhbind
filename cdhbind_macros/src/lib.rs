use proc_macro::*;

#[proc_macro_attribute]
pub fn py_impl(_attr:TokenStream,item:TokenStream)
->TokenStream{
let native_code = item.clone();
let binding= item.to_string();
let stream_string= binding.split(" ");
let return_code= format!(
r#"
 #[cfg(not(feature = "pypi"))]
{}

 #[cfg(feature = "pypi")]
#[no_mangle]
pub extern "C" {}
"#
, native_code.to_string(),
stream_string.skip(1)
.map(|i| {
let mut  j= i.to_string();
j.push_str(" ");
j}) .collect::<String>());
return_code.parse::<TokenStream>().unwrap()


}

#[proc_macro_attribute]
pub fn py_struct(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Force standard C memory layout alignment onto the Rust struct block
    let mut structural_layout = String::new();
    structural_layout.push_str("#[repr(C)]\n");
    structural_layout.push_str(&item.to_string());
    
    structural_layout.parse::<TokenStream>().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2+2;
        assert_eq!(result, 4);
    }
}
