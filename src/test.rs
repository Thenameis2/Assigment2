#[cfg(test)]
mod tests {
   
    use crate::DynamicLinkedList;
    use crate::staticlist::StaticLinkedList;

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


    #[test]
    fn test_update_element_at_index() {
        let mut list = DynamicLinkedList::new();
        
        // Insert elements into the linked list
        list.insert(10);
        list.insert(20);
        list.insert(30);
        
        // Test updating an element at a valid index
        assert!(list.update_element_at_index(1, 25)); // Update element at index 1
        assert_eq!(list.get(1), Some(25)); // Verify the update
        
        // Test updating an element at an invalid index
        assert!(!list.update_element_at_index(5, 15)); // Index 5 is out of bounds
        
        // Ensure other elements are not changed
        assert_eq!(list.get(0), Some(10));
        assert_eq!(list.get(2), Some(30));
    }

    #[test]
    fn dynamic_list_find() {
        let mut list = DynamicLinkedList::new();

        list.insert(10);
        list.insert(20);
        list.insert(30);
        
        // Element exists
        assert!(list.find(10)); 
        assert!(list.find(20)); 
        assert!(list.find(30)); 

        // Element does not exist
        assert!(!list.find(100)); 
        assert!(!list.find(0)); 
    }

    #[test]
    fn test_insert() {
        let mut list = StaticLinkedList::<i32>::new();
        
        list.insert(10);
        list.insert(20);
        list.insert(30);

        assert_eq!(list.len(), 3);
        
      
        assert_eq!(list.nodes[list.head.unwrap()].data, 10);

        let mut result = Vec::new();
        let mut current = list.head;
        
        while let Some(index) = current {
            result.push(list.nodes[index].data);
            current = list.nodes[index].next;
        }

        assert_eq!(result, vec![10, 20, 30]);
    }

    #[test]
    fn test_insert_at_index() {
        let mut list = StaticLinkedList::<i32>::new();

        list.insert(10);
        list.insert(20);
        list.insert(30);

        // Insert 15 at index 1 -> Expected list: 10 -> 15 -> 20 -> 30
        list.insert_at_index(1, 15);

        assert_eq!(list.len(), 4);

        let mut result = Vec::new();
        let mut current = list.head;
        
        while let Some(index) = current {
            result.push(list.nodes[index].data);
            current = list.nodes[index].next;
        }

        assert_eq!(result, vec![10, 15, 20, 30]);

        // Insert 5 at index 0 -> Expected list: 5 -> 10 -> 15 -> 20 -> 30
        list.insert_at_index(0, 5);

        let mut result2 = Vec::new();
        let mut current2 = list.head;
        
        while let Some(index) = current2 {
            result2.push(list.nodes[index].data);
            current2 = list.nodes[index].next;
        }

        assert_eq!(result2, vec![5, 10, 15, 20, 30]);
    }

    #[test]
    fn test_delete_element() {
        let mut list = StaticLinkedList::<i32>::new();

        list.insert(10);
        list.insert(20);
        list.insert(30);
        list.insert(40);

        assert_eq!(list.len(), 4);

        // Delete middle element
        assert!(list.delete_element(20));
        assert_eq!(list.len(), 3);

        let mut result = Vec::new();
        let mut current = list.head;
        
        while let Some(index) = current {
            result.push(list.nodes[index].data);
            current = list.nodes[index].next;
        }

        assert_eq!(result, vec![10, 30, 40]);

        // Delete head element
        assert!(list.delete_element(10));
        assert_eq!(list.len(), 2);

        let mut result2 = Vec::new();
        let mut current2 = list.head;
        
        while let Some(index) = current2 {
            result2.push(list.nodes[index].data);
            current2 = list.nodes[index].next;
        }

        assert_eq!(result2, vec![30, 40]);

        // Delete non-existing element
        assert_eq!(list.delete_element(100), false);
    }

    #[test]
    fn test_delete_at_index() {
        let mut list = StaticLinkedList::<i32>::new();

        list.insert(10);
        list.insert(20);
        list.insert(30);
        list.insert(40);

        // Delete at index 1 (Expected list: 10 -> 30 -> 40)
        let deleted = list.delete_at_index(1);
        assert!(deleted);
        assert_eq!(list.len(), 3);

        let mut result = Vec::new();
        let mut current = list.head;
        while let Some(index) = current {
            result.push(list.nodes[index].data);
            current = list.nodes[index].next;
        }

        assert_eq!(result, vec![10, 30, 40]);

        // Delete at index 0 (Expected list: 30 -> 40)
        let deleted = list.delete_at_index(0);
        assert!(deleted);
        assert_eq!(list.len(), 2);

        let mut result2 = Vec::new();
        let mut current2 = list.head;
        while let Some(index) = current2 {
            result2.push(list.nodes[index].data);
            current2 = list.nodes[index].next;
        }

        assert_eq!(result2, vec![30, 40]);
    }

    #[test]
    fn test_update_element() {
        let mut list = StaticLinkedList::<i32>::new();

        list.insert(10);
        list.insert(20);
        list.insert(30);

        // Update 20 to 25
        let updated = list.update_element(20, 25);
        assert!(updated);

        let mut result = Vec::new();
        let mut current = list.head;

        while let Some(index) = current {
            result.push(list.nodes[index].data);
            current = list.nodes[index].next;
        }

        assert_eq!(result, vec![10, 25, 30]);

        // Try to update a non-existing element
        let updated_false = list.update_element(100, 200);
        assert!(!updated_false);
    }

    #[test]
    fn teststatic_update_element_at_index() {
        let mut list = StaticLinkedList::<i32>::new();

        list.insert(10);
        list.insert(20);
        list.insert(30);

        // Update element at index 1
        let updated = list.update_element_at_index(1, 25);
        assert!(updated);

        let mut result = Vec::new();
        let mut current = list.head;

        while let Some(index) = current {
            result.push(list.nodes[index].data);
            current = list.nodes[index].next;
        }

        // Check that the element at index 1 is updated to 25
        assert_eq!(result, vec![10, 25, 30]);

        // Try updating an element at an out-of-bounds index
        let updated_false = list.update_element_at_index(5, 50);
        assert!(!updated_false);
    }

    #[test]
    fn test_find() {
        let mut list = StaticLinkedList::<i32>::new();

        list.insert(10);
        list.insert(20);
        list.insert(30);

        // Test finding an element that exists
        assert!(list.find(20));

        // Test finding an element that does not exist
        assert!(!list.find(40));
    }

    #[test]
    fn test_get() {
        let mut list = StaticLinkedList::<i32>::new();

        list.insert(10);
        list.insert(20);
        list.insert(30);

        // Test getting valid indices
        assert_eq!(list.get(0), Some(10));
        assert_eq!(list.get(1), Some(20));
        assert_eq!(list.get(2), Some(30));

        // Test getting an out-of-bounds index
        assert_eq!(list.get(3), None);
    }









}