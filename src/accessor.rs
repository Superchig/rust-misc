#[macro_export]
macro_rules! getter {
    ( $name:ident, $name_type:ident ) => {
        pub fn $name(&self) -> $name_type {
            self.$name
        }
    }
}

macro_rules! field_getter {
    ( $($name:ident, $name_type:ident),+ ) => {
        $(
            getter!($name, $name_type);
            )+
    }
}

macro_rules! setter {
    ( $name:ident, $set_name:ident, $name_type:ident ) => {
        pub fn $set_name(&mut self, to_set: $name_type) {
            self.$name = to_set;
        }
    }
}

macro_rules! ref_getter {
    ( $name:ident, $name_type:ident ) => {
        pub fn $name(&self) -> &$name_type {
            &self.$name
        }
    }
}