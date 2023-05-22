use super::*;
pub fn mono(node: Node) -> String {
    let node = node.into_child();
    match node.as_rule() {
        Rule::numeric => node.as_str().to_owned(),
        Rule::raw => node.as_str().to_owned(),
        Rule::font => {
            let font = node.into_child();
            match font.as_rule() {
                Rule::font_greek => font_greek(font),
                Rule::font_numcls => font_number_class(font),
                rule => unreachable!("Expect only Fonts: {:?}", rule),
            }
        }
        Rule::operator => match node.into_child().as_rule() {
            // Arithmetic
            Rule::op_arith_add => "+",
            Rule::op_arith_sub => "-",
            Rule::op_arith_mul => "*",
            Rule::op_arith_div => "\\div",
            Rule::op_arith_mul_dot => "\\cdot",
            Rule::op_arith_mul_cross => "\\times",
            // Equality
            Rule::op_eq_eq => "=",
            Rule::op_eq_gt => "\\gt",
            Rule::op_eq_lt => "\\lt",
            Rule::op_eq_ge => "\\ge",
            Rule::op_eq_le => "\\le",
            Rule::op_eq_equiv => "\\equiv",
            Rule::op_eq_colon => "\\coloneqq",
            Rule::op_eq_bar => "\\vDash",
            // Inequality
            Rule::op_ne_eq => "\\ne",
            Rule::op_ne_gt => "\\ngtr",
            Rule::op_ne_lt => "\\nltr",
            Rule::op_ne_ge => "\\ngeq",
            Rule::op_ne_le => "\\nleq",
            Rule::op_ne_equiv => "\\not\\equiv",
            Rule::op_ne_bar => "\\nvDash",
            rule => unreachable!("Expect only Operators: {:?}", rule),
        }
        .to_owned(),
        Rule::dot => match node.into_child().as_rule() {
            Rule::dot_centre => "\\cdot",
            Rule::dot_triple => "\\ldots",
            Rule::dot_compose => "\\circ",
            rule => unreachable!("Expect only Dots: {:?}", rule),
        }
        .to_owned(),
        Rule::arrow => match node.into_child().as_rule() {
            Rule::arr_up => "\\uparrow",
            Rule::arr_down => "\\downarrow",
            Rule::arr_left => "\\leftarrow",
            Rule::arr_right => "\\rightarrow",
            Rule::arr_double => "\\leftrightarrow",
            Rule::arr_thick_left => "\\Leftarrow",
            Rule::arr_thick_right => "\\Rightarrow",
            Rule::arr_thick_double => "\\Leftrightarrow",
            Rule::arr_long_left => "leftlongarrow",
            Rule::arr_long_right => "rightlongarrow",
            Rule::arr_long_double => "leftrightlongarrow",
            Rule::arr_thick_long_left => "Leftlongarrow",
            Rule::arr_thick_long_right => "Rightlongarrow",
            Rule::arr_thick_long_double => "Leftrightlongarrow",
            Rule::arr_ne => "\\nearrow",
            Rule::arr_se => "\\searrow",
            Rule::arr_nw => "\\nwarrow",
            Rule::arr_sw => "\\swarrow",
            rule => unreachable!("Expect only Arrows: {:?}", rule),
        }
        .to_owned(),
        Rule::logic => match node.into_child().as_rule() {
            Rule::logic_or => "\\lor",
            Rule::logic_and => "\\land",
            Rule::logic_not => "\\lnot",
            rule => unreachable!("Expect only Logical Operator: {:?}", rule),
        }
        .to_owned(),
        Rule::sets => {
            let set_op = node.into_child();
            match set_op.as_rule() {
                Rule::set_union => "\\cup".into(),
                Rule::set_intersection => "\\cap".into(),
                Rule::set_complement => set_complement(set_op),
                Rule::set_empty => "\\emptyset".into(),
                Rule::set_in => "\\in".into(),
                Rule::set_subset => "\\subset".into(),
                Rule::set_subseteq => "\\subseteq".into(),
                Rule::set_subsetne => "\\nsubseteq".into(),
                rule => unreachable!("Expect only Logical Operator: {:?}", rule),
            }
        }
        Rule::collection => {
            let collection = node.into_child();
            match collection.as_rule() {
                Rule::col_set => format!(
                    "\\set{{ {} }}",
                    collection.into_inner().map(mono).join(", ")
                ),
                Rule::col_tup => {
                    let inner = collection.into_inner().collect_vec();
                    let delim = format!(
                        " {} ",
                        if inner.len() > 1 {
                            let delim = inner[1].clone().into_inner().next();
                            delim.map_or(" ", |delim| delim.as_str())
                        } else {
                            "".into()
                        }
                    );
                    format!(
                        "( {} )",
                        inner
                            .into_iter()
                            .filter_map(|m| match m.as_rule() {
                                Rule::collection_delim => None,
                                _ => Some(mono(m)),
                            })
                            .join(&delim)
                    )
                }
                Rule::col_vec => {
                    let inner = collection.into_inner().collect_vec();
                    let (ext, monos) = inner
                        .split_last()
                        .expect("Vector must have an extension even if it is empty");
                    let delim = format!(
                        " {} ",
                        match ext.clone().into_child().as_rule() {
                            Rule::empty => "&",
                            Rule::col_vec_transpose => "\\\\",
                            rule => unreachable!("Expect only Vector Extensions: {:?}", rule),
                        }
                    );
                    format!(
                        "\\begin{{bmatrix}} {} \\end{{bmatrix}}",
                        monos.into_iter().map(|m| mono(m.clone())).join(&delim)
                    )
                }
                rule => unreachable!("Expect only Collections: {:?}", rule),
            }
        }
        rule => todo!("MONO ROOT: {:?}", rule),
    }
}

fn font_greek(char: Node) -> String {
    format!("\\{}", char.as_str())
}
fn font_number_class(char: Node) -> String {
    format!("\\mathbb{{{}}}", char.as_str())
}
fn set_complement(node: Node) -> String {
    format!("\\overline{{{}}}", mono(node.into_child()))
}
