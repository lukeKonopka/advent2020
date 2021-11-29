#[derive(Debug, Clone, Copy)]
pub struct Rule(pub u16, pub u16);

#[derive(Debug)]
pub struct Field<'a> {
    pub name: &'a str,
    pub rules: (Rule, Rule),
}

pub struct Ticket {
    pub field_values: Vec<u16>,
}

impl std::fmt::Debug for Ticket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let values = self
            .field_values
            .iter()
            .map(|a| format!("{}", a))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "\"{}\"", values)
    }
}

#[derive(Debug)]
pub struct Input<'a> {
    pub fields: Vec<Field<'a>>,
    pub your_ticket: Ticket,
    pub nearby_tickets: Vec<Ticket>,
}

pub fn parse(input: &str) -> Option<Input> {
    let mut iter = input.split("\n\n");
    let fields = iter
        .next()?
        .lines()
        .map(parse_field)
        .collect::<Option<Vec<_>>>()?;
    let your_ticket = iter.next()?.lines().skip(1).next().and_then(parse_ticket)?;
    let nearby_tickets = iter
        .next()?
        .lines()
        .skip(1)
        .map(parse_ticket)
        .collect::<Option<Vec<_>>>()?;
    Some(Input {
        fields,
        your_ticket,
        nearby_tickets,
    })
}

fn parse_ticket(input: &str) -> Option<Ticket> {
    let mut field_values = vec![];
    for value in input.split(',') {
        let num = value.parse::<u16>().ok()?;
        field_values.push(num);
    }
    Some(Ticket { field_values })
}

fn parse_field(input: &str) -> Option<Field> {
    let mut iter = input.split(": ");
    let name = iter.next()?;
    let rules_str = iter.next()?;
    let mut rules_iter = rules_str.split(" or ");
    let first_rule = rules_iter.next().and_then(parse_rule)?;
    let second_rule = rules_iter.next().and_then(parse_rule)?;
    Some(Field {
        name,
        rules: (first_rule, second_rule),
    })
}

fn parse_rule(input: &str) -> Option<Rule> {
    let mut iter = input.split('-');
    let from = iter.next()?;
    let to = iter.next()?;
    let from_num = from.parse::<u16>().ok()?;
    let to_num = to.parse::<u16>().ok()?;
    Some(Rule(from_num, to_num))
}
