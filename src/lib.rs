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
