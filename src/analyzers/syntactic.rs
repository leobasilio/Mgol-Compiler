use symbols;
use analyzers::Lexical;
use analyzers::pda::PDA;
use analyzers::pda::ActionType;

pub struct Syntactic {
    automaton: PDA
}

impl Syntactic {

    pub fn new() -> Self {

        let mut pda = PDA::new();

        pda.add_follow(0, &[""]);
        pda.add_follow(1, &[""]);
        pda.add_follow(2, &[""]);
        pda.add_follow(3, &[""]);
        pda.add_follow(4, &[""]);
        pda.add_follow(5, &[""]);
        pda.add_follow(6, &[""]);
        pda.add_follow(7, &[""]);
        pda.add_follow(8, &[""]);
        pda.add_follow(9, &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow(10, &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow(11, &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow(12, &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow(13, &["leia", "escreva", "id", "fimse", "se", "enquanto", "fim", "fimenquanto"]);
        pda.add_follow(14, &["leia", "escreva", "id", "fimse", "se", "enquanto"]);
        pda.add_follow(15, &["leia", "escreva", "id", "fimenquanto", "se", "enquanto"]);
        pda.add_follow(16, &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow(17, &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow(18, &[";"]);
        pda.add_follow(19, &[";"]);
        pda.add_follow(20, &[";"]);
        pda.add_follow(21, &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow(22, &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow(23, &["fim", "leia", "escreva", "id", "se", "enquanto"]);
        pda.add_follow(24, &["fim", "leia", "escreva", "id", "se", "enquanto"]);
        pda.add_follow(25, &["fim", "leia", "escreva", "id", "se", "enquanto"]);
        pda.add_follow(26, &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow(27, &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow(28, &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow(29, &[";"]);
        pda.add_follow(30, &[";"]);
        pda.add_follow(31, &[";"]);
        pda.add_follow(32, &["opm", ";", "opr", ")"]);
        pda.add_follow(33, &["opm", ";", "opr", ")"]);
        pda.add_follow(34, &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow(35, &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow(36, &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow(37, &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow(38, &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow(39, &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow(40, &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow(41, &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow(42, &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow(43, &["fim", "leia", "escreva", "id", "se", "enquanto", "fimse", "fimenquanto"]);
        pda.add_follow(44, &["leia", "escreva", "id", "fimse", "se", "enquanto", "fim", "fimenquanto"]);
        pda.add_follow(45, &["leia", "escreva", "id", "fimse", "se", "enquanto", "fim", "fimenquanto"]);
        pda.add_follow(46, &["leia", "escreva", "id", "fimse", "se", "enquanto", "fim", "fimenquanto"]);
        pda.add_follow(47, &["leia", "escreva", "id", "fimse", "se", "enquanto", "fim", "fimenquanto"]);
        pda.add_follow(48, &["leia", "escreva", "id", "fimse", "se", "enquanto", "fim", "fimenquanto"]);
        pda.add_follow(49, &["leia", "escreva", "id", "fimse", "se", "enquanto", "fim", "fimenquanto"]);
        pda.add_follow(50, &["leia", "escreva", "id", "fimse", "se", "enquanto", "fim", "fimenquanto"]);
        pda.add_follow(51, &["leia", "escreva", "id", "fimse", "se", "enquanto", "fim", "fimenquanto"]);
        pda.add_follow(52, &["leia", "escreva", "id", "fimse", "se", "enquanto", "fim", "fimenquanto"]);
        pda.add_follow(53, &["leia", "escreva", "id", "fimse", "se", "enquanto", "fim", "fimenquanto"]);
        pda.add_follow(54, &["leia", "escreva", "id", "fimse", "se", "enquanto"]);
        pda.add_follow(55, &["leia", "escreva", "id", "fimse", "se", "enquanto"]);
        pda.add_follow(56, &["leia", "escreva", "id", "fimse", "se", "enquanto"]);
        pda.add_follow(57, &["leia", "escreva", "id", "fimse", "se", "enquanto"]);
        pda.add_follow(58, &[")"]);
        pda.add_follow(59, &[")"]);
        pda.add_follow(60, &[")"]);
        pda.add_follow(61, &["leia", "escreva", "id", "fimenquanto", "se", "enquanto"]);
        pda.add_follow(62, &["leia", "escreva", "id", "fimenquanto", "se", "enquanto"]);
        pda.add_follow(63, &["leia", "escreva", "id", "fimenquanto", "se", "enquanto"]);
        pda.add_follow(64, &["leia", "escreva", "id", "fimenquanto", "se", "enquanto"]);
        pda.add_follow(65, &[""]);
        pda.add_follow(66, &[""]);
        pda.add_follow(67, &[""]);
        pda.add_follow(68, &[""]);
        pda.add_follow(69, &[""]);
        pda.add_follow(70, &["fim", "leia", "escreva", "id", "se", "enquanto"]);
        pda.add_follow(71, &["fim", "leia", "escreva", "id", "se", "enquanto"]);
        pda.add_follow(72, &["varfim", "id"]);
        pda.add_follow(73, &["fim", "leia", "escreva", "id", "se", "enquanto"]);
        pda.add_follow(74, &[";"]);
        pda.add_follow(75, &[";"]);
        pda.add_follow(76, &[";"]);
        pda.add_follow(77, &["varfim", "id"]);
        pda.add_follow(78, &["varfim", "id"]);

        pda.add_reduction(0, "P'", "P");
        pda.add_reduction(1, "P", "inicio V A");
        pda.add_reduction(2, "V", "varinicio LV");
        pda.add_reduction(3, "LV", "D LV");
        pda.add_reduction(4, "LV", "varfim ;");
        pda.add_reduction(5, "D", "id TIPO ;");
        pda.add_reduction(6, "TIPO", "int");
        pda.add_reduction(7, "TIPO", "real");
        pda.add_reduction(8, "TIPO", "lit");
        pda.add_reduction(9, "A", "ES A");
        pda.add_reduction(10, "ES", "leia id ;");
        pda.add_reduction(11, "ES", "escreva ARG ;");
        pda.add_reduction(12, "ARG", "literal");
        pda.add_reduction(13, "ARG", "num");
        pda.add_reduction(14, "ARG", "id");
        pda.add_reduction(15, "A", "CMD A");
        pda.add_reduction(16, "CMD", "id rcb LD ;");
        pda.add_reduction(17, "LD", "OPRD opm OPRD");
        pda.add_reduction(18, "LD", "OPRD");
        pda.add_reduction(19, "OPRD", "id");
        pda.add_reduction(20, "OPRD", "num");
        pda.add_reduction(21, "A", "COND A");
        pda.add_reduction(22, "COND", "CABEÇALHO CORPO");
        pda.add_reduction(23, "CABEÇALHO", "se ( EXP_R ) então");
        pda.add_reduction(24, "EXP_R", "OPRD opr OPRD");
        pda.add_reduction(25, "CORPO", "ES CORPO");
        pda.add_reduction(26, "CORPO", "CMD CORPO");
        pda.add_reduction(27, "CORPO", "COND CORPO");
        pda.add_reduction(28, "CORPO", "REP CORPO");
        pda.add_reduction(29, "CORPO", "fimse");
        pda.add_reduction(30, "A", "REP A");
        pda.add_reduction(31, "REP", "CABEÇALHOREP CORPOREP");
        pda.add_reduction(32, "CABEÇALHOREP", "enquanto ( EXP_R ) faça");
        pda.add_reduction(33, "CORPOREP", "ES CORPOREP");
        pda.add_reduction(34, "CORPOREP", "CMD CORPOREP");
        pda.add_reduction(35, "CORPOREP", "COND CORPOREP");
        pda.add_reduction(36, "CORPOREP", "REP CORPOREP");
        pda.add_reduction(37, "CORPOREP", "fimenquanto");
        pda.add_reduction(38, "A", "fim");

        pda.add_action(0, "inicio", ActionType::SHIFT, 2);
        pda.add_action(1, "", ActionType::ACCEPT, 0);
        pda.add_action(2, "varinicio", ActionType::SHIFT, 23);
        pda.add_action(3, "id", ActionType::SHIFT, 11);
        pda.add_action(3, "leia", ActionType::SHIFT, 9);
        pda.add_action(3, "escreva", ActionType::SHIFT, 10);
        pda.add_action(3, "se", ActionType::SHIFT, 14);
        pda.add_action(3, "enquanto", ActionType::SHIFT, 15);
        pda.add_action(3, "fim", ActionType::SHIFT, 8);
        pda.add_action(4, "id", ActionType::SHIFT, 11);
        pda.add_action(4, "leia", ActionType::SHIFT, 9);
        pda.add_action(4, "escreva", ActionType::SHIFT, 10);
        pda.add_action(4, "se", ActionType::SHIFT, 14);
        pda.add_action(4, "enquanto", ActionType::SHIFT, 15);
        pda.add_action(4, "fim", ActionType::SHIFT, 8);
        pda.add_action(5, "id", ActionType::SHIFT, 11);
        pda.add_action(5, "leia", ActionType::SHIFT, 9);
        pda.add_action(5, "escreva", ActionType::SHIFT, 10);
        pda.add_action(5, "se", ActionType::SHIFT, 14);
        pda.add_action(5, "enquanto", ActionType::SHIFT, 15);
        pda.add_action(5, "fim", ActionType::SHIFT, 8);
        pda.add_action(6, "id", ActionType::SHIFT, 11);
        pda.add_action(6, "leia", ActionType::SHIFT, 9);
        pda.add_action(6, "escreva", ActionType::SHIFT, 10);
        pda.add_action(6, "se", ActionType::SHIFT, 14);
        pda.add_action(6, "enquanto", ActionType::SHIFT, 15);
        pda.add_action(6, "fim", ActionType::SHIFT, 8);
        pda.add_action(7, "id", ActionType::SHIFT, 11);
        pda.add_action(7, "leia", ActionType::SHIFT, 9);
        pda.add_action(7, "escreva", ActionType::SHIFT, 10);
        pda.add_action(7, "se", ActionType::SHIFT, 14);
        pda.add_action(7, "enquanto", ActionType::SHIFT, 15);
        pda.add_action(7, "fim", ActionType::SHIFT, 8);
        pda.add_action(8, "", ActionType::REDUCE, 38);
        pda.add_action(9, "id", ActionType::SHIFT, 16);
        pda.add_action(10, "id", ActionType::SHIFT, 20);
        pda.add_action(10, "literal", ActionType::SHIFT, 18);
        pda.add_action(10, "num", ActionType::SHIFT, 19);
        pda.add_action(11, "rcb", ActionType::SHIFT, 26);
        pda.add_action(12, "id", ActionType::SHIFT, 11);
        pda.add_action(12, "leia", ActionType::SHIFT, 9);
        pda.add_action(12, "escreva", ActionType::SHIFT, 10);
        pda.add_action(12, "se", ActionType::SHIFT, 14);
        pda.add_action(12, "fimse", ActionType::SHIFT, 43);
        pda.add_action(12, "enquanto", ActionType::SHIFT, 15);
        pda.add_action(13, "id", ActionType::SHIFT, 11);
        pda.add_action(13, "leia", ActionType::SHIFT, 9);
        pda.add_action(13, "escreva", ActionType::SHIFT, 10);
        pda.add_action(13, "se", ActionType::SHIFT, 14);
        pda.add_action(13, "enquanto", ActionType::SHIFT, 15);
        pda.add_action(13, "fimenquanto", ActionType::SHIFT, 53);
        pda.add_action(14, "(", ActionType::SHIFT, 54);
        pda.add_action(15, "(", ActionType::SHIFT, 61);
        pda.add_action(16, ";", ActionType::SHIFT, 17);
        pda.add_action(17, "id", ActionType::REDUCE, 10);
        pda.add_action(17, "leia", ActionType::REDUCE, 10);
        pda.add_action(17, "escreva", ActionType::REDUCE, 10);
        pda.add_action(17, "se", ActionType::REDUCE, 10);
        pda.add_action(17, "fimse", ActionType::REDUCE, 10);
        pda.add_action(17, "enquanto", ActionType::REDUCE, 10);
        pda.add_action(17, "fimenquanto", ActionType::REDUCE, 10);
        pda.add_action(17, "fim", ActionType::REDUCE, 10);
        pda.add_action(18, ";", ActionType::REDUCE, 12);
        pda.add_action(19, ";", ActionType::REDUCE, 13);
        pda.add_action(20, ";", ActionType::REDUCE, 14);
        pda.add_action(21, ";", ActionType::SHIFT, 22);
        pda.add_action(22, "id", ActionType::REDUCE, 11);
        pda.add_action(22, "leia", ActionType::REDUCE, 11);
        pda.add_action(22, "escreva", ActionType::REDUCE, 11);
        pda.add_action(22, "se", ActionType::REDUCE, 11);
        pda.add_action(22, "fimse", ActionType::REDUCE, 11);
        pda.add_action(22, "enquanto", ActionType::REDUCE, 11);
        pda.add_action(22, "fimenquanto", ActionType::REDUCE, 11);
        pda.add_action(22, "fim", ActionType::REDUCE, 11);
        pda.add_action(23, "varfim", ActionType::SHIFT, 25);
        pda.add_action(23, "id", ActionType::SHIFT, 72);
        pda.add_action(24, "id", ActionType::REDUCE, 2);
        pda.add_action(24, "leia", ActionType::REDUCE, 2);
        pda.add_action(24, "escreva", ActionType::REDUCE, 2);
        pda.add_action(24, "se", ActionType::REDUCE, 2);
        pda.add_action(24, "enquanto", ActionType::REDUCE, 2);
        pda.add_action(24, "fim", ActionType::REDUCE, 2);
        pda.add_action(25, ";", ActionType::SHIFT, 71);
        pda.add_action(26, "id", ActionType::SHIFT, 32);
        pda.add_action(26, "num", ActionType::SHIFT, 33);
        pda.add_action(27, ";", ActionType::SHIFT, 28);
        pda.add_action(28, "id", ActionType::REDUCE, 16);
        pda.add_action(28, "leia", ActionType::REDUCE, 16);
        pda.add_action(28, "escreva", ActionType::REDUCE, 16);
        pda.add_action(28, "se", ActionType::REDUCE, 16);
        pda.add_action(28, "fimse", ActionType::REDUCE, 16);
        pda.add_action(28, "enquanto", ActionType::REDUCE, 16);
        pda.add_action(28, "fimenquanto", ActionType::REDUCE, 16);
        pda.add_action(28, "fim", ActionType::REDUCE, 16);
        pda.add_action(29, ";", ActionType::REDUCE, 18);
        pda.add_action(29, "opm", ActionType::SHIFT, 30);
        pda.add_action(30, "id", ActionType::SHIFT, 32);
        pda.add_action(30, "num", ActionType::SHIFT, 33);
        pda.add_action(31, ";", ActionType::REDUCE, 17);
        pda.add_action(32, ";", ActionType::REDUCE, 19);
        pda.add_action(32, "opm", ActionType::REDUCE, 19);
        pda.add_action(32, ")", ActionType::REDUCE, 19);
        pda.add_action(32, "opr", ActionType::REDUCE, 19);
        pda.add_action(33, ";", ActionType::REDUCE, 20);
        pda.add_action(33, "opm", ActionType::REDUCE, 20);
        pda.add_action(33, ")", ActionType::REDUCE, 20);
        pda.add_action(33, "opr", ActionType::REDUCE, 20);
        pda.add_action(34, "id", ActionType::SHIFT, 11);
        pda.add_action(34, "leia", ActionType::SHIFT, 9);
        pda.add_action(34, "escreva", ActionType::SHIFT, 10);
        pda.add_action(34, "se", ActionType::SHIFT, 14);
        pda.add_action(34, "fimse", ActionType::SHIFT, 43);
        pda.add_action(34, "enquanto", ActionType::SHIFT, 15);
        pda.add_action(35, "id", ActionType::SHIFT, 11);
        pda.add_action(35, "leia", ActionType::SHIFT, 9);
        pda.add_action(35, "escreva", ActionType::SHIFT, 10);
        pda.add_action(35, "se", ActionType::SHIFT, 14);
        pda.add_action(35, "fimse", ActionType::SHIFT, 43);
        pda.add_action(35, "enquanto", ActionType::SHIFT, 15);
        pda.add_action(36, "id", ActionType::SHIFT, 11);
        pda.add_action(36, "leia", ActionType::SHIFT, 9);
        pda.add_action(36, "escreva", ActionType::SHIFT, 10);
        pda.add_action(36, "se", ActionType::SHIFT, 14);
        pda.add_action(36, "fimse", ActionType::SHIFT, 43);
        pda.add_action(36, "enquanto", ActionType::SHIFT, 15);
        pda.add_action(37, "id", ActionType::SHIFT, 11);
        pda.add_action(37, "leia", ActionType::SHIFT, 9);
        pda.add_action(37, "escreva", ActionType::SHIFT, 10);
        pda.add_action(37, "se", ActionType::SHIFT, 14);
        pda.add_action(37, "fimse", ActionType::SHIFT, 43);
        pda.add_action(37, "enquanto", ActionType::SHIFT, 15);
        pda.add_action(38, "id", ActionType::REDUCE, 22);
        pda.add_action(38, "leia", ActionType::REDUCE, 22);
        pda.add_action(38, "escreva", ActionType::REDUCE, 22);
        pda.add_action(38, "se", ActionType::REDUCE, 22);
        pda.add_action(38, "fimse", ActionType::REDUCE, 22);
        pda.add_action(38, "enquanto", ActionType::REDUCE, 22);
        pda.add_action(38, "fimenquanto", ActionType::REDUCE, 22);
        pda.add_action(38, "fim", ActionType::REDUCE, 22);
        pda.add_action(39, "id", ActionType::REDUCE, 25);
        pda.add_action(39, "leia", ActionType::REDUCE, 25);
        pda.add_action(39, "escreva", ActionType::REDUCE, 25);
        pda.add_action(39, "se", ActionType::REDUCE, 25);
        pda.add_action(39, "fimse", ActionType::REDUCE, 25);
        pda.add_action(39, "enquanto", ActionType::REDUCE, 25);
        pda.add_action(39, "fimenquanto", ActionType::REDUCE, 25);
        pda.add_action(39, "fim", ActionType::REDUCE, 25);
        pda.add_action(40, "id", ActionType::REDUCE, 26);
        pda.add_action(40, "leia", ActionType::REDUCE, 26);
        pda.add_action(40, "escreva", ActionType::REDUCE, 26);
        pda.add_action(40, "se", ActionType::REDUCE, 26);
        pda.add_action(40, "fimse", ActionType::REDUCE, 26);
        pda.add_action(40, "enquanto", ActionType::REDUCE, 26);
        pda.add_action(40, "fimenquanto", ActionType::REDUCE, 26);
        pda.add_action(40, "fim", ActionType::REDUCE, 26);
        pda.add_action(41, "id", ActionType::REDUCE, 27);
        pda.add_action(41, "leia", ActionType::REDUCE, 27);
        pda.add_action(41, "escreva", ActionType::REDUCE, 27);
        pda.add_action(41, "se", ActionType::REDUCE, 27);
        pda.add_action(41, "fimse", ActionType::REDUCE, 27);
        pda.add_action(41, "enquanto", ActionType::REDUCE, 27);
        pda.add_action(41, "fimenquanto", ActionType::REDUCE, 27);
        pda.add_action(41, "fim", ActionType::REDUCE, 27);
        pda.add_action(42, "id", ActionType::REDUCE, 28);
        pda.add_action(42, "leia", ActionType::REDUCE, 28);
        pda.add_action(42, "escreva", ActionType::REDUCE, 28);
        pda.add_action(42, "se", ActionType::REDUCE, 28);
        pda.add_action(42, "fimse", ActionType::REDUCE, 28);
        pda.add_action(42, "enquanto", ActionType::REDUCE, 28);
        pda.add_action(42, "fimenquanto", ActionType::REDUCE, 28);
        pda.add_action(42, "fim", ActionType::REDUCE, 28);
        pda.add_action(43, "id", ActionType::REDUCE, 29);
        pda.add_action(43, "leia", ActionType::REDUCE, 29);
        pda.add_action(43, "escreva", ActionType::REDUCE, 29);
        pda.add_action(43, "se", ActionType::REDUCE, 29);
        pda.add_action(43, "fimse", ActionType::REDUCE, 29);
        pda.add_action(43, "enquanto", ActionType::REDUCE, 29);
        pda.add_action(43, "fimenquanto", ActionType::REDUCE, 29);
        pda.add_action(43, "fim", ActionType::REDUCE, 29);
        pda.add_action(44, "id", ActionType::SHIFT, 11);
        pda.add_action(44, "leia", ActionType::SHIFT, 9);
        pda.add_action(44, "escreva", ActionType::SHIFT, 10);
        pda.add_action(44, "se", ActionType::SHIFT, 14);
        pda.add_action(44, "enquanto", ActionType::SHIFT, 15);
        pda.add_action(44, "fimenquanto", ActionType::SHIFT, 53);
        pda.add_action(45, "id", ActionType::SHIFT, 11);
        pda.add_action(45, "leia", ActionType::SHIFT, 9);
        pda.add_action(45, "escreva", ActionType::SHIFT, 10);
        pda.add_action(45, "se", ActionType::SHIFT, 14);
        pda.add_action(45, "enquanto", ActionType::SHIFT, 15);
        pda.add_action(45, "fimenquanto", ActionType::SHIFT, 53);
        pda.add_action(46, "id", ActionType::SHIFT, 11);
        pda.add_action(46, "leia", ActionType::SHIFT, 9);
        pda.add_action(46, "escreva", ActionType::SHIFT, 10);
        pda.add_action(46, "se", ActionType::SHIFT, 14);
        pda.add_action(46, "enquanto", ActionType::SHIFT, 15);
        pda.add_action(46, "fimenquanto", ActionType::SHIFT, 53);
        pda.add_action(47, "id", ActionType::SHIFT, 11);
        pda.add_action(47, "leia", ActionType::SHIFT, 9);
        pda.add_action(47, "escreva", ActionType::SHIFT, 10);
        pda.add_action(47, "se", ActionType::SHIFT, 14);
        pda.add_action(47, "enquanto", ActionType::SHIFT, 15);
        pda.add_action(47, "fimenquanto", ActionType::SHIFT, 53);
        pda.add_action(48, "id", ActionType::REDUCE, 31);
        pda.add_action(48, "leia", ActionType::REDUCE, 31);
        pda.add_action(48, "escreva", ActionType::REDUCE, 31);
        pda.add_action(48, "se", ActionType::REDUCE, 31);
        pda.add_action(48, "fimse", ActionType::REDUCE, 31);
        pda.add_action(48, "enquanto", ActionType::REDUCE, 31);
        pda.add_action(48, "fimenquanto", ActionType::REDUCE, 31);
        pda.add_action(48, "fim", ActionType::REDUCE, 31);
        pda.add_action(49, "id", ActionType::REDUCE, 33);
        pda.add_action(49, "leia", ActionType::REDUCE, 33);
        pda.add_action(49, "escreva", ActionType::REDUCE, 33);
        pda.add_action(49, "se", ActionType::REDUCE, 33);
        pda.add_action(49, "fimse", ActionType::REDUCE, 33);
        pda.add_action(49, "enquanto", ActionType::REDUCE, 33);
        pda.add_action(49, "fimenquanto", ActionType::REDUCE, 33);
        pda.add_action(49, "fim", ActionType::REDUCE, 33);
        pda.add_action(50, "id", ActionType::REDUCE, 34);
        pda.add_action(50, "leia", ActionType::REDUCE, 34);
        pda.add_action(50, "escreva", ActionType::REDUCE, 34);
        pda.add_action(50, "se", ActionType::REDUCE, 34);
        pda.add_action(50, "fimse", ActionType::REDUCE, 34);
        pda.add_action(50, "enquanto", ActionType::REDUCE, 34);
        pda.add_action(50, "fimenquanto", ActionType::REDUCE, 34);
        pda.add_action(50, "fim", ActionType::REDUCE, 34);
        pda.add_action(51, "id", ActionType::REDUCE, 35);
        pda.add_action(51, "leia", ActionType::REDUCE, 35);
        pda.add_action(51, "escreva", ActionType::REDUCE, 35);
        pda.add_action(51, "se", ActionType::REDUCE, 35);
        pda.add_action(51, "fimse", ActionType::REDUCE, 35);
        pda.add_action(51, "enquanto", ActionType::REDUCE, 35);
        pda.add_action(51, "fimenquanto", ActionType::REDUCE, 35);
        pda.add_action(51, "fim", ActionType::REDUCE, 35);
        pda.add_action(52, "id", ActionType::REDUCE, 36);
        pda.add_action(52, "leia", ActionType::REDUCE, 36);
        pda.add_action(52, "escreva", ActionType::REDUCE, 36);
        pda.add_action(52, "se", ActionType::REDUCE, 36);
        pda.add_action(52, "fimse", ActionType::REDUCE, 36);
        pda.add_action(52, "enquanto", ActionType::REDUCE, 36);
        pda.add_action(52, "fimenquanto", ActionType::REDUCE, 36);
        pda.add_action(52, "fim", ActionType::REDUCE, 36);
        pda.add_action(53, "id", ActionType::REDUCE, 37);
        pda.add_action(53, "leia", ActionType::REDUCE, 37);
        pda.add_action(53, "escreva", ActionType::REDUCE, 37);
        pda.add_action(53, "se", ActionType::REDUCE, 37);
        pda.add_action(53, "fimse", ActionType::REDUCE, 37);
        pda.add_action(53, "enquanto", ActionType::REDUCE, 37);
        pda.add_action(53, "fimenquanto", ActionType::REDUCE, 37);
        pda.add_action(53, "fim", ActionType::REDUCE, 37);
        pda.add_action(54, "id", ActionType::SHIFT, 32);
        pda.add_action(54, "num", ActionType::SHIFT, 33);
        pda.add_action(55, ")", ActionType::SHIFT, 56);
        pda.add_action(56, "entao", ActionType::SHIFT, 57);
        pda.add_action(57, "id", ActionType::REDUCE, 23);
        pda.add_action(57, "leia", ActionType::REDUCE, 23);
        pda.add_action(57, "escreva", ActionType::REDUCE, 23);
        pda.add_action(57, "se", ActionType::REDUCE, 23);
        pda.add_action(57, "fimse", ActionType::REDUCE, 23);
        pda.add_action(57, "enquanto", ActionType::REDUCE, 23);
        pda.add_action(58, "opr", ActionType::SHIFT, 59);
        pda.add_action(59, "id", ActionType::SHIFT, 32);
        pda.add_action(59, "num", ActionType::SHIFT, 33);
        pda.add_action(60, ")", ActionType::REDUCE, 24);
        pda.add_action(61, "id", ActionType::SHIFT, 32);
        pda.add_action(61, "num", ActionType::SHIFT, 33);
        pda.add_action(62, ")", ActionType::SHIFT, 63);
        pda.add_action(63, "faça", ActionType::SHIFT, 64);
        pda.add_action(64, "id", ActionType::REDUCE, 32);
        pda.add_action(64, "leia", ActionType::REDUCE, 32);
        pda.add_action(64, "escreva", ActionType::REDUCE, 32);
        pda.add_action(64, "se", ActionType::REDUCE, 32);
        pda.add_action(64, "enquanto", ActionType::REDUCE, 32);
        pda.add_action(64, "fimenquanto", ActionType::REDUCE, 32);
        pda.add_action(65, "", ActionType::REDUCE, 1);
        pda.add_action(66, "", ActionType::REDUCE, 9);
        pda.add_action(67, "", ActionType::REDUCE, 15);
        pda.add_action(68, "", ActionType::REDUCE, 21);
        pda.add_action(69, "", ActionType::REDUCE, 30);
        pda.add_action(70, "varfim", ActionType::SHIFT, 25);
        pda.add_action(70, "id", ActionType::SHIFT, 72);
        pda.add_action(71, "id", ActionType::REDUCE, 4);
        pda.add_action(71, "leia", ActionType::REDUCE, 4);
        pda.add_action(71, "escreva", ActionType::REDUCE, 4);
        pda.add_action(71, "se", ActionType::REDUCE, 4);
        pda.add_action(71, "enquanto", ActionType::REDUCE, 4);
        pda.add_action(71, "fim", ActionType::REDUCE, 4);
        pda.add_action(72, "int", ActionType::SHIFT, 74);
        pda.add_action(72, "real", ActionType::SHIFT, 75);
        pda.add_action(72, "lit", ActionType::SHIFT, 76);
        pda.add_action(73, "id", ActionType::REDUCE, 3);
        pda.add_action(73, "leia", ActionType::REDUCE, 3);
        pda.add_action(73, "escreva", ActionType::REDUCE, 3);
        pda.add_action(73, "se", ActionType::REDUCE, 3);
        pda.add_action(73, "enquanto", ActionType::REDUCE, 3);
        pda.add_action(73, "fim", ActionType::REDUCE, 3);
        pda.add_action(74, ";", ActionType::REDUCE, 6);
        pda.add_action(75, ";", ActionType::REDUCE, 7);
        pda.add_action(76, ";", ActionType::REDUCE, 8);
        pda.add_action(77, ";", ActionType::SHIFT, 78);
        pda.add_action(78, "varfim", ActionType::REDUCE, 5);
        pda.add_action(78, "id", ActionType::REDUCE, 5);

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

        Syntactic {
            automaton: pda
        }

    }

    pub fn run(&mut self, lexical: &mut Lexical) -> Result<(), String>{

        loop {

            let current_line = lexical.current_line();
            let current_column = lexical.current_column();
            let item = lexical.next_token();

            match self.automaton.read(&Syntactic::get_pda_lexeme(&item)) {

                Ok(accepted) => {

                    if item.token.eq(symbols::tokens::EOF) {

                        if accepted {

                            return Ok(());

                        }else {

                            return Err("Final inesperado do arquivo".to_string());

                        }

                    }

                },

                Err(e) => return Err(format!("{}, linha {}, coluna {}, lido: {}", e, current_line, current_column, item.lexeme))

            }

        }

    }

    fn get_pda_lexeme(item: &symbols::Symbol) -> String {

        match item.token.as_ref() {

            symbols::tokens::EOF => String::from(""),

            symbols::tokens::IDENTIFIER => String::from("id"),

            symbols::tokens::LITERAL => String::from("literal"),

            symbols::tokens::ARITHMETIC => String::from("opm"),

            symbols::tokens::RELATIONAL => String::from("opr"),

            symbols::tokens::NUMBER => String::from("num"),

            symbols::tokens::ATTRIBUTION => String::from("rcb"),

            "inteiro" => String::from("int"),

            _ => item.lexeme.clone()

        }

    }

}