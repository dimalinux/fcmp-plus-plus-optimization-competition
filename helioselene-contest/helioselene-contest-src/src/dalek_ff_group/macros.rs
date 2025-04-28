macro_rules! constant_time {
    ($Value: ident, $Inner: ident) => {
        impl ConstantTimeEq for $Value {
            fn ct_eq(&self, other: &Self) -> Choice {
                self.0.ct_eq(&other.0)
            }
        }

        impl ConditionallySelectable for $Value {
            fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
                $Value($Inner::conditional_select(&a.0, &b.0, choice))
            }
        }
    };
}
pub(crate) use constant_time;

macro_rules! math_op {
    (
    $Value: ident,
    $Other: ident,
    $Op: ident,
    $op_fn: ident,
    $Assign: ident,
    $assign_fn: ident,
    $function: expr
  ) => {
        impl $Op<$Other> for $Value {
            type Output = $Value;

            fn $op_fn(self, other: $Other) -> Self::Output {
                Self($function(self.0, other.0))
            }
        }
        impl $Assign<$Other> for $Value {
            fn $assign_fn(&mut self, other: $Other) {
                self.0 = $function(self.0, other.0);
            }
        }
        impl<'a> $Op<&'a $Other> for $Value {
            type Output = $Value;

            fn $op_fn(self, other: &'a $Other) -> Self::Output {
                Self($function(self.0, other.0))
            }
        }
        impl<'a> $Assign<&'a $Other> for $Value {
            fn $assign_fn(&mut self, other: &'a $Other) {
                self.0 = $function(self.0, other.0);
            }
        }
    };
}
pub(crate) use math_op;

macro_rules! math {
    ($Value: ident, $Factor: ident, $add: expr, $sub: expr, $mul: expr) => {
        math_op!($Value, $Value, Add, add, AddAssign, add_assign, $add);
        math_op!($Value, $Value, Sub, sub, SubAssign, sub_assign, $sub);
        math_op!($Value, $Factor, Mul, mul, MulAssign, mul_assign, $mul);
    };
}
pub(crate) use math;
