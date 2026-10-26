use core::iter::zip;

use alloc::vec::{Vec};
use alloc::vec;

/// A deadlock detector
pub struct Detect{
    available: Vec<u16>,
    allocation: Vec<Vec<u16>>,
    need: Vec<Vec<u16>>
}


// https://chat.deepseek.com/share/hivpfcktijizihx5kw

impl Detect{
    /// New detector
    pub fn new() -> Self{
        Self {
            available: Vec::new(),
            allocation: vec![Vec::new()],
            need: vec![Vec::new()],
        }
    }
    /// Add resource

    pub fn add_resource(&mut self,count: u16){
        self.available.push(count);
        self.allocation
            .iter_mut()
            .for_each(|val| val.push(0));
        self.need
            .iter_mut()
            .for_each(|val| val.push(0));
    }

    /// Add thread
    pub fn add_thread(&mut self){
        self.allocation.push(vec![0;self.available.len()]);
        self.need.push(vec![0;self.available.len()]);
    }

    /// Alloc a lock to pending list
    pub fn alloc_pending(&mut self,thread: usize,resource: usize){
        self.need[thread][resource] += 1;
    }

    /// Remove lock from pending list and alloc actually
    pub fn update_pending(&mut self,thread: usize,resource: usize){
        self.need[thread][resource] -= 1;
        self.available[resource] -= 1;
        self.allocation[thread][resource] += 1;
    }

    /// Dealloc a lock
    pub fn dealloc(&mut self,thread: usize,resource: usize){
        self.available[resource] += 1;
        self.allocation[thread][resource] -= 1;
    }

    /// Drop a thread's allocation
    pub fn drop_thread(&mut self,thread: usize){
        self.allocation[thread]
        .iter_mut()
        .enumerate()
        .for_each(|(index,val)| {
            self.available[index] += *val;
            *val = 0
        });
    }



    /// Test if can alloc
    pub fn test_alloc(&mut self,thread: usize,resource: usize) -> bool{
        let mut work = self.available.clone();
        self.need[thread][resource] += 1;
        let mut finish = vec![false;self.allocation.len()];
        'exit: loop{
            let finish_id: Vec<usize> = 
                finish
                .iter()
                .enumerate()
                .filter(|(index,val)| **val == false && zip(self.need[*index].iter(),work.iter()).all(|(need,work)| work >= need))
                .map(|(index,_)| index)
                .collect();
            let len = finish_id.len();
            if len == 0{
                break 'exit;
            }
            for id in finish_id{
                for (index,val) in self.allocation[id].iter().enumerate(){
                    work[index] += val;
                }
                finish[id] = true;
            }
        }
        self.need[thread][resource] -= 1;
        finish.iter().all(|val| *val)
    }
}