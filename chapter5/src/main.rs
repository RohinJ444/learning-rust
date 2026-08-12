// Only primitives implement the Display trait by default, which tells Rust how to print them. 
// the  command below enables the struct to be printed by a predetermined convention in the Rust Debug trait.
#[derive(Debug)] 
struct RectPrism {
    width: u32,
    height: u32,
    depth: u32,
}

impl RectPrism {
    fn sfc_area(&self) -> u32 {
        2 * ((self.width * self.height) + (self.height * self.depth) + (self.depth * self.width))
    }

    fn can_hold(&self, other: &RectPrism) -> bool {
        return self.width > other.width &&
               self.height > other.height &&
               self.depth > other.depth;
    }
}

fn main() {
    println!("Hello, world!");

    let rp = RectPrism {
        width: 8,
        height: 10,
        depth: 12
    };

    println!("The volume of the rectangular prism is {}", volume(&rp));

    println!("{rp:?}");

    println!(
        "The surface area of the rectanglur prism is {}.",
        rp.sfc_area()
    );

    let rp2 = RectPrism {
        width: 10,
        height: 12,
        depth: 14,
    };

    println!("{}", rp2.can_hold(&rp));
    println!("{}", rp.can_hold(&rp2));
}

fn volume (rect_prism: &RectPrism) -> u64 {
    let vol: u64 = (rect_prism.width * rect_prism.height * rect_prism.depth).into();
    return vol;
}