use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::{Output, OutputValue};
use crate::range;

use icu::collator::provider::CollationSpecialPrimariesV1;
use icu::locid::Locale;
use icu::collator::*;
//use icu_provider::{AnyProvider, AsDeserializingBufferProvider, DataLocale};
use icu_provider::prelude::*;
use icu_provider_blob::BlobDataProvider;
use icu_provider_adapters::fallback::LocaleFallbackProvider;

pub struct Sort {}

impl traits::Op for Sort {
    fn name() -> &'static str { "sort" }
    fn usage() -> &'static str { "[<#1 string to-sort> one or more times]" }
    fn description() -> &'static str { "sort the strings provided" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexOpen(1) }
    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> { Ok(0.5) }

    fn run(args: &Args, conf: &Configuration) -> Result<Output, LibError> {
        // fetch arguments as strings
        let mut strings = vec![];
        for arg in args.iter() {
            strings.push(arg.str_or_panic());
        }

        // TODO move this to input.rs
        let blob = include_bytes!("../../data/icu4x.blob2");
        //let blob_provider = BlobDataProvider::try_new_from_blob(Box::new(*blob))?.as_deserializing().try_into();

        let blob_provider = BlobDataProvider::try_new_from_static_blob(blob)?;
        //let data_provider: &dyn DataProvider<CollationSpecialPrimariesV1> = &blob_provider.as_deserializing();
        let buffer_provider = LocaleFallbackProvider::try_new_with_buffer_provider(blob_provider)?;

        // prepare collation
        let locale: Locale = conf.locale.parse()?;
        let data_locale = DataLocale::from(&locale);
        let mut options = CollatorOptions::new();
        options.strength = Some(Strength::Primary);
        //let collator: Collator = Collator::try_new_unstable(&data_provider, &data_locale, options).unwrap();
        let collator: Collator = Collator::try_new_with_buffer_provider(&buffer_provider, &data_locale, options).unwrap();

        strings.sort_by(|a, b| collator.compare(a, b));

        Ok(Output::HomogeneousList {
            data: strings.iter().map(|s| OutputValue::from_str(s)).collect::<Vec<OutputValue>>(),
            notes: vec![],
        })
    }
}
