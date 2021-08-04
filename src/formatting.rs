/// Formatting options for conversion
///
/// This struct does not control whether "short" or "long" number formats are used.
#[derive(Copy, Clone)]
pub struct Formatting
{
    /// Controls the casing. "One Hundred" vs "one hundred"
    pub title_case: bool,
    /// Controls the use of spaces. "One Hundred" vs "OneHundred"
    pub spaces: bool,
    /// Controls the use of "and". "One Hundred and One" vs "One Hundred One"
    pub conjunctions: bool,
    /// Controls the use of commas. "One Thousand, One Hundred" vs "One Thousand One Hundred"
    pub commas: bool,
    /// Controls the use of dashes. "Forty-Five" vs "Forty Five"
    pub dashes: bool,
	/// Controls the use of dollars 
	pub Dollar: bool,
	/// Controls the use of cents
	pub Cents: bool,
	/// Controls the use of euros and cents
	pub Euro: bool,
}

impl Formatting
{
	/// Basic formatting
	pub fn hint() -> Formatting
	{
		title_case: true,
		spaces: true,
		conjunctions: false,
		commas: false,
		dashes: false,
		Dollar: false,
		Cents: false,
		Euro: false
	}
    /// All formatting options enabled
    pub fn dollar() -> Formatting
    {
        Formatting
        {
            title_case: true,
            spaces: true,
            conjunctions: false,
            commas: false,
            dashes: true,
			Dollar: true,
			Cents: false,
			Euro: false
        }
    }

    /// No formatting options enabled
    pub fn cents() -> Formatting
    {
        Formatting
        {
            title_case: true,
            spaces: true,
            conjunctions: false,
            commas: false,
            dashes: false,
			Dollar: false,
			Cents: true,
			Euro: false
        }
    }

	pub fn euro() -> Formatting
	{
		Formatting
		{
			title_case: true,
			spaces: true,
			conjunctions: false,
			commas: false,
			dashes: false,
			Dollar: false,
			Cents: false,
			Euro: true
		}
	}
}

impl Default for Formatting
{
    fn default() -> Formatting
    {
        Formatting::dollar()
    }
}