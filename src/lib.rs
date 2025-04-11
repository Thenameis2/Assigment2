use std::fmt::Debug;

pub struct Node<T> 
where
    T: PartialEq + Debug + Clone,
{
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct DynamicLinkedList<T> 
where
    T: PartialEq + Debug + Clone,
{
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> DynamicLinkedList<T> 
where
    T: PartialEq + Debug + Clone,
{
    pub fn new() -> Self {
        DynamicLinkedList {
            head: None,
            size: 0,
        }
    }
    
    pub fn insert(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: None,
        });
    
        let mut current = &mut self.head;
    
        while let Some(node) = current {
            current = &mut node.next;
        }
    
        *current = Some(new_node);
        self.size += 1;
    }

    pub fn insert_at_index(&mut self, index: usize, data: T) {
        if index > self.size {
            panic!("Index out of bounds");
        }

        let mut new_node = Box::new(Node { data, next: None });

        if index == 0 {
            new_node.next = self.head.take();
            self.head = Some(new_node);
        } else {
            let mut current = &mut self.head;
            for _ in 0..index - 1 {
                if let Some(node) = current {
                    current = &mut node.next;
                }
            }

            if let Some(node) = current {
                new_node.next = node.next.take();
                node.next = Some(new_node);
            }
        }

        self.size += 1;
    }

    /// Deletes the first occurrence of the specified `data` in the linked list.
    /// # Parameters
    /// - `data`: The value to be deleted from the list.
    ///
    /// # Returns
    /// - `true` if the element was successfully deleted from the list.
    /// - `false` if the element was not found in the list.

    pub fn delete_element(&mut self, data: T) -> bool {
        if self.head.is_none() {
            return false;
        }
        
        if let Some(head_node) = &self.head {
            if head_node.data == data {
                
                let mut old_head = self.head.take().unwrap();
                self.head = old_head.next.take();
                self.size -= 1;
                return true;
            }
        }
        
     
        let mut current = &mut self.head;
        
        while let Some(node) = current {
           
            if let Some(next_node) = &node.next {
                if next_node.data == data {
                    
                    let mut next = node.next.take().unwrap();
                    node.next = next.next.take();
                    self.size -= 1;
                    return true;
                }
            }
            
            
            current = &mut node.next;
        }
        
        false
    }

    /// Deletes the node at the specified `index` in the linked list.
    ///
    /// # Parameters
    /// - `index`: The index of the node to be deleted.
    ///
    /// # Returns
    /// - `true` if the node at the specified index was successfully deleted.
    /// - `false` if the index is out of bounds or the node could not be deleted.
    ///
    pub fn delete_at_index(&mut self, index: usize) -> bool {
        if index >= self.size {
            return false;
        }

        if index == 0 {
            if let Some(mut head_node) = self.head.take() {
                self.head = head_node.next.take();
                self.size -= 1;
            }
            return true;
        }

        let mut current = &mut self.head;
        for _ in 0..index - 1 {
            if let Some(node) = current {
                current = &mut node.next;
            }
        }

        if let Some(node) = current {
            if let Some(mut target_node) = node.next.take() {
                node.next = target_node.next.take();
                self.size -= 1;
                return true;
            }
        }

        false
    }

    /// Updates the first occurrence of `old_data` with `new_data` in the linked list.
    /// 
    /// # Parameters
    /// - `old_data`: The data to be replaced in the list.
    /// - `new_data`: The data that will replace the `old_data`.
    ///
    /// # Returns
    /// - `true` if the `old_data` was found and updated with `new_data`.
    /// - `false` if the `old_data` was not found in the list.
    ///

    pub fn update_element(&mut self, old_data: T, new_data: T) -> bool {
        let mut current = &mut self.head;

        while let Some(node) = current {
            if node.data == old_data {
                node.data = new_data; 
                return true;
            }
            current = &mut node.next;
        }

        false 
    }

    pub fn update_element_at_index(&mut self, index: usize, data: T) -> bool {
        if index >= self.size {
            return false; 
        }
    
        let mut current = &mut self.head;
        let mut current_index = 0;
    
        while let Some(node) = current {
            if current_index == index {
                node.data = data; 
                return true;
            }
            current = &mut node.next;
            current_index += 1;
        }
    
        false
    }

    pub fn find(&self, data: T) -> bool {
        todo!("Not Implemented");
    }

    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        }

        let mut current = &self.head;
        let mut current_index = 0;

        while let Some(node) = current {
            if current_index == index {
                return Some(node.data.clone());
            }
            current = &node.next;
            current_index += 1;
        }

        None
    }
    
    pub fn len(&self) -> usize {
        self.size
    }
    
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

mod test;
