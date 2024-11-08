use super::*;
pub fn mono(node: Node) -> String {
    let node = node.into_child();
    match node.as_rule() {
        Rule::tab => "&".to_owned(),
        Rule::numeric => node.as_str().to_owned(),
        Rule::alphabetic => node.as_str().to_owned(),
        Rule::punctuation => node.as_str().to_owned(),
        Rule::font => {
            let font = node.into_child();
            match font.as_rule() {
                Rule::font_greek => font_greek(font),
                Rule::font_numcls => format!(r"\mathbb{{{}}}", font.as_str()),
                Rule::font_curly => format!(r"\mathcal{{{}}}", font.as_str()),
                rule => unreachable!("Expect only Fonts: {:?}", rule),
            }
        }
        Rule::script => {
            let script = node.into_child();
            let cmd = match script.as_rule() {
                Rule::script_super => "^",
                Rule::script_sub => "_",
                rule => unreachable!("Expect only Scripts: {:?}", rule),
            };
            format!("{}{{{}}}", cmd, mono(script.into_child()))
        }
        Rule::operator => {
            let operator = node.into_child();
            match operator.as_rule() {
                // Arithmetic
                Rule::op_arith_add => "+".into(),
                Rule::op_arith_sub => "-".into(),
                Rule::op_arith_mul => "*".into(),
                Rule::op_arith_div_raw => "/".into(),
                Rule::op_arith_div => r"\div".into(),
                Rule::op_arith_mul_dot => r"\cdot".into(),
                Rule::op_arith_mul_cross => r"\times".into(),
                Rule::op_arith_frac => fraction(operator),
                // Equality
                Rule::op_eq_eq => "=".into(),
                Rule::op_eq_gt => r"\gt".into(),
                Rule::op_eq_lt => r"\lt".into(),
                Rule::op_eq_ge => r"\ge".into(),
                Rule::op_eq_le => r"\le".into(),
                Rule::op_eq_equiv => r"\equiv".into(),
                Rule::op_eq_colon => r"\coloneqq".into(),
                Rule::op_eq_bar => r"\vDash".into(),
                // Inequality
                Rule::op_ne_eq => r"\ne".into(),
                Rule::op_ne_gt => r"\ngtr".into(),
                Rule::op_ne_lt => r"\nltr".into(),
                Rule::op_ne_ge => r"\ngeq".into(),
                Rule::op_ne_le => r"\nleq".into(),
                Rule::op_ne_equiv => r"\not\equiv".into(),
                Rule::op_ne_bar => r"\nvDash".into(),
                rule => unreachable!("Expect only Operators: {:?}", rule),
            }
        }
        Rule::dot => match node.into_child().as_rule() {
            Rule::dot_centre => r"\cdot",
            Rule::dot_triple => r"\ldots",
            Rule::dot_compose => r"\circ",
            rule => unreachable!("Expect only Dots: {:?}", rule),
        }
        .to_owned(),
        Rule::arrow => match node.into_child().as_rule() {
            Rule::arr_up => r"\uparrow",
            Rule::arr_down => r"\downarrow",
            Rule::arr_left => r"\leftarrow",
            Rule::arr_right => r"\rightarrow",
            Rule::arr_double => r"\leftrightarrow",
            Rule::arr_thick_left => r"\Leftarrow",
            Rule::arr_thick_right => r"\Rightarrow",
            Rule::arr_thick_double => r"\Leftrightarrow",
            Rule::arr_long_left => r"\leftlongarrow",
            Rule::arr_long_right => r"\rightlongarrow",
            Rule::arr_long_double => r"\leftrightlongarrow",
            Rule::arr_thick_long_left => r"\Leftlongarrow",
            Rule::arr_thick_long_right => r"\Rightlongarrow",
            Rule::arr_thick_long_double => r"\Leftrightlongarrow",
            Rule::arr_ne => r"\nearrow",
            Rule::arr_se => r"\searrow",
            Rule::arr_nw => r"\nwarrow",
            Rule::arr_sw => r"\swarrow",
            rule => unreachable!("Expect only Arrows: {:?}", rule),
        }
        .to_owned(),
        Rule::logic => match node.into_child().as_rule() {
            Rule::logic_or => r"\lor",
            Rule::logic_and => r"\land",
            Rule::logic_not => r"\lnot",
            rule => unreachable!("Expect only Logical Operator: {:?}", rule),
        }
        .to_owned(),
        Rule::sets => {
            let set_op = node.into_child();
            match set_op.as_rule() {
                Rule::set_union => r"\cup".into(),
                Rule::set_intersection => r"\cap".into(),
                Rule::set_complement => set_complement(set_op),
                Rule::set_empty => r"\emptyset".into(),
                Rule::set_in => r"\in".into(),
                Rule::set_subset => r"\subset".into(),
                Rule::set_subseteq => r"\subseteq".into(),
                Rule::set_subsetne => r"\nsubseteq".into(),
                Rule::set_minus => r"\setminus".into(),
                rule => unreachable!("Expect only Logical Operator: {:?}", rule),
            }
        }
        Rule::text => {
            format!(r"\text{{{}}}", node.into_child().as_str())
        }
        Rule::group => {
            let group = node.into_child();
            group.into_inner().latex()
        }
        Rule::collection => {
            let collection = node.into_child();
            match collection.as_rule() {
                Rule::col_set => {
                    let seq = collection_sequence(collection);
                    let (seq, delim) = collection_deliminator(&seq);
                    format!(
                        r"\set{{ {} }}",
                        seq.into_iter().map(|m| mono(m.clone())).join(&delim)
                    )
                }
                Rule::col_tup => {
                    let seq = collection_sequence(collection);
                    let (seq, delim) = collection_deliminator(&seq);
                    format!(
                        "( {} )",
                        seq.into_iter().map(|m| mono(m.clone())).join(&delim)
                    )
                }
                Rule::col_vec => {
                    let seq = collection_sequence(collection);
                    let (seq, ext) = collection_ext(&seq);
                    let (seq, delim) = collection_deliminator(seq);
                    let delim = match ext.clone().into_child().as_rule() {
                        Rule::empty => format!(" {}{} ", delim.trim(), "&"),
                        Rule::col_vec_transpose => r" \\ ".into(),
                        rule => unreachable!("Expect only Vector Extensions: {:?}", rule),
                    };
                    format!(
                        r"\begin{{bmatrix}} {} \end{{bmatrix}}",
                        seq.into_iter().map(|m| mono(m.clone())).join(&delim)
                    )
                }
                rule => unreachable!("Expect only Collections: {:?}", rule),
            }
        }
        Rule::command => {
            let cmd = node.into_child();
            match cmd.as_rule() {
                Rule::cmd_cases => {
                    format!(
                        r"\begin{{cases}} {} \end{{cases}}",
                        cmd.into_inner()
                            .map(|sym| sym.into_inner().latex())
                            .join(r" \\ ")
                    )
                }
                Rule::cmd_sum => r"\sum".into(),
                Rule::cmd_prod => r"\prod".into(),
                Rule::cmd_sqrt => {
                    let operands = cmd.into_inner().map(|s| s.latex()).collect_vec();
                    if operands.len() > 1 {
                        format!(r"\sqrt[{}]{{{}}}", operands[0], operands[1])
                    } else {
                        format!(r"\sqrt{{{}}}", operands[0])
                    }
                }
                rule => unreachable!("Expect only Commands: {:?}", rule),
            }
        }
        rule => todo!("MONO ROOT: {:?}", rule),
    }
}

