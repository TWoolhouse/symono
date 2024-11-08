mod parse {
    use symono::{Error, Transpile};

    pub fn mono_r(syo: &str) -> Result<String, Error> {
        symono::parse(syo).map(|sequence| sequence.latex())
    }

    pub fn mono(syo: &str) -> String {
        mono_r(syo).unwrap()
    }
}

/// Any single element that can be written on it's own
mod single {
    use super::parse::*;

    #[test]
    fn dots() {
        assert_eq!(mono("."), r"\cdot", "centre");
        assert_eq!(mono("..."), r"\ldots", "triple dot");
        assert_eq!(mono(".o"), r"\circ", "composition");
    }

    #[test]
    fn operator_arithmetic() {
        assert_eq!(mono("+"), "+", "addition");
        assert_eq!(mono("-"), "-", "subtraction");
        assert_eq!(mono("*"), "*", "multiplication");
        assert_eq!(mono("*."), r"\cdot", "dot multiplication");
        assert_eq!(mono("*x"), r"\times", "cross multiplication");
        assert_eq!(mono("./."), r"\div", "division");
        assert_eq!(mono("/"), r"/", "raw division");
        assert_eq!(mono("1 / 2"), r"1 / 2", "raw division");
        assert_eq!(mono("1 / 2 / 3 /"), r"1 / 2 / 3 /", "raw division");
    }

    #[test]
    fn operator_equality() {
        assert_eq!(mono("="), "=", "equality");
        assert_eq!(mono(">"), r"\gt", "greater than");
        assert_eq!(mono("<"), r"\lt", "less than");
        assert_eq!(mono(">="), r"\ge", "greater than or equal to");
        assert_eq!(mono("<="), r"\le", "less than or equal to");
        assert_eq!(mono("=="), r"\equiv", "equivalence");
        assert_eq!(mono("==="), r"\equiv", "long equivalence");
        assert_eq!(mono(":="), r"\coloneqq", "colon equals");
        assert_eq!(mono("|="), r"\vDash", "bar equals");
    }

    #[test]
    fn operator_inequality() {
        assert_eq!(mono("!="), r"\ne", "inequality");
        assert_eq!(mono("!>"), r"\ngtr", "not greater than");
        assert_eq!(mono("!<"), r"\nltr", "not less than");
        assert_eq!(mono("!>="), r"\ngeq", "not greater than or equal to");
        assert_eq!(mono("!<="), r"\nleq", "not less than or equal to");
        assert_eq!(mono(">!="), r"\ngeq", "not greater than or equal to");
        assert_eq!(mono("<!="), r"\nleq", "not less than or equal to");
        assert_eq!(mono("!=="), r"\not\equiv", "inequivalent");
        assert_eq!(mono("!==="), r"\not\equiv", "long inequivalent");
        assert_eq!(mono("|!="), r"\nvDash", "bar inequality");
    }

    #[test]
    fn operator_arrows() {
        assert_eq!(mono("-^"), r"\uparrow", "up");
        assert_eq!(mono("-v"), r"\downarrow", "down");
        assert_eq!(mono("-V"), r"\downarrow", "down");

        assert_eq!(mono("<-"), r"\leftarrow", "left");
        assert_eq!(mono("->"), r"\rightarrow", "right");
        assert_eq!(mono("<->"), r"\leftrightarrow", "double");

        // FIXME: See collisions: arith_le_vs_arrow_left_thick
        // assert_eq!(mono("<="), r"\Leftarrow", "left thick");
        assert_eq!(mono("=>"), r"\Rightarrow", "right thick");
        assert_eq!(mono("<=>"), r"\Leftrightarrow", "double thick");

        assert_eq!(mono("<--"), r"\leftlongarrow", "left long");
        assert_eq!(mono("-->"), r"\rightlongarrow", "right long");
        assert_eq!(mono("<-->"), r"\leftrightlongarrow", "double long");

        assert_eq!(mono("<=="), r"\Leftlongarrow", "left long thick");
        assert_eq!(mono("==>"), r"\Rightlongarrow", "right long thick");
        assert_eq!(mono("<==>"), r"\Leftrightlongarrow", "double long thick");

        assert_eq!(mono("-/>"), r"\nearrow", "diagonal north east");
        assert_eq!(mono(r"-\>"), r"\searrow", "diagonal south east");
        assert_eq!(mono(r"<\-"), r"\nwarrow", "diagonal north west");
        assert_eq!(mono("</-"), r"\swarrow", "diagonal south west");
    }

