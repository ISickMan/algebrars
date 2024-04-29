use itertools::Itertools;

use crate::{
    arithmatic::OperationError,
    math_tree::{MathTree, TreeNodeRef},
    stepper::Steps,
};

// since contrary to addition, substraction is not an orderless operation,
// for simplification purposes, it is easier to represent substration as addition of a negative term
// this allows for grouping of addition and substraction

impl MathTree {
    pub fn simplify(&mut self, steps: &mut Steps) -> Result<(), OperationError> {
        while let Some(complete) = Self::simplify_node(&mut self.root, steps)? {
            self.root = complete;
        }
        Ok(())
    }

    fn simplify_node(
        node: &mut TreeNodeRef,
        steps: &mut Steps,
    ) -> Result<Option<TreeNodeRef>, OperationError> {
        // let node = &mut self.root;
        if !node.val().is_operator() {
            return Ok(None);
        }

        let mut borrow = node.borrow_mut();

        let operators = borrow.operands.operators().collect_vec();
        // let mut multipliers = Vec::new();
        for op_pos in operators {
            let mut op = borrow.operands[op_pos].clone();
            if let Some(complete) = Self::simplify_node(&mut op, steps)? {
                borrow.operands.replace_val(op_pos, complete);
            }
            // if let MathToken::Op(Operation
            // simplify_test("-(-2)", TreeNodeRef::constant(dec!(2)));

            // // lex: 5 sub ( sub 2 )
            // // pf: 52-- sub(sub(5, 2)) = sub(3) = -3 WRONG!
            // // pf: 5-2- sub(5, sub(2)) = sub(5, -2) = 7 right => -2 needs to be parsed as a decimal not substract
            // simplify_test("5-Token::Multiply) = op.val() {
            //     multipliers.push(op.clone());
            // }
            // borrow.add_operand(op);
        }

        // println!("simplifying {:#?}", borrow);
        std::mem::drop(borrow);

        Ok(if let Some(complete) = Self::perform_op(node, steps)? {
            Some(complete)
        } else {
            None
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        math_tree::{MathTree, TreeNodeRef},
        stepper::Steps,
        MathToken, OperationToken,
    };
    use pretty_assertions::assert_eq;
    use rust_decimal_macros::dec;

    fn simplify_test(expr: &str, res: TreeNodeRef){
        let mut simplified = MathTree::parse(expr);
        let mut steps = Steps::new();
        if let Err(e) = simplified.simplify(&mut steps) {
            panic!("{:?}", e);
        }

        assert_eq!(simplified.root, res);
    }

    #[test]
    fn simplify_constants() {
        simplify_test("1 + 2", TreeNodeRef::constant(dec!(3)));

        simplify_test("1 + 2 + 3", TreeNodeRef::constant(dec!(6)));

        simplify_test("1 + 2*2 + 3", TreeNodeRef::constant(dec!(8)));

        simplify_test("2 + 2^2", TreeNodeRef::constant(dec!(6)));

        simplify_test("2 + 2^3", TreeNodeRef::constant(dec!(10)));
    }

    #[test]
    fn simplify_x() {
        simplify_test(
            "1*x",
            TreeNodeRef::new_val(MathToken::variable(String::from("x"))),
        );

        simplify_test("0*x", TreeNodeRef::constant(dec!(0)));

        simplify_test(
            "0 + x",
            TreeNodeRef::new_val(MathToken::variable(String::from("x"))),
        );

        simplify_test(
            "x + x",
            TreeNodeRef::new_vals(
                MathToken::operator(OperationToken::Multiply),
                vec![
                    TreeNodeRef::new_val(MathToken::variable(String::from("x"))),
                    TreeNodeRef::constant(dec!(2)),
                ],
            ),
        );

        // simplify_test(
        //     "2*x + x",
        //     TreeNodeRef::new_vals(
        //         MathToken::Op(OperationToken::Multiply),
        //         vec![
        //             TreeNodeRef::constant(dec!(3)),
        //             TreeNodeRef::new_val(MathToken::Variable(String::from("x"))),
        //         ],
        //     ),
        // );
    }

    #[test]
    fn zero_and_double_add_subs() {
        simplify_test(
            "+x",
            TreeNodeRef::new_val(MathToken::variable(String::from("x"))),
        );

        simplify_test(
            "-x",
            TreeNodeRef::new_vals(
                MathToken::operator(OperationToken::Multiply),
                vec![
                    TreeNodeRef::constant(dec!(-1)),
                    TreeNodeRef::new_val(MathToken::variable(String::from("x"))),
                ],
            ),
        );

        simplify_test(
            "+(+x)",
            TreeNodeRef::new_val(MathToken::variable(String::from("x"))),
        );

        simplify_test(
            "-(-x)",
            TreeNodeRef::new_val(MathToken::variable(String::from("x"))),
        );

        simplify_test("-(-2)", TreeNodeRef::constant(dec!(2)));

        // // lex: 5 sub ( sub 2 )
        // // pf: 52-- sub(sub(5, 2)) = sub(3) = -3 WRONG!
        // // pf: 5-2- sub(5, sub(2)) = sub(5, -2) = 7 right => -2 needs to be parsed as a decimal not substract
        simplify_test("5-(-2)", TreeNodeRef::constant(dec!(7)));
    }
}