fn font_greek(char: Node) -> String {
    let s: &str = char.as_str();
    let first = |x: &str| x.chars().next().expect("Greek has at least 1 char").into();
    match s {
        "Alpha" => first(s),
        "Beta" => first(s),
        "Epsilon" => first(s),
        "Zeta" => first(s),
        "Eta" => first(s),
        "Iota" => first(s),
        "Kappa" => first(s),
        "Mu" => first(s),
        "Nu" => first(s),
        "Omicron" => first(s),
        "Rho" => first(s),
        "Tau" => first(s),
        "Chi" => "X".into(),
        "vepsilon" => r"\varepsilon".into(),
        "vrho" => r"\varrho".into(),
        "vtheta" => r"\vartheta".into(),
        "vphi" => r"\varphi".into(),
        _ => {
            format!(r"\{}", s)
        }
    }
}
fn set_complement(node: Node) -> String {
    format!(r"\overline{{{}}}", mono(node.into_child()))
}

fn collection_sequence(node: Node) -> Vec<Node> {
    node.into_inner().collect_vec()
}

fn collection_deliminator<'a>(collection: &'a [Node]) -> (Vec<&'a Node<'a>>, String) {
    if let Some(node) = collection.get(1) {
        let delim = node.clone().as_str();
        let seq = collection
            .iter()
            .take(1)
            .chain(collection.iter().skip(2))
            .collect_vec();
        (
            seq,
            if delim == "" {
                " ".into()
            } else {
                format!(" {} ", delim)
            },
        )
    } else {
        (collection.iter().collect_vec(), " ".into())
    }
}

fn collection_ext<'a>(collection: &'a [Node]) -> (&'a [Node<'a>], &'a Node<'a>) {
    let (ext, col) = collection
        .split_last()
        .expect("Collection requires Extension!");
    (col, ext)
}

fn fraction(node: Node) -> String {
    let operands = node
        .into_inner()
        .map(|s| s.into_inner().latex())
        .collect_vec();
    if operands.len() == 2 {
        format!(r"\frac{{{}}}{{{}}}", operands[0], operands[1])
    } else {
        fraction_interior(&operands)
    }
}

fn fraction_interior(elements: &[String]) -> String {
    debug_assert_ne!(elements.len(), 0);
    if elements.len() == 1 {
        elements[0].clone()
    } else {
        format!(
            r"\cfrac{{{}}}{{ {} }}",
            elements[0],
            fraction_interior(&elements[1..])
        )
    }
}
