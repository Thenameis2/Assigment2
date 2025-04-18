use std::fmt::Debug;

const MAX_SIZE: usize = 100;

#[derive(Debug)]
pub struct Node<T>
where
    T: PartialEq + Debug + Clone + Default,
{
    pub data: T,
    pub next: Option<usize>,
}


pub struct StaticLinkedList<T> 
where
    T: PartialEq + Debug + Clone + Default,
{
    pub nodes: [Node<T>; MAX_SIZE], 
    pub head: Option<usize>,
    pub free_list: Option<usize>,
    pub size: usize,
}


impl<T> StaticLinkedList<T> 
where
    T: PartialEq + Debug + Clone + Default,
{
    
    pub fn new() -> Self {
        let mut nodes = Vec::with_capacity(MAX_SIZE);
        
     
        for i in 0..MAX_SIZE-1 {
            nodes.push(Node {
                data: T::default(),
                next: Some(i + 1),
            });
        }
        
      
        nodes.push(Node {
            data: T::default(),
            next: None,
        });
        
        StaticLinkedList {
            nodes: nodes.try_into().unwrap(),
            head: None,
            free_list: Some(0), 
            size: 0,
        }
    }
    
    /// Inserts a new element at the end (tail) of the linked list.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to insert into the list.
    pub fn insert(&mut self, data: T) {

        if self.free_list.is_none() {
            panic!("StaticLinkedList is full, cannot insert more elements");
        }

        
        let new_index = self.free_list.unwrap();
        self.free_list = self.nodes[new_index].next; 

        self.nodes[new_index].data = data;
        self.nodes[new_index].next = None;

        
        if self.head.is_none() {
            self.head = Some(new_index);
        } else {
        
            let mut current = self.head.unwrap();
            while let Some(next_index) = self.nodes[current].next {
                current = next_index;
            }
            self.nodes[current].next = Some(new_index);
        }

        self.size += 1;
    }

   
    /// Inserts a new element at a specific index in the linked list.
    ///
    /// # Arguments
    ///
    /// * `index` - The position where the new element should be inserted (0-based).
    /// * `data` - The data to insert into the list.

    pub fn insert_at_index(&mut self, index: usize, data: T) {
        if self.free_list.is_none() {
            panic!("StaticLinkedList is full, cannot insert more elements");
        }
    
        if index > self.size {
            panic!("Index out of bounds");
        }
    
        let new_index = self.free_list.unwrap();
        self.free_list = self.nodes[new_index].next;
        self.nodes[new_index].data = data;
    
        if index == 0 {
            // Insert at head
            self.nodes[new_index].next = self.head;
            self.head = Some(new_index);
        } else {
            // Traverse to node before index
            let mut current = self.head.unwrap();
            for _ in 0..index - 1 {
                current = self.nodes[current].next.unwrap();
            }
    
            self.nodes[new_index].next = self.nodes[current].next;
            self.nodes[current].next = Some(new_index);
        }
    
        self.size += 1;
    }

    /// Deletes the first occurrence of the given data from the linked list.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to delete from the list.
    ///
    /// # Returns
    ///
    /// Returns `true` if the element was found and deleted, otherwise `false`.
    pub fn delete_element(&mut self, data: T) -> bool {
        let mut current = self.head;
        let mut prev: Option<usize> = None;

        while let Some(index) = current {
            if self.nodes[index].data == data {
                // Remove the node
                if let Some(prev_index) = prev {
                    self.nodes[prev_index].next = self.nodes[index].next;
                } else {
                    // Removing head
                    self.head = self.nodes[index].next;
                }

                // Add removed node to free_list
                self.nodes[index].next = self.free_list;
                self.free_list = Some(index);

                self.size -= 1;
                return true;
            }
            prev = current;
            current = self.nodes[index].next;
        }

        false
    }

   /// Deletes the element at a specific index in the linked list.
    ///
    /// # Arguments
    ///
    /// * `index` - The position (0-based) of the element to delete.
    ///
    /// # Returns
    ///
    /// Returns `true` if the element was successfully deleted, otherwise `false`.
    ///
    pub fn delete_at_index(&mut self, index: usize) -> bool {
        if index >= self.size {
            panic!("Index out of bounds");
        }
    
        let mut current = self.head;
        let mut prev: Option<usize> = None;
    
        for _ in 0..index {
            prev = current;
            current = self.nodes[current.unwrap()].next;
        }
    
        if let Some(delete_index) = current {
            if let Some(prev_index) = prev {
                self.nodes[prev_index].next = self.nodes[delete_index].next;
            } else {
                // Delete head
                self.head = self.nodes[delete_index].next;
            }
    
            // Add deleted node to free_list
            self.nodes[delete_index].next = self.free_list;
            self.free_list = Some(delete_index);
    
            self.size -= 1;
            return true;
        }
    
        false
    }

   
    pub fn update_element(&mut self, old_data: T, new_data: T) -> bool {
        let mut current = self.head;

        while let Some(index) = current {
            if self.nodes[index].data == old_data {
                self.nodes[index].data = new_data;
                return true;
            }
            current = self.nodes[index].next;
        }

        false
    }

  
    pub fn update_element_at_index(&mut self, index: usize, data: T) -> bool {
        if index >= self.size {
            return false; 
        }
    
        let mut current = self.head;
    
        for _ in 0..index {
            current = self.nodes[current.unwrap()].next;
        }
    
        if let Some(index_to_update) = current {
            self.nodes[index_to_update].data = data;
            return true;
        }
    
        false
    }

   /// Finds the first occurrence of `data` in the linked list.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to search for.
    ///
    /// # Returns
    ///
    /// Returns `true` if the element is found, otherwise `false`.
    pub fn find(&self, data: T) -> bool {
        let mut current = self.head;

        while let Some(index) = current {
            if self.nodes[index].data == data {
                return true;
            }
            current = self.nodes[index].next;
        }
    
        false
    }

   /// Retrieves the element at the specified index in the linked list.
    ///
    /// # Arguments
    ///
    /// * `index` - The position (0-based) of the element to retrieve.
    ///
    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;  
        }
        
        let mut current = self.head;
        
        for _ in 0..index {
            current = self.nodes[current.unwrap()].next;
        }
        
        current.map(|idx| self.nodes[idx].data.clone())
    }
    
  
    pub fn len(&self) -> usize {
        self.size
    }
    
 
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

