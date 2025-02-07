use super::opcode_result_type::*;
use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use std::collections::HashMap;

pub fn create<'a>(
    key: &'a String,
    value: &ValueType,
    target: &mut HashMap<&'a String, ValueType>,
) -> Result<OpCodeResultType, String> {
    match value {
        Int(int) => target.insert(key, Int(*int)),
        Line(str) => match target.get(str) {
            Some(some) => target.insert(key, some.clone()),
            None => match complex_assignments_value(str, target) {
                Some(some_value) => target.insert(key, some_value),
                None => target.insert(key, value.clone()),
            },
        },
        Arr(_arr) => target.insert(key, Arr(Vec::new())),
    };

    Ok(OpCodeResultType::Empty)
}

fn complex_assignments_value<'a>(
    expression: &String,
    target: &HashMap<&'a String, ValueType>,
) -> Option<ValueType> {
    let values: Vec<&str> = expression.split_whitespace().collect();

    if values.len() == 2 {
        multiple_values(values, target)
    } else if values.len() == 1 {
        one_value(values, target)
    } else {
        return None;
    }
}

// > a b 1
fn multiple_values<'a>(
    values: Vec<&str>,
    target: &HashMap<&'a String, ValueType>,
) -> Option<ValueType> {
    let index = match values[1].to_string().parse::<f64>() {
        Ok(p) => p,
        Err(_) => match target.get(&values[1].to_string()) {
            Some(some) => match some {
                Int(i) => *i,
                _ => return None,
            },
            None => return None,
        },
    };

    match target.get(&values[0].to_string()) {
        Some(some) => match some {
            Int(_i) => None,
            Line(l) => Some(Line(parse_char(l, index))),
            Arr(arr) => Some(parse_elem(arr, index)),
        },
        None => None,
    }
}

fn parse_elem(vec: &Vec<ValueType>, index: f64) -> ValueType {
    let elem = &vec[index as usize];

    match elem {
        Line(l) => Line(l.to_string()),
        Int(i) => Int(*i),
        Arr(a) => Arr(a.to_vec()),
    }
}

fn parse_char(str: &String, index: f64) -> String {
    if (index as usize) > (str.len() - 1) {
        return "".to_string();
    } else {
        return str.chars().nth(index as usize).unwrap().to_string();
    }
}

// > a b
fn one_value<'a>(values: Vec<&str>, target: &HashMap<&'a String, ValueType>) -> Option<ValueType> {
    match target.get(&values[0].to_string()) {
        Some(s) => match s {
            Int(int) => Some(Int(*int)),
            Line(str) => Some(Line(str.to_string())),
            Arr(arr) => Some(Arr(arr.to_vec())),
        },
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;

    #[test]
    fn test_basic_create_one() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        let _ = create(&binding, &Int(10.0), &mut map);
        let result = map.get(&binding);
        assert_eq!(result, Some(&Int(10.0)));
    }

    #[test]
    fn test_basic_create_two() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        let _ = create(&binding, &Line("b 2".to_string()), &mut map);
        let result = map.get(&binding);
        assert_eq!(result, Some(&Line("b 2".to_string())));
    }

    #[test]
    fn test_basic_create_three() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        let _ = create(&binding, &Line("qwert".to_string()), &mut map);
        let result = map.get(&binding);
        assert_eq!(result, Some(&Line("qwert".to_string())));
    }

    #[test]
    fn test_not_basic_create_one() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let old_key = String::from("old_key");
        let new_key = String::from("new_key");

        map.insert(&old_key, Int(10.0));

        let _ = create(&new_key, &Line("old_key".to_string()), &mut map);

        let result_1 = map.get(&old_key);
        let result_2 = map.get(&new_key);

        assert_eq!(result_1, Some(&Int(10.0)));
        assert_eq!(result_2, Some(&Int(10.0)));
    }

    #[test]
    fn test_not_basic_create_two() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let old_key = String::from("old_key");
        let new_key = String::from("new_key");

        map.insert(&old_key, Line("line".to_string()));

        let _ = create(&new_key, &Line("old_key".to_string()), &mut map);

        let result_1 = map.get(&old_key);
        let result_2 = map.get(&new_key);

        assert_eq!(result_1, Some(&Line("line".to_string())));
        assert_eq!(result_2, Some(&Line("line".to_string())));
    }

    #[test]
    fn test_not_basic_create_three() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let old_key = String::from("old_key");
        let new_key = String::from("new_key");

        map.insert(&old_key, Line("line".to_string()));

        let _ = create(&new_key, &Line("old_key 1".to_string()), &mut map);

        let result_1 = map.get(&old_key);
        let result_2 = map.get(&new_key);

        assert_eq!(result_1, Some(&Line("line".to_string())));
        assert_eq!(result_2, Some(&Line("i".to_string())));
    }

    #[test]
    fn test_not_basic_create_float() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let old_key = String::from("old_key");
        let new_key = String::from("new_key");

        map.insert(&old_key, Line("line".to_string()));

        let _ = create(&new_key, &Line("old_key 1.43".to_string()), &mut map);

        let result_1 = map.get(&old_key);
        let result_2 = map.get(&new_key);

        assert_eq!(result_1, Some(&Line("line".to_string())));
        assert_eq!(result_2, Some(&Line("i".to_string())));
    }

    #[test]
    fn test_not_basic_create_float_two() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let old_key = String::from("old_key");
        let new_key = String::from("new_key");

        map.insert(&old_key, Line("line".to_string()));

        let _ = create(&new_key, &Line("old_key 1.93".to_string()), &mut map);

        let result_1 = map.get(&old_key);
        let result_2 = map.get(&new_key);

        assert_eq!(result_1, Some(&Line("line".to_string())));
        assert_eq!(result_2, Some(&Line("i".to_string())));
    }

    #[test]
    fn test_not_basic_create_four() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let old_key = String::from("old_key");
        let old_key_2 = String::from("old_key_2");
        let new_key = String::from("new_key");

        map.insert(&old_key, Line("line".to_string()));
        map.insert(&old_key_2, Int(1.0));

        let _ = create(&new_key, &Line("old_key old_key_2".to_string()), &mut map);

        let result_1 = map.get(&old_key);
        let result_2 = map.get(&new_key);

        assert_eq!(result_1, Some(&Line("line".to_string())));
        assert_eq!(result_2, Some(&Line("i".to_string())));
    }

    #[test]
    fn test_not_stand_create_one() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let old_key = String::from("old_key");
        let new_key = String::from("new_key");

        map.insert(&old_key, Int(1.0));

        let _ = create(&new_key, &Line("old_key 2".to_string()), &mut map);

        let result_1 = map.get(&old_key);
        let result_2 = map.get(&new_key);

        assert_eq!(result_1, Some(&Int(1.0)));
        assert_eq!(result_2, Some(&Line("old_key 2".to_string())));
    }

    #[test]
    fn test_not_stand_create_two() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let old_key = String::from("old_key");
        let old_key_2 = String::from("old_key_2");
        let new_key = String::from("new_key");

        map.insert(&old_key, Line("line".to_string()));
        map.insert(&old_key_2, Int(10.0));

        let _ = create(&new_key, &Line("old_key old_key_2".to_string()), &mut map);

        let result_1 = map.get(&old_key);
        let result_2 = map.get(&new_key);

        assert_eq!(result_1, Some(&Line("line".to_string())));
        assert_eq!(result_2, Some(&Line("".to_string())));
    }
}
