extern crate paste;

#[macro_export]
macro_rules! create_enum {
    ($enum:ident, $type:ident, $func_name:ident,
    ($($field_name:ident),*$(,)*)) => {
        #[derive(Clone, Copy, Debug)]
        pub enum $enum {
            $($field_name,)*
        }

        impl $enum {
            paste::item! {
                pub(crate) fn [<to_ $func_name>](&self) -> $type {
                    let pass = match self {
                        $(
                            $enum::$field_name => $type::[<FFMS_ $field_name>],
                        )*
                    };
                pass
                }
            }
        }
    }
}

#[macro_export]
macro_rules! errors {
    ($enum:ident, $type:ident, $func_name:ident,
    ($($field_name:ident: $field_err:expr),*$(,)*)) => {

        create_enum!($enum, $type, $func_name, ($($field_name,)*));

        impl fmt::Display for $enum {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let v = match self {
                    $(
                        $enum::$field_name => $field_err,
                    )*
                };

                write!(f, "{}", v)
            }
        }

        impl $enum {
            paste::item! {
                fn [<from_ $func_name>](e: &$type) -> Self {
                    match e {
                        $(
                            $type::[<FFMS_ $field_name>] => $enum::$field_name,
                        )*
                    }
                }
            }
        }
    }
}

#[macro_export]
macro_rules! set_struct {
    ($struct:ident, $param:ident, $type:tt) => {
        pub struct $struct {
            $param: $type,
        }
    };
}

#[macro_export]
macro_rules! default_struct {
    ($struct:ident, $param:ident, $type:tt,
     ($($field_name:ident),*$(,)*),
     ($($field_default_expr:expr),*$(,)*),
     ($(($feature:meta, $feature_name:ident, $feature_expr:expr)),*$(,)*))
     => {
      impl Default for $struct {
            fn default() -> Self {
                let $param = $type {
                    $($field_name: $field_default_expr,)*
                    $(
                        #[$feature]
                        $feature_name: $feature_expr,
                    )*
                };
                $struct{ $param }
            }
        }
    }
}

#[macro_export]
macro_rules! create_struct {
    ($struct:ident, $param:ident, $type:tt,
     ($($field_name:ident),*$(,)*),
     ($($field_type:ty),*$(,)*),
     ($($field_default_expr:expr),*$(,)*),
     ($($field_expr:expr),*$(,)*)
     ) => {

        set_struct!($struct, $param, $type);

        default_struct!($struct, $param, $type,
                       ($($field_name,)*),
                       ($($field_default_expr,)*),
                       ());

        set_params!($struct, $param,
                   ($($field_name,)*),
                   ($($field_type,)*),
                   ($($field_expr,)*));
    }
}

#[macro_export]
macro_rules! set_params {
    ($struct:ident, $param:ident,
    ($($field_name:ident),*$(,)*),
    ($($field_type:ty),*$(,)*),
    ($($field_expr:expr),*$(,)*))
    => {
            impl $struct {
                paste::item! {
                    $(
                        pub fn [<set_ $field_name>](&mut self, $field_name: $field_type) {
                            self.$param.$field_name = paste::expr! { ($field_expr) }
                        }
                    )*
                }
            }
       }
}

#[macro_export]
macro_rules! set_feature_params {
    ($struct:ident, $param:ident,
    ($($feat:meta, $feat_name:ident, $feat_type:ty, $feat_expr:expr),*$(,)*))
    => {
            impl $struct {
                paste::item! {
                    $(
                        #[$feat]
                        pub fn [<set_ $feat_name>](&mut self, $feat_name: $feat_type) {
                            self.$param.$feat_name = paste::expr! { ($feat_expr) }
                        }
                    )*
                }
            }
       }
}