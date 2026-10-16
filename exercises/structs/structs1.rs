// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.(hink:提示)

// 一个经典的C结构体
struct ColorClassicStruct {
    // TODO: Something goes here
    red: u8,
    green: u8,
    blue: u8,
}

// 一个元组结构体
struct ColorTupleStruct(/* TODO: Something goes here */
    u8,
    u8,
    u8
);

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!(实例化一个经典的C结构体!)
        let green = ColorClassicStruct{red:0,green:255,blue:0};

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!(实例化一个元组结构！)
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: 实例化一个类似单元的结构体！
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
