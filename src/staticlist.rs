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
    
  
    pub fn insert(&mut self, data: T) {
        todo!("Not Implemented");
    }

   
    pub fn insert_at_index(&mut self, index: usize, data: T) {
        todo!("Not Implemented");
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

