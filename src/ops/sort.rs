use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::{Output, OutputValue};
use crate::range;

pub struct Sort {}

impl Sort {
    #[cfg(feature = "icu")]
    fn sort_strings(conf: &Configuration, notes: &mut Vec<String>, mut strings: Vec<String>) -> Result<Vec<String>, LibError> {
        use std::env;
        use std::fs;

        // TODO reduce number of imports
        use icu::collator::*;
        use icu_provider::prelude::*;
        use icu_provider_blob::BlobDataProvider;
        use icu_provider_adapters::fallback::LocaleFallbackProvider;

        // prepare collation
        if let Some(loc) = &conf.locale {
            // read locale data file or use default data contained in binary
            let blob_provider = if let Ok(filepath_template) = env::var("OPSTR_LOCALE_DATAFILE") {
                let filepath = filepath_template.replace("{locale}", &loc.to_string());
                notes.push(format!("Using locale data from '{}' to initialize collator to sort strings", filepath));

                let errmsg = format!("file '{}' mentioned in environment variable OPSTR_LOCALE_DATAFILE cannot be read", filepath);
                let data = fs::read(&filepath).expect(&errmsg);

                BlobDataProvider::try_new_from_blob(data.into_boxed_slice())?
            } else {
                notes.push(format!("Using default locale data shipped with this program to initialize collator to sort strings"));
                BlobDataProvider::try_new_from_static_blob(crate::DEFAULT_LOCALE_DATA)?
            };
            let buffer_provider = LocaleFallbackProvider::try_new_with_buffer_provider(blob_provider)?;

            // create collator to sort strings
            let data_locale = DataLocale::from(loc);
            let mut options = CollatorOptions::new();
            options.strength = Some(Strength::Primary);
            let collator: Collator = Collator::try_new_with_buffer_provider(&buffer_provider, &data_locale, options)?;

            strings.sort_by(|a, b| collator.compare(a, b));
        } else {
            // Sort strings lexicographically.
            // Sorting locale-dependent “is outside the scope of the `str` type”
            // https://doc.rust-lang.org/std/primitive.str.html#impl-Ord-for-str
            strings.sort();
        }

        Ok(strings)
    }

    #[cfg(not(feature = "icu"))]
    fn sort_strings(_conf: &Configuration, _notes: &mut Vec<String>, mut strings: Vec<String>) -> Result<Vec<String>, LibError> {
        // Sort strings lexicographically.
        // Sorting locale-dependent “is outside the scope of the `str` type”
        // https://doc.rust-lang.org/std/primitive.str.html#impl-Ord-for-str
        strings.sort();
        Ok(strings)
    }
}

impl traits::Op for Sort {
    fn name() -> &'static str { "sort" }
    fn usage() -> &'static str { "[<#1 string to-sort> one or more times]" }
    fn description() -> &'static str { "sort the strings provided" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexOpen(1) }
    fn priority(_args: &Args, _conf: &Configuration) -> Result<f32, LibError> { Ok(0.5) }

    fn run(args: &Args, conf: &Configuration) -> Result<Output, LibError> {
        let mut notes = vec![];

        // fetch arguments as strings
        let mut strings: Vec<String> = vec![];
        for arg in args.iter() {
            strings.push(arg.str_or_panic().to_string());
        }

        strings = Self::sort_strings(conf, &mut notes, strings)?;

        Ok(Output::HomogeneousList {
            data: strings.iter().map(|s| OutputValue::from_str(s)).collect::<Vec<OutputValue>>(),
            notes,
        })
    }
}
