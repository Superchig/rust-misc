#[macro_use]
pub mod accessor;

#[macro_export]
macro_rules! getter {
    ( $name:ident, $name_type:ident ) => {
        pub fn $name(&self) -> $name_type {
            self.$name
        }
    }
}

#[macro_export]
macro_rules! field_getter {
    ( $($name:ident, $name_type:ident),+ ) => {
        $(
            getter!($name, $name_type);
            )+
    }
}

#[macro_export]
macro_rules! setter {
    ( $name:ident, $set_name:ident, $name_type:ident ) => {
        pub fn $set_name(&mut self, to_set: $name_type) {
            self.$name = to_set;
        }
    }
}

#[macro_export]
macro_rules! field_setter {
    ( $($name:ident, $set_name:ident, $name_type:ident ),+ ) => {
        $(
            setter!($ident, $set_name, $name_type);
            )+
    }
}

#[macro_export]
macro_rules! ref_getter {
    ( $name:ident, $name_type:ident ) => {
        pub fn $name(&self) -> &$name_type {
            &self.$name
        }
    }
}
