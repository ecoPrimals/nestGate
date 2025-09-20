// **RESULT TYPE EXTENSIONS**
//! Extensions functionality and utilities.
// Extension methods for Result types providing additional functionality.

use super::result_types::IdioResult;

/// **RESULT CHAINING EXTENSIONS**
/// Extensions for chaining operations on Result types
pub trait ResultChain<T, E> {
    /// Chain another operation that returns a Result
    fn and_then_chain<U, F>(self, f: F) -> IdioResult<U, E>
    where
        F: FnOnce(T) -> IdioResult<U, E>;
    
    /// Apply a function to the Ok value and flatten nested Results
    fn flatten_map<U, F>(self, f: F) -> IdioResult<U, E>
    where
        F: FnOnce(T) -> IdioResult<U, E>;
}
impl<T, E> ResultChain<T, E> for IdioResult<T, E> {
    fn and_then_chain<U, F>(self, f: F) -> IdioResult<U, E>
    where
        F: FnOnce(T) -> IdioResult<U, E>,
    {
        self.and_then(f)
    }
    
    fn flatten_map<U, F>(self, f: F) -> IdioResult<U, E>
    where
        F: FnOnce(T) -> IdioResult<U, E>,
    {
        self.and_then(f)
    }
}

/// **RESULT INSPECTION EXTENSIONS**
/// Extensions for inspecting Result values without consuming them
pub trait ResultInspect<T, E> {
    /// Inspect the Ok value without consuming the Result
    fn inspect_ok<F>(self, f: F) -> Self
    where
        ,
            F: FnOnce(&T);
    
    /// Inspect the Err value without consuming the Result
    fn inspect_err<F>(self, f: F) -> Self
    where
        ,
            F: FnOnce(&E);
    
    /// Inspect both Ok and Err values
    fn inspect_both<F, G>(self, ok_fn: F, err_fn: G) -> Self
    where
        ,
            F: FnOnce(&T),
        G: FnOnce(&E);
}
impl<T, E> ResultInspect<T, E> for IdioResult<T, E> {
    fn inspect_ok<F>(self, f: F) -> Self
    where
        ,
            F: FnOnce(&T),
    {
        if let Ok(ref value) = self {
            f(value);
        }
        self
    }
    
    fn inspect_err<F>(self, f: F) -> Self
    where
        ,
            F: FnOnce(&E),
    {
        if let Err(ref error) = self {
            f(error);
        }
        self
    }
    
    fn inspect_both<F, G>(self, ok_fn: F, err_fn: G) -> Self
    where
        ,
            F: FnOnce(&T),
        G: FnOnce(&E),
    {
        match &self {
            Ok(value) => ok_fn(value),
            Err(error) => err_fn(error),
        }
        self
    }
}

/// **RESULT RECOVERY EXTENSIONS**
/// Extensions for recovering from errors with fallback values
pub trait ResultRecover<T, E> {
    /// Provide a default value on error
    fn or_default(self) -> T
    where
        T: Default;
    
    /// Recover with a computed value
    fn recover_with<F>(self, f: F) -> T
    where
        ,
            F: FnOnce(E) -> T;
    
    /// Try to recover with another Result
    fn or_else_try<F>(self, f: F) -> IdioResult<T, E>
    where
        F: FnOnce(E) -> IdioResult<T, E>;
}
impl<T, E> ResultRecover<T, E> for IdioResult<T, E> {
    fn or_default(self) -> T
    where
        T: Default,
    {
        self.unwrap_or_default()
    }
    
    fn recover_with<F>(self, f: F) -> T
    where
        ,
            F: FnOnce(E) -> T,
    {
        self.unwrap_or_else(f)
    }
    
    fn or_else_try<F>(self, f: F) -> IdioResult<T, E>
    where
        F: FnOnce(E) -> IdioResult<T, E>,
    {
        self.or_else(f)
    }
} 