    #[test]
    fn logic() {
        assert_eq!(mono("&&"), r"\land", "and");
        assert_eq!(mono("&|"), r"\lor", "or");
        assert_eq!(mono("&!"), r"\lnot", "not");

        assert_eq!(mono("||"), r"\lor", "or");
        assert_eq!(mono("¬"), r"\lnot", "not");
    }

    #[test]
    fn set_ops() {
        assert_eq!(mono("{|"), r"\cup", "union");
        assert_eq!(mono("{&"), r"\cap", "intersection");

        assert!(mono_r("{!").is_err(), "complement requires a operand");
        assert_eq!(mono("{!ABC"), r"\overline{ABC}", "complement w/ space");
        assert_eq!(mono("{! ABC"), r"\overline{ABC}", "complement wo/ space");

        assert_eq!(mono("{-"), r"\setminus", "minus");
        assert_eq!(mono("{0"), r"\emptyset", "empty set");

        assert_eq!(mono("{i"), r"\in", "element in set");
        assert_eq!(mono("{c"), r"\in", "element in set");
        assert_eq!(mono("in"), r"\in", "element in set");

        assert_eq!(mono("{<"), r"\subset", "proper subset");
        assert_eq!(mono("{<="), r"\subseteq", "weak subset");
        assert_eq!(mono("{<!="), r"\nsubseteq", "subset not equal to");
        assert_eq!(mono("{!<="), r"\nsubseteq", "weak not equal to");
    }
}

mod modifier {
    use super::parse::*;
    #[test]
    fn superscript() {
        assert_eq!(mono("^lol"), "^{lol}");
        assert_eq!(mono("x^lol"), "x ^{lol}");
    }

    #[test]
    fn subscript() {
        assert_eq!(mono("_lol"), "_{lol}");
        assert_eq!(mono("x_lol"), "x _{lol}");
    }
}

mod collection {
    use super::parse::*;

    #[test]
    fn group() {
        assert_eq!(mono("((1))"), mono("1"));
        assert_eq!(mono("((1 <= 3))"), mono("1 <= 3"));
        assert_eq!(mono("(( 1 <= 3 ))"), mono("1 <= 3"));
    }

    #[test]
    fn generic() {
        assert_eq!(mono("(1 @alpha 3)"), r"( 1 \alpha 3 )");
        assert_eq!(mono("(1, @alpha, 3)"), r"( 1 , \alpha , 3 )");
        assert_eq!(mono("(1 , @alpha , 3)"), r"( 1 , \alpha , 3 )");
    }

    #[test]
    fn set() {
        assert_eq!(mono("{1, @alpha, 3}"), r"\set{ 1 , \alpha , 3 }");
    }

    #[test]
    fn tuple() {
        assert_eq!(mono("(1, @alpha, 3)"), r"( 1 , \alpha , 3 )");
    }

    #[test]
    fn vector() {
        assert_eq!(
            mono("[1, @alpha, 3]"),
            r"\begin{bmatrix} 1 ,& \alpha ,& 3 \end{bmatrix}"
        );
        assert_eq!(
            mono("[1 @alpha 3]"),
            r"\begin{bmatrix} 1 & \alpha & 3 \end{bmatrix}"
        );
        assert_eq!(
            mono("[1 @alpha 3]T"),
            r"\begin{bmatrix} 1 \\ \alpha \\ 3 \end{bmatrix}"
        );
        assert_eq!(
            mono("[1, @alpha, 3]T"),
            r"\begin{bmatrix} 1 \\ \alpha \\ 3 \end{bmatrix}"
        );
    }
}

