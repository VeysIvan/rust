/// Операчия над двумя выражениями.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// Выражение в форме узла дерева.
#[derive(Debug)]
enum Expression {
    /// Операция над двумя дочерними выражениями.
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },

    /// Значение
    Value(i64),
}

fn eval(e: Expression) -> i64 {
    
    if let Expression::Value(val) = e {
        return val;
    } else if let Expression::Op{op,left,right} = e {
        let mut leftX:i64;
        let mut rightX:i64;
        //if let Box::new(a) = left {
            leftX = eval(*left);
        //} else { leftX = 0;}
        //if let Box::new(a) = right {
            rightX = eval(*right);
        //} else { rightX = 0;}
        match op {
            Operation::Add => {
                return leftX + rightX;
            }
            Operation::Mul => {
                return leftX * rightX;
            }
            Operation::Sub => {
                return leftX - rightX;
            }
            Operation::Div => {
                return leftX / rightX;
            }
        }
        return 0;
    } else {
        return 0;
    }
}

#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), 19);
}

#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        30
    );
}

#[test]
fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        85
    );
}

#[test]
fn test_zeros() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
}
