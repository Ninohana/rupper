## Rupper

驱动级模拟键盘操作

> 导入项目后需要将cper.lib放在根目录才能正常编译。

> 运行时需要和cper.dll在同一目录下

> [Cper（站内跳转）](https://github.com/Ninohana/Cper)

## 测试代码

```rust
use rupper::keyboard;
use rupper::keyboard::{Keycode, Modifier};
use std::thread;
use std::time::Duration;

fn main() {
    keyboard::press_key(vec![Keycode::A], Modifier::None);
    thread::sleep(Duration::new(1, 0));
    keyboard::release_key();
}
```
