use xmltree::Element;

macro_rules! try {
    ($e:expr) => {
        $e.expect(concat!(file!(), ":", line!(), " ", stringify!($e)))
    }
}

pub fn u32(tree: &Element) -> Option<u32> {
    let text = try!(tree.text.as_ref());

    if text.starts_with("0x") || text.starts_with("0X") {
        u32::from_str_radix(&text["0x".len()..], 16).ok()
    } else if text.starts_with('#') {
        // Handle strings in the binary form of:
        // #01101x1
        // along with don't care character x (replaced with 0)
        u32::from_str_radix(&str::replace(&text["#".len()..], "x", "0"), 2).ok()
    } else {
        text.parse().ok()
    }
}

pub fn bool(tree: &Element) -> Option<bool> {
    let text = try!(tree.text.as_ref());
    match text.as_ref() {
        "0" => Some(false),
        "1" => Some(true),
        _ => text.parse::<bool>().ok(),
    }
}

pub fn dim_index(text: &str) -> Vec<String> {
    if text.contains('-') {
        let mut parts = text.splitn(2, '-');
        let start = try!(try!(parts.next()).parse::<u32>());
        let end = try!(try!(parts.next()).parse::<u32>()) + 1;

        (start..end).map(|i| i.to_string()).collect()
    } else if text.contains(',') {
        text.split(',').map(|s| s.to_string()).collect()
    } else {
        unreachable!()
    }
}
