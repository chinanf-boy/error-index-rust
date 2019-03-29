// 简单而健壮 错误处理 error-chain!
// 使用这个作为新项目的模版

// `error_chain!` 可以递归得很深
#![recursion_limit = "1024"]

// 导入宏. 不要忘了在你的
// `Cargo.toml`，添加 `error-chain`!
#[macro_use]
extern crate error_chain;

// 我们会把错误放到 `errors` 模块, 同个crate的其他模块通过 `use errors::*;` ，获取到
// `error_chain!` creates 所制作的一切。
mod errors {
    // 创建 Error, ErrorKind, ResultExt, 和 Result 类型
    error_chain!{}
}

// 在这模块内，唯一的访问点.  如果该类型必须 从其他模块 访问，
// 用`pub use errors::*;` 替代 (例如, 在
// 一个 `links` 部分里面).
use errors::*;

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // 这个回溯(backtrace) 不会总生成。 试试用`RUST_BACKTRACE=1`
        // 运行下这个例子文件.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

// 上面的 main 给你最大的控制错误格式的权限. 
// 如果你不关心这些 (或者. 你想甩出完整的错误
// ) 你只要在错误对象上，调用 `display_chain` 方法就可以了
#[allow(dead_code)]
fn alternative_main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        use error_chain::ChainedError; // 助力 `display_chain` 的 trait
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "{}", e.display_chain()).expect(errmsg);
        ::std::process::exit(1);
    }
}

// 这宏会自动处理上面的 main。也许你想设个`RUST_BACKTRACE`环境变量
// 看看回溯。
// quick_main!(run);


// 多数函数会返回`Result` 类型, 从
// `errors` 模块导入。 标准 `Result`类型的错误变种
// 终将是我们自己的`Error`。
fn run() -> Result<()> {
    use std::fs::File;

    // This operation will fail
    File::open("tretrete")
        .chain_err(|| "unable to open tretrete file")?;

    Ok(())
}