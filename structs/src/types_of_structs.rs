// There are 3 types of structures that can be created
// using the struct keyword:

struct ColorClassicStruct<'a> {
    name: &'a str,
    hex: &'a str,
}

pub fn classic_c_structs() {
    let green = ColorClassicStruct {
        name: "green",
        hex: "#00FF00",
    };
    assert_eq!(green.name, "green");
    assert_eq!(green.hex, "#00FF00");
}

struct ColorTupleStruct<'a>(&'a str, &'a str);

pub fn tuple_structs() {
    let green = ColorTupleStruct("green", "#00FF00");
    assert_eq!(green.0, "green");
    assert_eq!(green.1, "#00FF00");
}

#[derive(Debug)]
struct UnitStruct;

pub fn unit_structs() {
    let unit_struct = UnitStruct;
    let message = format!("{:?}s are fun!", unit_struct);
    assert_eq!(message, "UnitStructs are fun!");
}
