use crate::ast;
use crate::parse;

use std::error;

macro_rules! ok {
    ($id:ident, $fn:ident, $arg:expr, $cmp:expr) => {
        #[test]
        fn $id() -> Result<(), Box<dyn error::Error>> {
            assert_eq!(parse::$fn::new().parse($arg)?, $cmp);
            Ok(())
        }
    };
}

macro_rules! err {
    ($id:ident, $fn:ident, $arg:expr) => {
        #[test]
        fn $id() {
            assert!(parse::$fn::new().parse($arg).is_err());
        }
    };
}

// PIM syntax 1
err!(ident_null, IdentParser, "");
err!(ident_space, IdentParser, " ");
err!(ident_asterisk, IdentParser, "*");

ok!(ident, IdentParser, "azAZ09", ast::Ident("azAZ09"));
ok!(sp_ident, IdentParser, " abc", ast::Ident("abc"));
ok!(ident_sp, IdentParser, "abc ", ast::Ident("abc"));
ok!(sp_ident_sp, IdentParser, " abc ", ast::Ident("abc"));
ok!(tab_ident_tab, IdentParser, "\tabc\t", ast::Ident("abc"));

// 03 Integer
err!(integer_null, NumberParser, "");
err!(integer_a, NumberParser, "A");
err!(integer_8_oct, NumberParser, "8B");
err!(integer_f_hex, NumberParser, "FH");
err!(integer_f_hex_low, NumberParser, "fH");
err!(integer_g_hex, NumberParser, "GH");

ok!(integer_1, NumberParser, "1", ast::Number::Dec("1"));
ok!(integer_12, NumberParser, "12", ast::Number::Dec("12"));
ok!(integer_7_oct, NumberParser, "7B", ast::Number::Oct("7B"));
ok!(integer_9f_hex, NumberParser, "9FH", ast::Number::Hex("9FH"));

// 05 Real
err!(real_null, NumberParser, "");
err!(real_dot, NumberParser, ".");
err!(real_dot_1, NumberParser, ".1");
err!(real_e_1, NumberParser, "E1");
err!(real_12_dot_3_e, NumberParser, "12.3E");

ok!(real_1_dot, NumberParser, "1.", ast::Number::Real("1."));
ok!(real_12_dot, NumberParser, "12.", ast::Number::Real("12."));
ok!(real_12_dot_3, NumberParser, "12.3", ast::Number::Real("12.3"));
ok!(real_12_dot_3_e_1, NumberParser, "12.3E1", ast::Number::Real("12.3E1"));
ok!(real_12_dot_3_e_add_1, NumberParser, "12.3E+1", ast::Number::Real("12.3E+1"));
ok!(real_12_dot_3_e_sub_1, NumberParser, "12.3E-1", ast::Number::Real("12.3E-1"));

// 10 String
err!(string_null, StringParser, "");
err!(string_q, StringParser, "'");
err!(string_qq, StringParser, "\"");
err!(string_u_q, StringParser, "u'");
err!(string_u_qq, StringParser, "u\"");
err!(string_q_u, StringParser, "'u");
err!(string_qq_u, StringParser, "\"u");

ok!(string_q_u_q, StringParser, "'u'", ast::QStr("'u'"));
ok!(string_qq_u_qq, StringParser, "\"u\"", ast::QStr("\"u\""));
ok!(string_q_uv_q, StringParser, "'uv'", ast::QStr("'uv'"));
ok!(string_qq_uv_qq, StringParser, "\"uv\"", ast::QStr("\"uv\""));
ok!(string_q_qq_q, StringParser, "'\"'", ast::QStr("'\"'"));
ok!(string_qq_q_qq, StringParser, "\"'\"", ast::QStr("\"'\""));

// 11 Qualident
err!(qualident_null, QualidentParser, "");
err!(qualident_one_two, QualidentParser, "one two");

ok!(qualident_one, QualidentParser, "one", ast::Qualident(vec![ast::Ident("one")]));
ok!(
    qualident_one_dot_two,
    QualidentParser,
    "one.two",
    ast::Qualident(vec![ast::Ident("one"), ast::Ident("two")])
);

// 14 Relation
ok!(relation_eq, RelationParser, "=", ast::Relation::EQ);
ok!(relation_hh, RelationParser, "#", ast::Relation::HASH);
ok!(relation_ne, RelationParser, "<>", ast::Relation::NE);
ok!(relation_lt, RelationParser, "<", ast::Relation::LT);
ok!(relation_lte, RelationParser, "<=", ast::Relation::LTE);
ok!(relation_gt, RelationParser, ">", ast::Relation::GT);
ok!(relation_gte, RelationParser, ">=", ast::Relation::GTE);
ok!(relation_in, RelationParser, "IN", ast::Relation::IN);

// 16 AddOp
err!(addop_or_low, AddOperatorParser, "or");

ok!(addop_add, AddOperatorParser, "+", ast::AddOperator::ADD);
ok!(addop_sub, AddOperatorParser, "-", ast::AddOperator::SUB);
ok!(addop_or, AddOperatorParser, "OR", ast::AddOperator::OR);

// 18 MulOp
err!(mulop_div_low, MulOperatorParser, "div");
err!(mulop_mod_low, MulOperatorParser, "mod");
err!(mulop_and_low, MulOperatorParser, "and");

ok!(mulop_mul, MulOperatorParser, "*", ast::MulOperator::MUL);
ok!(mulop_div_char, MulOperatorParser, "/", ast::MulOperator::DIV);
ok!(mulop_div, MulOperatorParser, "DIV", ast::MulOperator::DIV);
ok!(mulop_mod, MulOperatorParser, "MOD", ast::MulOperator::MOD);
ok!(mulop_and, MulOperatorParser, "AND", ast::MulOperator::AND);
ok!(mulop_and_char, MulOperatorParser, "&", ast::MulOperator::AND);
