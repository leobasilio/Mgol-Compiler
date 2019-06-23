use symbols;
use analyzers::Lexical;
use analyzers::Semantic;
use analyzers::pda::PDA;
use analyzers::pda::ActionMethod;
use analyzers::error::CompilerError;
use analyzers::error::LexicalError;
use analyzers::error::EndOfFileError;

pub struct Syntactic {
    automaton: PDA
}

impl Syntactic {

    pub fn new() -> Self {

        let mut pda = PDA::new(Semantic::new());

        pda.add_rule(0, "P'", "P", None);
        pda.add_rule(1, "P", "inicio V A", None);
        pda.add_rule(2, "V", "varinicio LV", None);
        pda.add_rule(3, "LV", "D LV", None);
        pda.add_rule(4, "LV", "varfim ;", None);
        pda.add_rule(5, "D", "id TIPO ;", Some(Semantic::handle_var_decl));
        pda.add_rule(6, "TIPO", "int", Some(Semantic::handle_type_int));
        pda.add_rule(7, "TIPO", "real", Some(Semantic::handle_type_real));
        pda.add_rule(8, "TIPO", "lit", Some(Semantic::handle_type_lit));
        pda.add_rule(9, "A", "ES A", None);
        pda.add_rule(10, "ES", "leia id ;", Some(Semantic::handle_es_in));
        pda.add_rule(11, "ES", "escreva ARG ;", Some(Semantic::handle_es_out));
        pda.add_rule(12, "ARG", "literal", Some(Semantic::handle_arg_lit));
        pda.add_rule(13, "ARG", "num", Some(Semantic::handle_arg_num));
        pda.add_rule(14, "ARG", "id", Some(Semantic::handle_arg_id));
        pda.add_rule(15, "A", "CMD A", None);
        pda.add_rule(16, "CMD", "id rcb LD ;", Some(Semantic::handle_cmd));
        pda.add_rule(17, "LD", "OPRD opm OPRD", Some(Semantic::handle_ld_opm));
        pda.add_rule(18, "LD", "OPRD", Some(Semantic::handle_ld));
        pda.add_rule(19, "OPRD", "id", Some(Semantic::handle_oprd_id));
        pda.add_rule(20, "OPRD", "num", Some(Semantic::handle_oprd_num));
        pda.add_rule(21, "A", "COND A", None);
        pda.add_rule(22, "COND", "CABEÇALHO CORPO", Some(Semantic::handle_if_end));
        pda.add_rule(23, "CABEÇALHO", "se ( EXP_R ) entao", Some(Semantic::handle_if_begin));
        pda.add_rule(24, "EXP_R", "OPRD opr OPRD", Some(Semantic::handle_expr));
        pda.add_rule(25, "CORPO", "ES CORPO", None);
        pda.add_rule(26, "CORPO", "CMD CORPO", None);
        pda.add_rule(27, "CORPO", "COND CORPO", None);
        pda.add_rule(28, "CORPO", "REP CORPO", None);
        pda.add_rule(29, "CORPO", "fimse", None);
        pda.add_rule(30, "A", "REP A", None);
        pda.add_rule(31, "REP", "CABEÇALHOREP CORPOREP", Some(Semantic::handle_while_end));
        pda.add_rule(32, "CABEÇALHOREP", "enquanto ( EXP_R ) faca", Some(Semantic::handle_while_begin));
        pda.add_rule(33, "CORPOREP", "ES CORPOREP", None);
        pda.add_rule(34, "CORPOREP", "CMD CORPOREP", None);
        pda.add_rule(35, "CORPOREP", "COND CORPOREP", None);
        pda.add_rule(36, "CORPOREP", "REP CORPOREP", None);
        pda.add_rule(37, "CORPOREP", "fimenquanto", None);
        pda.add_rule(38, "A", "fim", None);

        pda.add_follow("P'", &[""]);
        pda.add_follow("P", &[""]);
        pda.add_follow("V", &["fim", "leia", "escreva", "id", "se", "enquanto"]);
        pda.add_follow("LV", &["fim", "leia", "escreva", "id", "se", "enquanto"]);
        pda.add_follow("D", &["varfim", "id"]);
        pda.add_follow("TIPO", &[";"]);
        pda.add_follow("A", &[""]);
        pda.add_follow("ES", &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow("ARG", &[";"]);
        pda.add_follow("CMD", &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow("LD", &[";"]);
        pda.add_follow("OPRD", &["opm", ";", "opr", ")"]);
        pda.add_follow("COND", &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow("CABEÇALHO", &["leia", "escreva", "id", "fimse", "se", "enquanto"]);
        pda.add_follow("EXP_R", &[")"]);
        pda.add_follow("CORPO", &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow("REP", &["leia", "escreva", "id", "fimse", "se", "enquanto", "fim", "fimenquanto"]);
        pda.add_follow("CABEÇALHOREP", &["leia", "escreva", "id", "fimenquanto", "se", "enquanto"]);
        pda.add_follow("CORPOREP", &["leia", "escreva", "id", "fimse", "se", "enquanto", "fim", "fimenquanto"]);

        pda.add_action(0, "inicio", ActionMethod::SHIFT, 2);
        pda.add_action(1, "", ActionMethod::ACCEPT, 0);
        pda.add_action(2, "varinicio", ActionMethod::SHIFT, 23);
        pda.add_action(3, "id", ActionMethod::SHIFT, 11);
        pda.add_action(3, "leia", ActionMethod::SHIFT, 9);
        pda.add_action(3, "escreva", ActionMethod::SHIFT, 10);
        pda.add_action(3, "se", ActionMethod::SHIFT, 14);
        pda.add_action(3, "enquanto", ActionMethod::SHIFT, 15);
        pda.add_action(3, "fim", ActionMethod::SHIFT, 8);
        pda.add_action(4, "id", ActionMethod::SHIFT, 11);
        pda.add_action(4, "leia", ActionMethod::SHIFT, 9);
        pda.add_action(4, "escreva", ActionMethod::SHIFT, 10);
        pda.add_action(4, "se", ActionMethod::SHIFT, 14);
        pda.add_action(4, "enquanto", ActionMethod::SHIFT, 15);
        pda.add_action(4, "fim", ActionMethod::SHIFT, 8);
        pda.add_action(5, "id", ActionMethod::SHIFT, 11);
        pda.add_action(5, "leia", ActionMethod::SHIFT, 9);
        pda.add_action(5, "escreva", ActionMethod::SHIFT, 10);
        pda.add_action(5, "se", ActionMethod::SHIFT, 14);
        pda.add_action(5, "enquanto", ActionMethod::SHIFT, 15);
        pda.add_action(5, "fim", ActionMethod::SHIFT, 8);
        pda.add_action(6, "id", ActionMethod::SHIFT, 11);
        pda.add_action(6, "leia", ActionMethod::SHIFT, 9);
        pda.add_action(6, "escreva", ActionMethod::SHIFT, 10);
        pda.add_action(6, "se", ActionMethod::SHIFT, 14);
        pda.add_action(6, "enquanto", ActionMethod::SHIFT, 15);
        pda.add_action(6, "fim", ActionMethod::SHIFT, 8);
        pda.add_action(7, "id", ActionMethod::SHIFT, 11);
        pda.add_action(7, "leia", ActionMethod::SHIFT, 9);
        pda.add_action(7, "escreva", ActionMethod::SHIFT, 10);
        pda.add_action(7, "se", ActionMethod::SHIFT, 14);
        pda.add_action(7, "enquanto", ActionMethod::SHIFT, 15);
        pda.add_action(7, "fim", ActionMethod::SHIFT, 8);
        pda.add_action(8, "", ActionMethod::REDUCE, 38);
        pda.add_action(9, "id", ActionMethod::SHIFT, 16);
        pda.add_action(10, "id", ActionMethod::SHIFT, 20);
        pda.add_action(10, "literal", ActionMethod::SHIFT, 18);
        pda.add_action(10, "num", ActionMethod::SHIFT, 19);
        pda.add_action(11, "rcb", ActionMethod::SHIFT, 26);
        pda.add_action(12, "id", ActionMethod::SHIFT, 11);
        pda.add_action(12, "leia", ActionMethod::SHIFT, 9);
        pda.add_action(12, "escreva", ActionMethod::SHIFT, 10);
        pda.add_action(12, "se", ActionMethod::SHIFT, 14);
        pda.add_action(12, "fimse", ActionMethod::SHIFT, 43);
        pda.add_action(12, "enquanto", ActionMethod::SHIFT, 15);
        pda.add_action(13, "id", ActionMethod::SHIFT, 11);
        pda.add_action(13, "leia", ActionMethod::SHIFT, 9);
        pda.add_action(13, "escreva", ActionMethod::SHIFT, 10);
        pda.add_action(13, "se", ActionMethod::SHIFT, 14);
        pda.add_action(13, "enquanto", ActionMethod::SHIFT, 15);
        pda.add_action(13, "fimenquanto", ActionMethod::SHIFT, 53);
        pda.add_action(14, "(", ActionMethod::SHIFT, 54);
        pda.add_action(15, "(", ActionMethod::SHIFT, 61);
        pda.add_action(16, ";", ActionMethod::SHIFT, 17);
        pda.add_action(17, "id", ActionMethod::REDUCE, 10);
        pda.add_action(17, "leia", ActionMethod::REDUCE, 10);
        pda.add_action(17, "escreva", ActionMethod::REDUCE, 10);
        pda.add_action(17, "se", ActionMethod::REDUCE, 10);
        pda.add_action(17, "fimse", ActionMethod::REDUCE, 10);
        pda.add_action(17, "enquanto", ActionMethod::REDUCE, 10);
        pda.add_action(17, "fimenquanto", ActionMethod::REDUCE, 10);
        pda.add_action(17, "fim", ActionMethod::REDUCE, 10);
        pda.add_action(18, ";", ActionMethod::REDUCE, 12);
        pda.add_action(19, ";", ActionMethod::REDUCE, 13);
        pda.add_action(20, ";", ActionMethod::REDUCE, 14);
        pda.add_action(21, ";", ActionMethod::SHIFT, 22);
        pda.add_action(22, "id", ActionMethod::REDUCE, 11);
        pda.add_action(22, "leia", ActionMethod::REDUCE, 11);
        pda.add_action(22, "escreva", ActionMethod::REDUCE, 11);
        pda.add_action(22, "se", ActionMethod::REDUCE, 11);
        pda.add_action(22, "fimse", ActionMethod::REDUCE, 11);
        pda.add_action(22, "enquanto", ActionMethod::REDUCE, 11);
        pda.add_action(22, "fimenquanto", ActionMethod::REDUCE, 11);
        pda.add_action(22, "fim", ActionMethod::REDUCE, 11);
        pda.add_action(23, "varfim", ActionMethod::SHIFT, 25);
        pda.add_action(23, "id", ActionMethod::SHIFT, 72);
        pda.add_action(24, "id", ActionMethod::REDUCE, 2);
        pda.add_action(24, "leia", ActionMethod::REDUCE, 2);
        pda.add_action(24, "escreva", ActionMethod::REDUCE, 2);
        pda.add_action(24, "se", ActionMethod::REDUCE, 2);
        pda.add_action(24, "enquanto", ActionMethod::REDUCE, 2);
        pda.add_action(24, "fim", ActionMethod::REDUCE, 2);
        pda.add_action(25, ";", ActionMethod::SHIFT, 71);
        pda.add_action(26, "id", ActionMethod::SHIFT, 32);
        pda.add_action(26, "num", ActionMethod::SHIFT, 33);
        pda.add_action(27, ";", ActionMethod::SHIFT, 28);
        pda.add_action(28, "id", ActionMethod::REDUCE, 16);
        pda.add_action(28, "leia", ActionMethod::REDUCE, 16);
        pda.add_action(28, "escreva", ActionMethod::REDUCE, 16);
        pda.add_action(28, "se", ActionMethod::REDUCE, 16);
        pda.add_action(28, "fimse", ActionMethod::REDUCE, 16);
        pda.add_action(28, "enquanto", ActionMethod::REDUCE, 16);
        pda.add_action(28, "fimenquanto", ActionMethod::REDUCE, 16);
        pda.add_action(28, "fim", ActionMethod::REDUCE, 16);
        pda.add_action(29, ";", ActionMethod::REDUCE, 18);
        pda.add_action(29, "opm", ActionMethod::SHIFT, 30);
        pda.add_action(30, "id", ActionMethod::SHIFT, 32);
        pda.add_action(30, "num", ActionMethod::SHIFT, 33);
        pda.add_action(31, ";", ActionMethod::REDUCE, 17);
        pda.add_action(32, ";", ActionMethod::REDUCE, 19);
        pda.add_action(32, "opm", ActionMethod::REDUCE, 19);
        pda.add_action(32, ")", ActionMethod::REDUCE, 19);
        pda.add_action(32, "opr", ActionMethod::REDUCE, 19);
        pda.add_action(33, ";", ActionMethod::REDUCE, 20);
        pda.add_action(33, "opm", ActionMethod::REDUCE, 20);
        pda.add_action(33, ")", ActionMethod::REDUCE, 20);
        pda.add_action(33, "opr", ActionMethod::REDUCE, 20);
        pda.add_action(34, "id", ActionMethod::SHIFT, 11);
        pda.add_action(34, "leia", ActionMethod::SHIFT, 9);
        pda.add_action(34, "escreva", ActionMethod::SHIFT, 10);
        pda.add_action(34, "se", ActionMethod::SHIFT, 14);
        pda.add_action(34, "fimse", ActionMethod::SHIFT, 43);
        pda.add_action(34, "enquanto", ActionMethod::SHIFT, 15);
        pda.add_action(35, "id", ActionMethod::SHIFT, 11);
        pda.add_action(35, "leia", ActionMethod::SHIFT, 9);
        pda.add_action(35, "escreva", ActionMethod::SHIFT, 10);
        pda.add_action(35, "se", ActionMethod::SHIFT, 14);
        pda.add_action(35, "fimse", ActionMethod::SHIFT, 43);
        pda.add_action(35, "enquanto", ActionMethod::SHIFT, 15);
        pda.add_action(36, "id", ActionMethod::SHIFT, 11);
        pda.add_action(36, "leia", ActionMethod::SHIFT, 9);
        pda.add_action(36, "escreva", ActionMethod::SHIFT, 10);
        pda.add_action(36, "se", ActionMethod::SHIFT, 14);
        pda.add_action(36, "fimse", ActionMethod::SHIFT, 43);
        pda.add_action(36, "enquanto", ActionMethod::SHIFT, 15);
        pda.add_action(37, "id", ActionMethod::SHIFT, 11);
        pda.add_action(37, "leia", ActionMethod::SHIFT, 9);
        pda.add_action(37, "escreva", ActionMethod::SHIFT, 10);
        pda.add_action(37, "se", ActionMethod::SHIFT, 14);
        pda.add_action(37, "fimse", ActionMethod::SHIFT, 43);
        pda.add_action(37, "enquanto", ActionMethod::SHIFT, 15);
        pda.add_action(38, "id", ActionMethod::REDUCE, 22);
        pda.add_action(38, "leia", ActionMethod::REDUCE, 22);
        pda.add_action(38, "escreva", ActionMethod::REDUCE, 22);
        pda.add_action(38, "se", ActionMethod::REDUCE, 22);
        pda.add_action(38, "fimse", ActionMethod::REDUCE, 22);
        pda.add_action(38, "enquanto", ActionMethod::REDUCE, 22);
        pda.add_action(38, "fimenquanto", ActionMethod::REDUCE, 22);
        pda.add_action(38, "fim", ActionMethod::REDUCE, 22);
        pda.add_action(39, "id", ActionMethod::REDUCE, 25);
        pda.add_action(39, "leia", ActionMethod::REDUCE, 25);
        pda.add_action(39, "escreva", ActionMethod::REDUCE, 25);
        pda.add_action(39, "se", ActionMethod::REDUCE, 25);
        pda.add_action(39, "fimse", ActionMethod::REDUCE, 25);
        pda.add_action(39, "enquanto", ActionMethod::REDUCE, 25);
        pda.add_action(39, "fimenquanto", ActionMethod::REDUCE, 25);
        pda.add_action(39, "fim", ActionMethod::REDUCE, 25);
        pda.add_action(40, "id", ActionMethod::REDUCE, 26);
        pda.add_action(40, "leia", ActionMethod::REDUCE, 26);
        pda.add_action(40, "escreva", ActionMethod::REDUCE, 26);
        pda.add_action(40, "se", ActionMethod::REDUCE, 26);
        pda.add_action(40, "fimse", ActionMethod::REDUCE, 26);
        pda.add_action(40, "enquanto", ActionMethod::REDUCE, 26);
        pda.add_action(40, "fimenquanto", ActionMethod::REDUCE, 26);
        pda.add_action(40, "fim", ActionMethod::REDUCE, 26);
        pda.add_action(41, "id", ActionMethod::REDUCE, 27);
        pda.add_action(41, "leia", ActionMethod::REDUCE, 27);
        pda.add_action(41, "escreva", ActionMethod::REDUCE, 27);
        pda.add_action(41, "se", ActionMethod::REDUCE, 27);
        pda.add_action(41, "fimse", ActionMethod::REDUCE, 27);
        pda.add_action(41, "enquanto", ActionMethod::REDUCE, 27);
        pda.add_action(41, "fimenquanto", ActionMethod::REDUCE, 27);
        pda.add_action(41, "fim", ActionMethod::REDUCE, 27);
        pda.add_action(42, "id", ActionMethod::REDUCE, 28);
        pda.add_action(42, "leia", ActionMethod::REDUCE, 28);
        pda.add_action(42, "escreva", ActionMethod::REDUCE, 28);
        pda.add_action(42, "se", ActionMethod::REDUCE, 28);
        pda.add_action(42, "fimse", ActionMethod::REDUCE, 28);
        pda.add_action(42, "enquanto", ActionMethod::REDUCE, 28);
        pda.add_action(42, "fimenquanto", ActionMethod::REDUCE, 28);
        pda.add_action(42, "fim", ActionMethod::REDUCE, 28);
        pda.add_action(43, "id", ActionMethod::REDUCE, 29);
        pda.add_action(43, "leia", ActionMethod::REDUCE, 29);
        pda.add_action(43, "escreva", ActionMethod::REDUCE, 29);
        pda.add_action(43, "se", ActionMethod::REDUCE, 29);
        pda.add_action(43, "fimse", ActionMethod::REDUCE, 29);
        pda.add_action(43, "enquanto", ActionMethod::REDUCE, 29);
        pda.add_action(43, "fimenquanto", ActionMethod::REDUCE, 29);
        pda.add_action(43, "fim", ActionMethod::REDUCE, 29);
        pda.add_action(44, "id", ActionMethod::SHIFT, 11);
        pda.add_action(44, "leia", ActionMethod::SHIFT, 9);
        pda.add_action(44, "escreva", ActionMethod::SHIFT, 10);
        pda.add_action(44, "se", ActionMethod::SHIFT, 14);
        pda.add_action(44, "enquanto", ActionMethod::SHIFT, 15);
        pda.add_action(44, "fimenquanto", ActionMethod::SHIFT, 53);
        pda.add_action(45, "id", ActionMethod::SHIFT, 11);
        pda.add_action(45, "leia", ActionMethod::SHIFT, 9);
        pda.add_action(45, "escreva", ActionMethod::SHIFT, 10);
        pda.add_action(45, "se", ActionMethod::SHIFT, 14);
        pda.add_action(45, "enquanto", ActionMethod::SHIFT, 15);
        pda.add_action(45, "fimenquanto", ActionMethod::SHIFT, 53);
        pda.add_action(46, "id", ActionMethod::SHIFT, 11);
        pda.add_action(46, "leia", ActionMethod::SHIFT, 9);
        pda.add_action(46, "escreva", ActionMethod::SHIFT, 10);
        pda.add_action(46, "se", ActionMethod::SHIFT, 14);
        pda.add_action(46, "enquanto", ActionMethod::SHIFT, 15);
        pda.add_action(46, "fimenquanto", ActionMethod::SHIFT, 53);
        pda.add_action(47, "id", ActionMethod::SHIFT, 11);
        pda.add_action(47, "leia", ActionMethod::SHIFT, 9);
        pda.add_action(47, "escreva", ActionMethod::SHIFT, 10);
        pda.add_action(47, "se", ActionMethod::SHIFT, 14);
        pda.add_action(47, "enquanto", ActionMethod::SHIFT, 15);
        pda.add_action(47, "fimenquanto", ActionMethod::SHIFT, 53);
        pda.add_action(48, "id", ActionMethod::REDUCE, 31);
        pda.add_action(48, "leia", ActionMethod::REDUCE, 31);
        pda.add_action(48, "escreva", ActionMethod::REDUCE, 31);
        pda.add_action(48, "se", ActionMethod::REDUCE, 31);
        pda.add_action(48, "fimse", ActionMethod::REDUCE, 31);
        pda.add_action(48, "enquanto", ActionMethod::REDUCE, 31);
        pda.add_action(48, "fimenquanto", ActionMethod::REDUCE, 31);
        pda.add_action(48, "fim", ActionMethod::REDUCE, 31);
        pda.add_action(49, "id", ActionMethod::REDUCE, 33);
        pda.add_action(49, "leia", ActionMethod::REDUCE, 33);
        pda.add_action(49, "escreva", ActionMethod::REDUCE, 33);
        pda.add_action(49, "se", ActionMethod::REDUCE, 33);
        pda.add_action(49, "fimse", ActionMethod::REDUCE, 33);
        pda.add_action(49, "enquanto", ActionMethod::REDUCE, 33);
        pda.add_action(49, "fimenquanto", ActionMethod::REDUCE, 33);
        pda.add_action(49, "fim", ActionMethod::REDUCE, 33);
        pda.add_action(50, "id", ActionMethod::REDUCE, 34);
        pda.add_action(50, "leia", ActionMethod::REDUCE, 34);
        pda.add_action(50, "escreva", ActionMethod::REDUCE, 34);
        pda.add_action(50, "se", ActionMethod::REDUCE, 34);
        pda.add_action(50, "fimse", ActionMethod::REDUCE, 34);
        pda.add_action(50, "enquanto", ActionMethod::REDUCE, 34);
        pda.add_action(50, "fimenquanto", ActionMethod::REDUCE, 34);
        pda.add_action(50, "fim", ActionMethod::REDUCE, 34);
        pda.add_action(51, "id", ActionMethod::REDUCE, 35);
        pda.add_action(51, "leia", ActionMethod::REDUCE, 35);
        pda.add_action(51, "escreva", ActionMethod::REDUCE, 35);
        pda.add_action(51, "se", ActionMethod::REDUCE, 35);
        pda.add_action(51, "fimse", ActionMethod::REDUCE, 35);
        pda.add_action(51, "enquanto", ActionMethod::REDUCE, 35);
        pda.add_action(51, "fimenquanto", ActionMethod::REDUCE, 35);
        pda.add_action(51, "fim", ActionMethod::REDUCE, 35);
        pda.add_action(52, "id", ActionMethod::REDUCE, 36);
        pda.add_action(52, "leia", ActionMethod::REDUCE, 36);
        pda.add_action(52, "escreva", ActionMethod::REDUCE, 36);
        pda.add_action(52, "se", ActionMethod::REDUCE, 36);
        pda.add_action(52, "fimse", ActionMethod::REDUCE, 36);
        pda.add_action(52, "enquanto", ActionMethod::REDUCE, 36);
        pda.add_action(52, "fimenquanto", ActionMethod::REDUCE, 36);
        pda.add_action(52, "fim", ActionMethod::REDUCE, 36);
        pda.add_action(53, "id", ActionMethod::REDUCE, 37);
        pda.add_action(53, "leia", ActionMethod::REDUCE, 37);
        pda.add_action(53, "escreva", ActionMethod::REDUCE, 37);
        pda.add_action(53, "se", ActionMethod::REDUCE, 37);
        pda.add_action(53, "fimse", ActionMethod::REDUCE, 37);
        pda.add_action(53, "enquanto", ActionMethod::REDUCE, 37);
        pda.add_action(53, "fimenquanto", ActionMethod::REDUCE, 37);
        pda.add_action(53, "fim", ActionMethod::REDUCE, 37);
        pda.add_action(54, "id", ActionMethod::SHIFT, 32);
        pda.add_action(54, "num", ActionMethod::SHIFT, 33);
        pda.add_action(55, ")", ActionMethod::SHIFT, 56);
        pda.add_action(56, "entao", ActionMethod::SHIFT, 57);
        pda.add_action(57, "id", ActionMethod::REDUCE, 23);
        pda.add_action(57, "leia", ActionMethod::REDUCE, 23);
        pda.add_action(57, "escreva", ActionMethod::REDUCE, 23);
        pda.add_action(57, "se", ActionMethod::REDUCE, 23);
        pda.add_action(57, "fimse", ActionMethod::REDUCE, 23);
        pda.add_action(57, "enquanto", ActionMethod::REDUCE, 23);
        pda.add_action(58, "opr", ActionMethod::SHIFT, 59);
        pda.add_action(59, "id", ActionMethod::SHIFT, 32);
        pda.add_action(59, "num", ActionMethod::SHIFT, 33);
        pda.add_action(60, ")", ActionMethod::REDUCE, 24);
        pda.add_action(61, "id", ActionMethod::SHIFT, 32);
        pda.add_action(61, "num", ActionMethod::SHIFT, 33);
        pda.add_action(62, ")", ActionMethod::SHIFT, 63);
        pda.add_action(63, "faca", ActionMethod::SHIFT, 64);
        pda.add_action(64, "id", ActionMethod::REDUCE, 32);
        pda.add_action(64, "leia", ActionMethod::REDUCE, 32);
        pda.add_action(64, "escreva", ActionMethod::REDUCE, 32);
        pda.add_action(64, "se", ActionMethod::REDUCE, 32);
        pda.add_action(64, "enquanto", ActionMethod::REDUCE, 32);
        pda.add_action(64, "fimenquanto", ActionMethod::REDUCE, 32);
        pda.add_action(65, "", ActionMethod::REDUCE, 1);
        pda.add_action(66, "", ActionMethod::REDUCE, 9);
        pda.add_action(67, "", ActionMethod::REDUCE, 15);
        pda.add_action(68, "", ActionMethod::REDUCE, 21);
        pda.add_action(69, "", ActionMethod::REDUCE, 30);
        pda.add_action(70, "varfim", ActionMethod::SHIFT, 25);
        pda.add_action(70, "id", ActionMethod::SHIFT, 72);
        pda.add_action(71, "id", ActionMethod::REDUCE, 4);
        pda.add_action(71, "leia", ActionMethod::REDUCE, 4);
        pda.add_action(71, "escreva", ActionMethod::REDUCE, 4);
        pda.add_action(71, "se", ActionMethod::REDUCE, 4);
        pda.add_action(71, "enquanto", ActionMethod::REDUCE, 4);
        pda.add_action(71, "fim", ActionMethod::REDUCE, 4);
        pda.add_action(72, "int", ActionMethod::SHIFT, 74);
        pda.add_action(72, "real", ActionMethod::SHIFT, 75);
        pda.add_action(72, "lit", ActionMethod::SHIFT, 76);
        pda.add_action(73, "id", ActionMethod::REDUCE, 3);
        pda.add_action(73, "leia", ActionMethod::REDUCE, 3);
        pda.add_action(73, "escreva", ActionMethod::REDUCE, 3);
        pda.add_action(73, "se", ActionMethod::REDUCE, 3);
        pda.add_action(73, "enquanto", ActionMethod::REDUCE, 3);
        pda.add_action(73, "fim", ActionMethod::REDUCE, 3);
        pda.add_action(74, ";", ActionMethod::REDUCE, 6);
        pda.add_action(75, ";", ActionMethod::REDUCE, 7);
        pda.add_action(76, ";", ActionMethod::REDUCE, 8);
        pda.add_action(77, ";", ActionMethod::SHIFT, 78);
        pda.add_action(78, "varfim", ActionMethod::REDUCE, 5);
        pda.add_action(78, "id", ActionMethod::REDUCE, 5);

        pda.add_goto(0, "P", 1);
        pda.add_goto(2, "V", 3);
        pda.add_goto(3, "A", 65);
        pda.add_goto(3, "ES", 4);
        pda.add_goto(3, "CMD", 5);
        pda.add_goto(3, "COND", 6);
        pda.add_goto(3, "CABEÇALHO", 12);
        pda.add_goto(3, "REP", 7);
        pda.add_goto(3, "CABEÇALHOREP", 13);
        pda.add_goto(4, "A", 66);
        pda.add_goto(4, "ES", 4);
        pda.add_goto(4, "CMD", 5);
        pda.add_goto(4, "COND", 6);
        pda.add_goto(4, "CABEÇALHO", 12);
        pda.add_goto(4, "REP", 7);
        pda.add_goto(4, "CABEÇALHOREP", 13);
        pda.add_goto(5, "A", 67);
        pda.add_goto(5, "ES", 4);
        pda.add_goto(5, "CMD", 5);
        pda.add_goto(5, "COND", 6);
        pda.add_goto(5, "CABEÇALHO", 12);
        pda.add_goto(5, "REP", 7);
        pda.add_goto(5, "CABEÇALHOREP", 13);
        pda.add_goto(6, "A", 68);
        pda.add_goto(6, "ES", 4);
        pda.add_goto(6, "CMD", 5);
        pda.add_goto(6, "COND", 6);
        pda.add_goto(6, "CABEÇALHO", 12);
        pda.add_goto(6, "REP", 7);
        pda.add_goto(6, "CABEÇALHOREP", 13);
        pda.add_goto(7, "A", 69);
        pda.add_goto(7, "ES", 4);
        pda.add_goto(7, "CMD", 5);
        pda.add_goto(7, "COND", 6);
        pda.add_goto(7, "CABEÇALHO", 12);
        pda.add_goto(7, "REP", 7);
        pda.add_goto(7, "CABEÇALHOREP", 13);
        pda.add_goto(10, "ARG", 21);
        pda.add_goto(12, "ES", 34);
        pda.add_goto(12, "CMD", 35);
        pda.add_goto(12, "COND", 36);
        pda.add_goto(12, "CABEÇALHO", 12);
        pda.add_goto(12, "CORPO", 38);
        pda.add_goto(12, "REP", 37);
        pda.add_goto(12, "CABEÇALHOREP", 13);
        pda.add_goto(13, "ES", 44);
        pda.add_goto(13, "CMD", 45);
        pda.add_goto(13, "COND", 46);
        pda.add_goto(13, "CABEÇALHO", 12);
        pda.add_goto(13, "REP", 47);
        pda.add_goto(13, "CABEÇALHOREP", 13);
        pda.add_goto(13, "CORPOREP", 48);
        pda.add_goto(23, "LV", 24);
        pda.add_goto(23, "D", 70);
        pda.add_goto(26, "LD", 27);
        pda.add_goto(26, "OPRD", 29);
        pda.add_goto(30, "OPRD", 31);
        pda.add_goto(34, "ES", 34);
        pda.add_goto(34, "CMD", 35);
        pda.add_goto(34, "COND", 36);
        pda.add_goto(34, "CABEÇALHO", 12);
        pda.add_goto(34, "CORPO", 39);
        pda.add_goto(34, "REP", 37);
        pda.add_goto(34, "CABEÇALHOREP", 13);
        pda.add_goto(35, "ES", 34);
        pda.add_goto(35, "CMD", 35);
        pda.add_goto(35, "COND", 36);
        pda.add_goto(35, "CABEÇALHO", 12);
        pda.add_goto(35, "CORPO", 40);
        pda.add_goto(35, "REP", 37);
        pda.add_goto(35, "CABEÇALHOREP", 13);
        pda.add_goto(36, "ES", 34);
        pda.add_goto(36, "CMD", 35);
        pda.add_goto(36, "COND", 36);
        pda.add_goto(36, "CABEÇALHO", 12);
        pda.add_goto(36, "CORPO", 41);
        pda.add_goto(36, "REP", 37);
        pda.add_goto(36, "CABEÇALHOREP", 13);
        pda.add_goto(37, "ES", 34);
        pda.add_goto(37, "CMD", 35);
        pda.add_goto(37, "COND", 36);
        pda.add_goto(37, "CABEÇALHO", 12);
        pda.add_goto(37, "CORPO", 42);
        pda.add_goto(37, "REP", 37);
        pda.add_goto(37, "CABEÇALHOREP", 13);
        pda.add_goto(44, "ES", 44);
        pda.add_goto(44, "CMD", 45);
        pda.add_goto(44, "COND", 46);
        pda.add_goto(44, "CABEÇALHO", 12);
        pda.add_goto(44, "REP", 47);
        pda.add_goto(44, "CABEÇALHOREP", 13);
        pda.add_goto(44, "CORPOREP", 49);
        pda.add_goto(45, "ES", 44);
        pda.add_goto(45, "CMD", 45);
        pda.add_goto(45, "COND", 46);
        pda.add_goto(45, "CABEÇALHO", 12);
        pda.add_goto(45, "REP", 47);
        pda.add_goto(45, "CABEÇALHOREP", 13);
        pda.add_goto(45, "CORPOREP", 50);
        pda.add_goto(46, "ES", 44);
        pda.add_goto(46, "CMD", 45);
        pda.add_goto(46, "COND", 46);
        pda.add_goto(46, "CABEÇALHO", 12);
        pda.add_goto(46, "REP", 47);
        pda.add_goto(46, "CABEÇALHOREP", 13);
        pda.add_goto(46, "CORPOREP", 51);
        pda.add_goto(47, "ES", 44);
        pda.add_goto(47, "CMD", 45);
        pda.add_goto(47, "COND", 46);
        pda.add_goto(47, "CABEÇALHO", 12);
        pda.add_goto(47, "REP", 47);
        pda.add_goto(47, "CABEÇALHOREP", 13);
        pda.add_goto(47, "CORPOREP", 52);
        pda.add_goto(54, "OPRD", 58);
        pda.add_goto(54, "EXP_R", 55);
        pda.add_goto(59, "OPRD", 60);
        pda.add_goto(61, "OPRD", 58);
        pda.add_goto(61, "EXP_R", 62);
        pda.add_goto(70, "LV", 73);
        pda.add_goto(70, "D", 70);
        pda.add_goto(72, "TIPO", 77);

        pda.add_reduction(0, 0, 0);
        pda.add_reduction(1, 0, 1);
        pda.add_reduction(2, 1, 1);
        pda.add_reduction(3, 1, 2);
        pda.add_reduction(4, 9, 1);
        pda.add_reduction(5, 15, 1);
        pda.add_reduction(6, 21, 1);
        pda.add_reduction(7, 30, 1);
        pda.add_reduction(8, 38, 1);
        pda.add_reduction(9, 10, 1);
        pda.add_reduction(10, 11, 1);
        pda.add_reduction(11, 16, 1);
        pda.add_reduction(12, 22, 1);
        pda.add_reduction(13, 31, 1);
        pda.add_reduction(14, 23, 1);
        pda.add_reduction(15, 32, 1);
        pda.add_reduction(16, 10, 2);
        pda.add_reduction(17, 10, 3);
        pda.add_reduction(18, 12, 1);
        pda.add_reduction(19, 13, 1);
        pda.add_reduction(20, 14, 1);
        pda.add_reduction(21, 11, 2);
        pda.add_reduction(22, 11, 3);
        pda.add_reduction(23, 2, 1);
        pda.add_reduction(24, 2, 2);
        pda.add_reduction(25, 4, 1);
        pda.add_reduction(26, 16, 2);
        pda.add_reduction(27, 16, 3);
        pda.add_reduction(28, 16, 4);
        pda.add_reduction(29, 18, 1);
        pda.add_reduction(30, 17, 2);
        pda.add_reduction(31, 17, 3);
        pda.add_reduction(32, 19, 1);
        pda.add_reduction(33, 20, 1);
        pda.add_reduction(34, 25, 1);
        pda.add_reduction(35, 26, 1);
        pda.add_reduction(36, 27, 1);
        pda.add_reduction(37, 28, 1);
        pda.add_reduction(38, 22, 2);
        pda.add_reduction(39, 25, 2);
        pda.add_reduction(40, 26, 2);
        pda.add_reduction(41, 27, 2);
        pda.add_reduction(42, 28, 2);
        pda.add_reduction(43, 29, 1);
        pda.add_reduction(44, 33, 1);
        pda.add_reduction(45, 34, 1);
        pda.add_reduction(46, 35, 1);
        pda.add_reduction(47, 36, 1);
        pda.add_reduction(48, 31, 2);
        pda.add_reduction(49, 33, 2);
        pda.add_reduction(50, 34, 2);
        pda.add_reduction(51, 35, 2);
        pda.add_reduction(52, 36, 2);
        pda.add_reduction(53, 37, 1);
        pda.add_reduction(54, 23, 2);
        pda.add_reduction(55, 23, 3);
        pda.add_reduction(56, 23, 4);
        pda.add_reduction(57, 23, 5);
        pda.add_reduction(58, 24, 1);
        pda.add_reduction(59, 24, 2);
        pda.add_reduction(60, 24, 3);
        pda.add_reduction(61, 32, 2);
        pda.add_reduction(62, 32, 3);
        pda.add_reduction(63, 32, 4);
        pda.add_reduction(64, 32, 5);
        pda.add_reduction(65, 1, 3);
        pda.add_reduction(66, 9, 2);
        pda.add_reduction(67, 15, 2);
        pda.add_reduction(68, 21, 2);
        pda.add_reduction(69, 30, 2);
        pda.add_reduction(70, 3, 1);
        pda.add_reduction(71, 4, 2);
        pda.add_reduction(72, 5, 1);
        pda.add_reduction(73, 3, 2);
        pda.add_reduction(74, 6, 1);
        pda.add_reduction(75, 7, 1);
        pda.add_reduction(76, 8, 1);
        pda.add_reduction(77, 5, 2);
        pda.add_reduction(78, 5, 3);

        Syntactic {
            automaton: pda
        }

    }

    pub fn run(&mut self, lexical: &mut Lexical, output_file: &str) -> Result<(), Vec<CompilerError>> {

        let mut errors: Vec<CompilerError> = vec![];

        self.automaton.reset();

        loop {

            let current_line = lexical.current_line();
            let current_column = lexical.current_column();
            let item = lexical.next_token();
            let item_ref = item.borrow();

            if item_ref.token.eq(symbols::tokens::ERROR) {

                errors.push(CompilerError::new(
                    Box::new(LexicalError::new(&item_ref.lexeme)),
                    current_line,
                    current_column));

                continue;

            }

            loop {

                let (result, semantic_errors) = self.automaton.read(&item);

                for e in semantic_errors {

                    errors.push(CompilerError::new(Box::new(e), current_line, current_column));

                }

                match result {

                    Ok(accepted) => {

                        if item_ref.token.eq(symbols::tokens::EOF) {

                            if !accepted {

                                errors.push(CompilerError::new(
                                    Box::new(EndOfFileError{}),
                                    current_line,
                                    current_column));

                            }else if errors.is_empty() {

                                if let Err(e) = self.automaton.semantic().dump(output_file) {

                                    errors.push(CompilerError::new(Box::new(e), current_line, current_column));

                                }else{

                                    return Ok(());

                                }

                            }

                            return Err(errors);

                        }

                        break;

                    },

                    Err(e) => errors.push(CompilerError::new(Box::new(e), current_line, current_column))

                }

            }

        }

    }

}