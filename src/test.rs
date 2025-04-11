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

    #[test]
    fn dynamic_list_delete_element() {
        let mut list = DynamicLinkedList::new();

        list.insert(1);
        list.insert(2);
        list.insert(3);
        list.insert(4);
        // List: 1 -> 2 -> 3 -> 4

        assert_eq!(list.delete_element(3), true);
        // List after deletion: 1 -> 2 -> 4
        assert_eq!(list.get(0), Some(1));
        assert_eq!(list.get(1), Some(2));
        assert_eq!(list.get(2), Some(4));
        assert_eq!(list.len(), 3);

        assert_eq!(list.delete_element(1), true);
        // List after deletion: 2 -> 4
        assert_eq!(list.get(0), Some(2));
        assert_eq!(list.get(1), Some(4));
        assert_eq!(list.len(), 2);

        assert_eq!(list.delete_element(100), false); // Element not in list
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn dynamic_list_delete_at_index() {
        let mut list = DynamicLinkedList::new();

        list.insert(1);
        list.insert(2);
        list.insert(3);
        list.insert(4);
        // List: 1 -> 2 -> 3 -> 4

        // Deleting at index 2 (third element)
        assert!(list.delete_at_index(2));
        assert_eq!(list.len(), 3);
        assert_eq!(list.get(0), Some(1));
        assert_eq!(list.get(1), Some(2));
        assert_eq!(list.get(2), Some(4));

        // Deleting at index 0 (head)
        assert!(list.delete_at_index(0));
        assert_eq!(list.len(), 2);
        assert_eq!(list.get(0), Some(2));
        assert_eq!(list.get(1), Some(4));

        // Deleting at an out-of-bounds index
        assert!(!list.delete_at_index(5));
        assert_eq!(list.len(), 2);

        // Deleting the last element (index 1)
        assert!(list.delete_at_index(1));
        assert_eq!(list.len(), 1);
        assert_eq!(list.get(0), Some(2));

        // Deleting the only remaining element (index 0)
        assert!(list.delete_at_index(0));
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn dynamic_list_update_element() {
        let mut list = DynamicLinkedList::new();
        
        // Insert elements into the list
        list.insert(5);
        list.insert(10);
        list.insert(15);

        // Test updating an element that exists
        assert_eq!(list.update_element(10, 20), true); // Successfully update 10 to 20
        assert_eq!(list.get(1), Some(20)); // Verify the update at index 1

        // Test updating an element that does not exist
        assert_eq!(list.update_element(30, 40), false); // Element not found

        // Verify that the list size is unchanged after a failed update
        assert_eq!(list.len(), 3); // The size should still be 3
    }




}