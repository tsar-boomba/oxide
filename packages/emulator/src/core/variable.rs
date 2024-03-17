use std::ffi::{c_char, CStr};

#[derive(Debug)]
pub struct Variable {
    key: Box<str>,
    /// For ENVIRONMENT_GET_VARIABLE this may be null
    value: Option<Box<str>>,
}

#[derive(Debug)]
pub struct VariableDef {
    key: Box<str>,
    desc: Box<str>,
    opts: Box<[Box<str>]>,
}

impl Variable {
    /// This function assumes `raw` is a non-null pointer to a valid libretro `Variable` struct
    pub unsafe fn from_raw(raw: *const libretro_sys::Variable) -> Self {
        let raw = &*(raw);
        Self {
            key: raw_cstr_to_boxed_str(raw.key.cast()),
            value: if raw.value.is_null() {
                None
            } else {
                Some(raw_cstr_to_boxed_str(raw.value.cast()))
            },
        }
    }
}

impl VariableDef {
    /// ENVIRONMENT_SET_VARIABLES sends an array of `retro_variable`s terminated with two NULL pointers
    /// This is used to take that and clone the data out of it onto the heap
    pub unsafe fn from_raw_array(mut raw: *const *const u8) -> Box<[Self]> {
        let mut result = Vec::<Self>::new();
        let mut i = 0;

        loop {
            // Pull two items from array and check if they're both null
            let raw_key = raw.offset(i * 2).read();
            let raw_value = raw.offset((i * 2) + 1).read();

            if raw_key.is_null() && raw_value.is_null() {
                break;
            }

            let key = raw_cstr_to_boxed_str(raw_key.cast());
            // Borrow str to parse it before allocating
            let unparsed_value = CStr::from_ptr(raw_value.cast()).to_str().unwrap();
            let (desc, opts) = unparsed_value
                .split_once("; ")
                .expect("Malformed Variable desc/options");

            result.push(Self {
                key,
                desc: desc.into(),
                opts: opts
                    .split("|")
                    .map(|opt| <&str as Into<Box<str>>>::into(opt))
                    .collect::<Box<[Box<str>]>>(),
            });
            i += 1;
        }

        result.into_boxed_slice()
    }
}

unsafe fn raw_cstr_to_boxed_str(raw_cstr: *const c_char) -> Box<str> {
    if raw_cstr.is_null() {
        panic!("Received a null pointer.")
    }

    CStr::from_ptr(raw_cstr.cast()).to_str().unwrap().into()
}
