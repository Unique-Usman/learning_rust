#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    // Example: 42 / 0
    DivideByZero,
    // Only case for `i64`: `i64::MIN / -1` because the result is `i64::MAX + 1`
    IntegerOverflow,
    // Example: 5 / 2 = 2.5
    NotDivisible,
}

// TODO: Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    if b == 0 {
        Err(DivisionError::DivideByZero)
    } else if a == i64::MIN && b == -1 {
        Err(DivisionError::IntegerOverflow)
    } else if a % b != 0 {
        Err(DivisionError::NotDivisible)
    } else {
        Ok(a / b)
    }
}

// TODO: Add the correct return type and complete the function body.
// Desired output: `Ok([1, 11, 1426, 3])`
fn result_with_list() -> Result<Vec<i64>, Vec<DivisionError>> {
    let numbers = [27, 297, 38502, 81];
    let division_results: (
        Vec<Result<i64, DivisionError>>,
        Vec<Result<i64, DivisionError>>,
    ) = numbers
        .iter()
        .map(|n| divide(*n, 27))
        .partition(Result::is_ok);
    let (oks, errs) = division_results;
    if errs.is_empty() {
        Ok(oks.into_iter().map(Result::unwrap).collect())
    } else {
        Err(errs.into_iter().map(Result::unwrap_err).collect())
    }
}

// TODO: Add the correct return type and complete the function body.
// Desired output: `[Ok(1), Ok(11), Ok(1426), Ok(3)]`
fn list_of_results() -> Vec<Result<i64, DivisionError>> {
    let numbers = [27, 297, 38502, 81];
    let division_results: (
        Vec<Result<i64, DivisionError>>,
        Vec<Result<i64, DivisionError>>,
    ) = numbers
        .iter()
        .map(|n| divide(*n, 27))
        .partition(Result::is_ok);
    let (oks, errs) = division_results;
    if errs.is_empty() {
        oks
    } else {
        errs
    }
}

fn main() {
    // You can optionally experiment here.
    assert_eq!(divide(81, 9), Ok(9));
    assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    assert_eq!(divide(i64::MIN, -1), Err(DivisionError::IntegerOverflow));
    assert_eq!(divide(0, 81), Ok(0));
    assert_eq!(divide(81, 6), Err(DivisionError::NotDivisible));
    assert_eq!(result_with_list().unwrap(), [1, 11, 1426, 3]);
    assert_eq!(list_of_results(), [Ok(1), Ok(11), Ok(1426), Ok(3)]);
}
