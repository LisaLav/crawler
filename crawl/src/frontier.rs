use queues::*;


//front queue contains tuples (url, fingerprint) for easy prioritisation and such
pub fn createFrontQueue() -> Queue<(String,String)>{
    let mut q: Queue<(String,String)> = Queue::new();
    q.add((String::from("a"),String::from("bbb")));
    q.add((String::from("b"),String::from("ccc")));
    q
    //println!("{}", q.remove().unwrap());
    //println!("{}", q.remove().unwrap());

}

pub fn createBackQueue(){
    let mut q: Queue<&str> = Queue::new();
    q.add("a");
    q.add("b");
    println!("{}", q.remove().unwrap());
    println!("{}", q.remove().unwrap());

}

pub fn prioritise(url:(&str,&str), newFingerprint:&str, mut currPriority:i32){
    //if url hasnt changed, decrease priority by 0.1, stopping at 0.1 priority
    let (urlLink,oldFingerprint) = url;

    if (oldFingerprint == newFingerprint && currPriority > 1){
        currPriority -= 1;
    }

    //need to put url with updated fingerprint into correct queue now

}