/// Description for new columns
#[derive(Debug, Clone)]
pub struct ColumnDescription {
    pub name: String,

    // TODO: make this use one of the enums
    /// Type of the data, see the cfitsio documentation
    pub data_type: String,
}

#[derive(Debug, Clone)]
pub struct ColumnDataDescription {
    pub repeat: usize,
    pub width: usize,
    pub typ: ColumnDataType,
}

impl ColumnDataDescription {
    pub fn new(typ: ColumnDataType) -> Self {
        ColumnDataDescription {
            repeat: 1,
            width: 1,
            typ: typ,
        }
    }

    /* XXX These two methods force a call to clone which is wasteful of memory. I do not know if
     * this means that memory is leaked, or that destructors are needlessly called (I suspect the
     * latter) but it is fairly wasteful. On the other hand, it's unlikely this sort of thing will
     * be called in performance-critical code, and is more likely a one-dime definition. I will
     * leave it for now - SRW 2017-03-07
     * */
    pub fn repeats(&mut self, repeat: usize) -> Self {
        // TODO check that repeat >= 1
        self.repeat = repeat;
        self.clone()
    }

    pub fn width(&mut self, width: usize) -> Self {
        // TODO check that width >= 1
        self.width = width;
        self.clone()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ColumnDataType {
    Int,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_column_data_descriptions_builder_pattern() {
        let desc = ColumnDataDescription::new(ColumnDataType::Int)
            .width(100)
            .repeats(5);
        assert_eq!(desc.repeat, 5);
        assert_eq!(desc.width, 100);
    }
}
