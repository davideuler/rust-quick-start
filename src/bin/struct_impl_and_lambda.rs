// cargo add rand
// rustc src/struct_trait_and_impl.rs  --extern rand=target/debug/deps/librand-ddb5b4867168c7f4.rlib -L dependency=target/debug/deps
// https://juejin.cn/post/7042571788349341703
extern crate rand;

use std::rc::Rc;
use std::cell::RefCell;

use rand::Rng;


// 使用Box来表示接受单个u32参数的闭包
type OnReceivedDamage = Box<dyn Fn(u32)>;

struct Monster {
    health: u32,
    received_damage: Vec<OnReceivedDamage>,
}

// struct 的实现方法：
impl Monster {
    fn take_damage(&mut self, amount: u32) {
        let damage_received = std::cmp::min(self.health, amount);
        self.health -= damage_received;
        for callback in &mut self.received_damage {
            callback(damage_received);
        }
    }

    fn add_listener(&mut self, listener: OnReceivedDamage) {
        self.received_damage.push(listener);
    }
}

impl Default for Monster {
    fn default() -> Self {
        Monster { health: 100, received_damage: Vec::new() }
    }
}

#[derive(Default)]
struct DamageCounter {
    damage_inflicted: u32,
}

impl DamageCounter {
    fn reached_target_damage(&self) -> bool {
        self.damage_inflicted > 100
    }

    fn on_damage_received(&mut self, damage: u32) {
        self.damage_inflicted += damage;
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    // let mut counter = DamageCounter::default();
    let counter = Rc::new(RefCell::new(DamageCounter::default()));
    let mut monsters: Vec<_> = (0..5).map(|_| Monster::default()).collect();

    for monster in &mut monsters {
        // cannot borrow `counter` as mutable, as it is a captured variable in a `Fn` closure
        // monster.add_listener(Box::new(|damage| counter.on_damage_received(damage)));
        let counter = Rc::clone(&counter);
        monster.add_listener(Box::new(move |damage| {
             counter.borrow_mut().on_damage_received(damage)
        }));

    }

    //while !counter.reached_target_damage() {
    while !counter.borrow().reached_target_damage() {
        let index = rng.gen_range(0..monsters.len());
        let target = &mut monsters[index];

        let damage = rng.gen_range(0..50);
        target.take_damage(damage);

        println!("Monster {} received {} damage", index, damage);
    }
}
