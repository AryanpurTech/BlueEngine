use blue_engine::signal::Signal;

struct Thing {
    total: u64,
}

impl Thing {
    fn get_total(&mut self, things: &mut (Vec<u64>, u8)) -> () {
        for thing in things.0.iter() {
            self.total += thing;
        }
    }

    fn mul_tot(&mut self, stuff: &mut Vec<u64>) -> () {
        for x in stuff.iter() {
            self.total *= x;
        }
    }
}

fn main() {
    let mut a: Signal<Thing, (Vec<u64>, u8)> = Signal::new();
    // You can store objects on the heap or the stack.
    // Both heap & stack pointers can be registered into a delegate.
    let mut heap = Box::new(Thing { total: 300_000 });
    {
        let mut stack = Thing { total: 400_000 };
        // We have lifetime code, if you uncomment the code below it won't compile.
        // This is because the compiler understands the signal outlives the delegate we're trying to bind.
        //a.add(&mut stack, Thing::get_total);
    }
    a.add(&mut heap, Thing::get_total);
    let mut data = (vec![10, 10, 100, 2000], 2);
    a.broadcast(&mut data);
    println!("{}", heap.total);


    // Example code of using single list/struct paramter instead of a tuple.
    let mut b: Signal<Thing, Vec<u64>> = Signal::new();
    b.add(&mut heap, Thing::mul_tot);
    let mut data = vec![10, 10, 10, 10];
    b.broadcast(&mut data);
    println!("{}", heap.total);
}