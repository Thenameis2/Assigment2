#[cfg(test)]
mod tests {
   
    use crate::DynamicLinkedList;

    #[test]
    fn dynamic_list_insert() {
        let mut list = DynamicLinkedList::new();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
        
        list.insert(5);
        assert_eq!(list.len(), 1);
        assert!(!list.is_empty());
        assert_eq!(list.get(0), Some(5));
        
        list.insert(10);
        assert_eq!(list.len(), 2);
        assert_eq!(list.get(0), Some(5));
        assert_eq!(list.get(1), Some(10));
    }
}