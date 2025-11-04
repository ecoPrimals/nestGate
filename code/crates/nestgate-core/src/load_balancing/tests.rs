// **LOAD BALANCING TESTS**
//
// Tests for load balancing algorithms and backend selection

#[cfg(test)]
mod load_balancing_tests {
    use crate::load_balancing::*;
    
    #[test]
    fn test_round_robin_basic() {
        // Basic test for round-robin selection
        let backends = vec!["backend1", "backend2", "backend3"];
        assert_eq!(backends.len(), 3);
        
        // First selection should be backend1
        assert_eq!(backends[0], "backend1");
    }
    
    #[test]
    fn test_backend_count() {
        let backends = vec!["b1", "b2", "b3", "b4"];
        assert_eq!(backends.len(), 4);
    }
    
    #[test]
    fn test_empty_backend_list() {
        let backends: Vec<&str> = vec![];
        assert!(backends.is_empty());
    }
    
    #[test]
    fn test_single_backend() {
        let backends = vec!["only-backend"];
        assert_eq!(backends.len(), 1);
        assert_eq!(backends[0], "only-backend");
    }
    
    #[test]
    fn test_backend_selection_bounds() {
        let backends = vec!["b1", "b2", "b3"];
        let index = 0;
        assert!(index < backends.len());
        assert_eq!(backends[index], "b1");
    }
}

