//! Culture settings for the C# format type: number separators and currency layout.

/// The cultures with built-in number formats.
#[derive(clap::ValueEnum, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum Culture {
    #[default]
    #[value(name = "en-GB")]
    EnGb,
    #[value(name = "en-US")]
    EnUs,
    #[value(name = "de-DE")]
    DeDe,
    #[value(name = "fr-FR")]
    FrFr,
}

/// Where the currency symbol goes relative to the number.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CurrencyPosition {
    /// `£1,234.50`
    Before,
    /// `1.234,50 €`
    AfterWithSpace,
}

/// The separators and currency layout used to format numbers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct NumberFormat {
    pub(crate) decimal_separator: String,
    pub(crate) group_separator: String,
    pub(crate) currency_symbol: String,
    pub(crate) currency_position: CurrencyPosition,
}

impl Culture {
    /// The culture's number format. fr-FR uses plain spaces where .NET uses
    /// non-breaking ones, so the output is easy to read and to process.
    pub(crate) fn number_format(self) -> NumberFormat {
        let (decimal_separator, group_separator, currency_symbol, currency_position) = match self {
            Culture::EnGb => (".", ",", "£", CurrencyPosition::Before),
            Culture::EnUs => (".", ",", "$", CurrencyPosition::Before),
            Culture::DeDe => (",", ".", "€", CurrencyPosition::AfterWithSpace),
            Culture::FrFr => (",", " ", "€", CurrencyPosition::AfterWithSpace),
        };

        NumberFormat {
            decimal_separator: decimal_separator.to_string(),
            group_separator: group_separator.to_string(),
            currency_symbol: currency_symbol.to_string(),
            currency_position,
        }
    }
}
