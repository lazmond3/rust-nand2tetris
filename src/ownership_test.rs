use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
struct FancyBox(usize);

impl Display for FancyBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "(inner: {})", self.0)
    }
}

fn say(u: usize) {
    println!("usize: {}", u);
}
fn say_fancy(u: FancyBox) {
    println!("usize: {}", u);
}
fn say_mut_fancy(u: &FancyBox) {
    println!("usize: {}", *u);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_move_multiple() {
        let v: usize = 100;
        // usize プリミティブは Copy に対応してるので、引数がコピーされて渡される。
        say(v);
        say(v);
    }

    #[test]
    fn for_move_say_fancy() {
        let v: FancyBox = FancyBox(100);
        // copy トレイトがないと、move されるのでerrorになる
        say_fancy(v.clone()); // clone しないとエラーになる。
        say_fancy(v.clone());
        say_fancy(v);
    }
    #[test]
    fn for_move_say_mut_fancy() {
        let v: FancyBox = FancyBox(100);
        // 参照で呼び出す場合(mutではない)ときは、参照の値が渡される(?)のでmoveが起こらず、
        // エラーにならない。
        say_mut_fancy(&v);
        say_mut_fancy(&v);
        say_mut_fancy(&v);
    }
}
