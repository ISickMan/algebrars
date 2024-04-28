use crate::{
    lexer::OPERATOR_MAP,
    math_tree::{MathTree, TreeNodeRef},
    MathToken, MathTokenType,
};

impl MathTree {
    pub fn to_latex(&self) -> String {
        let mut res = String::new();

        Self::to_latex_node(self.root.clone(), &mut res);
        res
    }

    fn to_latex_node(node: TreeNodeRef, res: &mut String) {
        let borrow = node.borrow();

        let mut childs = borrow.operand_iter();
        Self::token_to_latex(childs.next().unwrap().1, res);

        for (_, child) in childs {
            // res +=
            if !node.val().is_operator() {
                panic!()
            };
            res.push(OPERATOR_MAP.get_by_right(&node.val()).unwrap().clone());

            Self::token_to_latex(child, res);
        }
    }

    fn token_to_latex(child: &TreeNodeRef, res: &mut String) {
        let val = child.val();
        match val.kind {
            MathTokenType::Constant => res.push_str(&val.constant.unwrap().to_string()),
            MathTokenType::Variable => res.push_str(&val.variable.unwrap()),
            MathTokenType::Operator => {
                res.push('(');
                Self::to_latex_node(child.clone(), res);
                res.push(')');
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::math_tree::MathTree;

    #[test]
    pub fn simple_latex() {
        assert_eq!(MathTree::parse("2 * x").to_latex(), "2*x");

        assert_eq!(MathTree::parse("2 * (x + 1)").to_latex(), "2*(x+1)");

        assert_eq!(
            MathTree::parse("2 * (x + 1 + (2 + 3))").to_latex(),
            "2*(x+1+2+3)"
        );

        assert_eq!(
            MathTree::parse("2 * ((x) + (1) + (2 + 3))").to_latex(),
            "2*(x+1+2+3)"
        );

        assert_eq!(
            MathTree::parse("1 + 5 + 2 * 5 + 3 + 1").to_latex(),
            "(2*5)+1+5+3+1"
        );

        assert_eq!(
            MathTree::parse("2 * 5 * 3 + 1 * 2 + 3").to_latex(),
            "(2*5*3)+(1*2)+3"
        );
    }
}
