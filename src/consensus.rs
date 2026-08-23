use std::collections::HashMap;

#[derive(Debug,Clone,Copy,PartialEq,Eq)] pub enum Role {Follower,Candidate,Leader}
#[derive(Debug)] pub struct Node { pub id:u64, pub role:Role, pub term:u64, pub votes:u64, pub alive:bool }

impl Node { pub fn new(id:u64)->Self{Self{id,role:Role::Follower,term:0,votes:0,alive:true}} pub fn start_election(&mut self){if !self.alive{return} self.term+=1;self.role=Role::Candidate;self.votes=1;} pub fn receive_vote(&mut self,term:u64){if term==self.term && self.role==Role::Candidate {self.votes+=1;}} pub fn quorum(&self,total:usize)->bool{self.votes as usize > total/2} }

pub fn elect(nodes:&mut HashMap<u64,Node>)->Option<u64>{ let total=nodes.len(); for n in nodes.values_mut(){n.start_election(); for _ in 1..total {n.receive_vote(n.term);} if n.quorum(total){n.role=Role::Leader; return Some(n.id);} } None }
