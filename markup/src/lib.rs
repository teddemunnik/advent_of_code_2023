use proc_macro::{TokenStream};
use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Ident,Token, parse_macro_input, LitInt, ItemFn};
use quote::quote;

struct Args{
    year: u32,
    day: u32,
    part: Option<u32>,
}

impl Parse for Args{
    fn parse(input: ParseStream) -> Result<Self>{
        let vars = Punctuated::<LitInt, Token![,]>::parse_terminated(input)?;
        if vars.len() < 2 || vars.len() > 3{
            return Err(syn::Error::new(input.span(), "Expected 2 or 3 arguments (year, day, [part])"))
        }

        let year = vars[0].base10_parse::<u32>().unwrap();
        let day = vars[1].base10_parse::<u32>().unwrap();
        let part = if vars.len() == 3{
            Some(vars[2].base10_parse::<u32>().unwrap())
        }
        else{
            None
        };

        Ok(Args { year, day, part})
    }
}


#[proc_macro_attribute]
pub fn aoc_task(args: TokenStream, input: TokenStream) -> TokenStream{
    let input = parse_macro_input!(input as ItemFn);
    let args= parse_macro_input!(args as Args);

    let year = args.year;
    let day = args.day;
    let part = args.part.unwrap_or(1);

    let task_function_name = input.sig.ident.clone();
    let task_internal_mod_name = input.sig.ident.to_string() + "_aoc_task_internal";
    let task_internal_mod_ident = Ident::new(&task_internal_mod_name, Span::call_site());
    quote!{
        mod #task_internal_mod_ident{
            struct AocTaskImpl;

            impl crate::AocTask for AocTaskImpl{
                fn year(&self) -> u32{
                    #year
                }

                fn day(&self) -> u32{
                    #day 
                }

                fn part(&self) -> u32{
                    #part
                }

                fn invoke(&self, read: &mut dyn std::io::BufRead){
                    super::#task_function_name(read);
                }
            }

            #[linkme::distributed_slice(crate::AOC_ENTRIES)]
            //#[linkme(crate = crate::linkme)]
            static TASK : &(dyn crate::AocTask + Sync) = &AocTaskImpl;
        }

        #input
    }.into()
}