mod command {
    use super::parse::*;

    #[test]
    fn fraction() {
        assert_eq!(mono("/1/2/"), r"\frac{1}{2}");
        assert_eq!(mono("/1/2/3/"), r"\cfrac{1}{ \cfrac{2}{ 3 } }");
        assert_eq!(mono("/1/((/2/3/))/"), r"\frac{1}{\frac{2}{3}}");
        assert_eq!(mono("/1 + 1/2/"), r"\frac{1 + 1}{2}");
    }

    #[test]
    fn sqrt() {
        assert_eq!(mono("sqrt alpha"), r"\sqrt{\alpha}");
        assert_eq!(mono("sqrt 1"), r"\sqrt{1}");
        assert_eq!(mono("sqrt 1 + 1"), r"\sqrt{1} + 1");

        assert_eq!(mono("_/ 1"), r"\sqrt{1}");
        assert_eq!(mono("_/alpha"), r"\sqrt{\alpha}");
        assert_eq!(mono("_//1/2/"), r"\sqrt{\frac{1}{2}}");
        assert_eq!(
            mono("_/((4 . /alpha + 3/2/))"),
            r"\sqrt{4 \cdot \frac{\alpha + 3}{2}}"
        );
        assert_eq!(mono("_alpha/beta"), r"\sqrt[\alpha]{\beta}");
        assert_eq!(mono("_alpha/ beta"), r"\sqrt[\alpha]{\beta}");
    }

    #[test]
    fn cases() {
        assert_eq!(
            mono(
                r#"Gamma(sigma) = {
				case! f(x) \t x > 0
				case! g(x) \t "otherwise"
			}"#
            ),
            r"\Gamma ( \sigma ) = \begin{cases} f ( x ) & x \gt 0 \\ g ( x ) & \text{otherwise} \end{cases}"
        )
    }

    #[test]
    fn sum() {
        assert_eq!(mono("sum_((x=1))^10"), r"\sum _{x = 1} ^{10}");
    }
    #[test]
    fn prod() {
        assert_eq!(mono("prod_((x=1))^10"), r"\prod _{x = 1} ^{10}");
    }
}

mod font {
    use super::parse::*;

    #[test]
    fn greek() {
        let mapping = [
            // Uppercase
            ("Alpha", "A"),
            ("Beta", "B"),
            ("Gamma", r"\Gamma"),
            ("Delta", r"\Delta"),
            ("Epsilon", "E"),
            ("Zeta", "Z"),
            ("Eta", "E"),
            ("Theta", r"\Theta"),
            ("Iota", "I"),
            ("Kappa", "K"),
            ("Lambda", r"\Lambda"),
            ("Mu", "M"),
            ("Nu", "N"),
            ("Omicron", "O"),
            ("Pi", r"\Pi"),
            ("Rho", "R"),
            ("Sigma", r"\Sigma"),
            ("Tau", "T"),
            ("Upsilon", r"\Upsilon"),
            ("Phi", r"\Phi"),
            ("Chi", "X"),
            ("Psi", r"\Psi"),
            ("Omega", r"\Omega"),
            // Lowercase
            ("alpha", r"\alpha"),
            ("beta", r"\beta"),
            ("gamma", r"\gamma"),
            ("delta", r"\delta"),
            ("epsilon", r"\epsilon"),
            ("zeta", r"\zeta"),
            ("eta", r"\eta"),
            ("theta", r"\theta"),
            ("iota", r"\iota"),
            ("kappa", r"\kappa"),
            ("lambda", r"\lambda"),
            ("mu", r"\mu"),
            ("nu", r"\nu"),
            ("omicron", r"\omicron"),
            ("pi", r"\pi"),
            ("rho", r"\rho"),
            ("sigma", r"\sigma"),
            ("tau", r"\tau"),
            ("upsilon", r"\upsilon"),
            ("phi", r"\phi"),
            ("chi", r"\chi"),
            ("psi", r"\psi"),
            ("omega", r"\omega"),
            // variants
            ("vepsilon", r"\varepsilon"),
            ("vrho", r"\varrho"),
            ("vtheta", r"\vartheta"),
            ("vphi", r"\varphi"),
        ];
        for delim in ["", "@", "g@", "G@"] {
            let g = |name| format!("{}{}", delim, name);

            for (sym, ltx) in &mapping {
                assert_eq!(mono(&g(*sym)), *ltx, "{}", *sym);
            }
        }
    }

