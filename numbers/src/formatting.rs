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
	/// Controls the use of dollars and cents
	pub Dollar: bool,
	/// Controls the use of euros and cents
	pub Euro: bool,
}

impl Formatting
{
    /// All formatting options enabled
    pub fn all() -> Formatting
    {
        Formatting
        {
            title_case: true,
            spaces: true,
            conjunctions: true,
            commas: false,
            dashes: true,
			Dollar: true,
			Euro: false
        }
    }

    /// No formatting options enabled
    pub fn none() -> Formatting
    {
        Formatting
        {
            title_case: false,
            spaces: false,
            conjunctions: false,
            commas: false,
            dashes: false,
			Dollar: false,
			Euro: false
        }
    }
}

impl Default for Formatting
{
    fn default() -> Formatting
    {
        Formatting::none()
    }
}