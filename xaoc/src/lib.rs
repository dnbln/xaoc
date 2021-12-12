pub extern crate linkme;
pub use linkme::distributed_slice as ds;

pub struct DayData {
    year: u32,
    day: u32,
}

impl DayData {
    pub fn is(&self, y: u32, d: u32) -> bool {
        self.year == y && self.day == d
    }
}

pub struct ExampleData<OP1, OP2> {
    input: &'static str,
    output_part1: Option<OP1>,
    output_part2: Option<OP2>,
}

impl DayData {
    #[must_use]
    pub const fn new(year: u32, day: u32) -> DayData {
        DayData { year, day }
    }
}

impl<OP1, OP2> ExampleData<OP1, OP2> {
    #[must_use]
    pub const fn new(
        input: &'static str,
        output_part1: Option<OP1>,
        output_part2: Option<OP2>,
    ) -> Self {
        Self {
            input,
            output_part1,
            output_part2,
        }
    }

    #[must_use]
    pub const fn input(&self) -> &'static str {
        self.input
    }

    #[must_use]
    pub const fn output_p1(&self) -> Option<&OP1> {
        self.output_part1.as_ref()
    }

    #[must_use]
    pub const fn output_p2(&self) -> Option<&OP2> {
        self.output_part2.as_ref()
    }
}