    #[test]
    fn number_class() {
        for delim in ["|", "n@", "N@"] {
            let n = |name| format!("{}{}", delim, name);
            for ascii in ('A')..=('Z') {
                assert_eq!(mono(&n(ascii)), format!(r"\mathbb{{{}}}", ascii));
                let ascii = ascii.to_ascii_lowercase();
                if let Ok(res) = mono_r(&n(ascii)) {
                    assert_ne!(res, format!(r"\mathbb{{{}}}", ascii), "{}", ascii);
                }
            }
        }
    }

    #[test]
    fn curly() {
        for delim in ["@", "c@", "C@"] {
            let n = |name| format!("{}{}", delim, name);
            for ascii in ('A')..=('Z') {
                assert_eq!(mono(&n(ascii)), format!(r"\mathcal{{{}}}", ascii));
                let ascii = ascii.to_ascii_lowercase();
                if let Ok(res) = mono_r(&n(ascii)) {
                    assert_ne!(res, format!(r"\mathcal{{{}}}", ascii), "{}", ascii);
                }
            }
        }
    }
}

mod literals {
    use std::iter;

    use itertools::Itertools;

    use super::parse::*;

    #[test]
    fn alphabetic() {
        let mut alphabet = 'a'..='z';
        for ascii in alphabet.clone() {
            for character in [ascii, ascii.to_ascii_uppercase()] {
                for i in 1..=10 {
                    let string = iter::repeat(character).take(i).join("");
                    assert_eq!(mono(&string), string);
                }
            }
        }
        let string = alphabet.join("");
        assert_eq!(mono(&string), string);
    }

    #[test]
    fn numeric() {
        let numerals = ["0", "1", "123", "100", "001", "999", "1234567890"];

        for &pre in &numerals {
            for &post in &numerals {
                let check = |number| {
                    assert_eq!(mono(number), number);
                    assert_eq!(mono(&format!("-{}", number)), format!("-{}", number));
                    assert_eq!(mono(&format!("+{}", number)), format!("+{}", number));
                };
                check(pre);
                check(&format!("{}.{}", pre, post));
            }
            {
                let number = pre;
                assert_ne!(mono(&format!(".{}", number)), format!(".{}", number));
                assert_ne!(mono(&format!("{}.", number)), format!("{}.", number));
            }
        }
    }

    #[test]
    fn punctuation() {
        let symbols = [",", "!", ":", "|", "?", "%", "&", "$", "£"];
        for symbol in symbols {
            assert_eq!(mono(symbol), symbol);
        }
    }
}

mod collisions {
    use super::parse::*;

    #[test]
    #[should_panic]
    /// The same symono syntax has been given to two symbols
    fn arith_le_vs_arrow_left_thick() {
        assert_eq!(mono("<="), r"\Leftarrow");
        assert_eq!(mono("<="), r"\le");
    }

    #[test]
    fn sqrt_fraction_subscript() {
        assert_eq!(mono("_ /1/2/"), r"_{\frac{1}{2}}");
        assert_eq!(mono("_/1/2/"), r"\sqrt{1} / 2 /");
        assert_eq!(mono("_//1/2/"), r"\sqrt{\frac{1}{2}}");
    }
}
