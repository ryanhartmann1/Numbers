use crate::formatting::Formatting;

#[derive(Clone)]
pub struct Words(Vec<Word>);

impl Words
{
    pub fn new(val: Vec<Word>) -> Words
    {
        Words(val)
    }

    pub fn add(&mut self, mut other: Words)
    {
        self.0.append(&mut other.0);
    }

    pub fn build(&self, fmt: Formatting) -> String
    {
        self.0.iter()
            .fold(String::new(), |mut acc, item| { acc.push_str(&item.build(fmt)); acc })
    }

    pub fn is_empty(&self) -> bool
    {
        self.0.len() == 0
    }
}

#[derive(Clone)]
pub enum Word
{
    Number(String),
    And,
    Dash,
    Comma,
    Space,
	Dollar,
	Cents,
	Euro,
	Tenths,
	Hundreths,
	Thousandths,
	TenThousandths,
	HundredThousandths,
	Millionths,
}

impl Word
{
    pub fn build(&self, fmt: Formatting) -> String
    {
        match self
        {
            &Word::Number(ref n) => if fmt.title_case && n.len() > 0
            {
                let first = n.chars().nth(0).unwrap().to_string().to_uppercase();
                let last = &n[1..n.len()];

                format!("{}{}", first, last)
            }
            else
            {
                n.to_owned()
            },
            &Word::And => if fmt.conjunctions && fmt.spaces
            {
                String::from(" and ")
            }
            else if fmt.conjunctions
            {
                String::from("and")
            }
            else if fmt.spaces
            {
                String::from(" ")
            }
            else
            {
                String::new()
            },
            &Word::Dash => if fmt.dashes
            {
                String::from("-")
            }
            else if fmt.spaces
            {
                String::from(" ")
            }
            else
            {
                String::new()
            },
            &Word::Comma => if fmt.commas && fmt.spaces
            {
                String::from(", ")
            }
            else if fmt.spaces
            {
                String::from(" ")
            }
            else
            {
                String::new()
            },
            &Word::Space => if fmt.spaces
            {
                String::from(" ")
            }
            else
            {
                String::new()
            }
			&Word::Dollar => if fmt.Dollar
			{
				String::from(" Dollars")
			}
			else
			{
				String::new()
			}
			&Word::Cents => if fmt.Cents
			{
				String::from(" Cents")
			}
			else
			{
				String::new()
			}
			&Word::Euro => if fmt.Euro
			{
				String::from(" Euros")
			}
			else
			{
				String::new()
			}
			&Word::Tenths => if fmt.Decimal
			{
				String::from(" Tenths")
			}
			else
			{
				String::new()
			}
			&Word::Hundreths => if fmt.Decimal
			{
				String::from(" Hundreths")
			}
			else
			{
				String::new()
			}
			&Word::Thousandths => if fmt.Decimal
			{
				String::from(" Tousandths")
			}
			else
			{
				String::new()
			}
			&Word::TenThousandths => if fmt.Decimal
			{
				String::from(" TenThousandths")
			}
			else
			{
				String::new()
			}
			&Word::HundredThousandths => if fmt.Decimal
			{
				String::from(" HundredThousandths")
			}
			else
			{
				String::new()
			}
			&Word::Millionths => if fmt.Decimal
			{
				String::from(" Millionths")
			}
			else
			{
				String::new()
			}
        }
    }
}