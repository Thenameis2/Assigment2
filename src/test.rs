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

    #[test]
    fn dynamic_list_insert_at_index() {
        let mut list = DynamicLinkedList::new();

        list.insert(1);
        list.insert(3);
        list.insert(5);
        // List: 1 -> 3 -> 5

        list.insert_at_index(1, 2); 
        // List: 1 -> 2 -> 3 -> 5
        assert_eq!(list.get(0), Some(1));
        assert_eq!(list.get(1), Some(2));
        assert_eq!(list.get(2), Some(3));
        assert_eq!(list.get(3), Some(5));

        list.insert_at_index(0, 0); 
        // List: 0 -> 1 -> 2 -> 3 -> 5
        assert_eq!(list.get(0), Some(0));

        list.insert_at_index(5, 7); 
        // List: 0 -> 1 -> 2 -> 3 -> 5 -> 7
        assert_eq!(list.get(5), Some(7));

        assert_eq!(list.len(), 6);
    }

}