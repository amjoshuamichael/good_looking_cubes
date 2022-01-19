use std::collections::HashMap;

pub fn parse_pec(pec_file: &String) -> HashMap<u8, u32> {
    let mut output_map = HashMap::new();

    for statement in pec_file.split("\n") {
        if statement.len() == 0 { continue; }

        let statement_args = statement.split(">").collect::<Vec<&str>>();

        let color_index = statement_args[0].parse::<u8>().unwrap();

        let color_info_args = statement_args[1];
        let color_info_type = color_info_args.chars().nth(0).unwrap();
        let color_info_strength = &color_info_args[1..].parse::<u32>().unwrap();

        let mut color_info = 0;
        match color_info_type {
            'e' => color_info = color_info_strength << 20,
            'g' => color_info = color_info_strength << 18,
            't' => color_info = color_info_strength << 16,
            _ => {}
        }

        output_map.insert(color_index, color_info);
    }

    output_map
}