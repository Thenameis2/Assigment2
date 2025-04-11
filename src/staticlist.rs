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

    
    pub fn delete_element(&mut self, data: T) -> bool {
        todo!("Not Implemented");
    }

   
    pub fn delete_at_index(&mut self, index: usize) -> bool {
        todo!("Not Implemented");
    }

   
    pub fn update_element(&mut self, old_data: T, new_data: T) -> bool {
        todo!("Not Implemented");
    }

  
    pub fn update_element_at_index(&mut self, index: usize, data: T) -> bool {
        todo!("Not Implemented");
    }

   
    pub fn find(&self, data: T) -> bool {
        todo!("Not Implemented");
    }

   
    pub fn get(&self, index: usize) -> Option<T> {
        todo!("Not Implemented");
    }
    
  
    pub fn len(&self) -> usize {
        self.size
    }
    
 
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

