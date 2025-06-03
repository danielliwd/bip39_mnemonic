use bip39::Mnemonic;
use rand::Rng;
use clap::Parser;

/// 生成 BIP39 标准助记词工具
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 助记词单词数量 (12, 15, 18, 21 或 24)
    #[arg(short, long, default_value_t = 12)]
    word_count: usize,
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}

fn generate_mnemonic(word_count: usize) -> String {
    // 根据单词数量确定熵长度
    let entropy_len = match word_count {
        12 => 16,  // 128 bits
        15 => 20,  // 160 bits
        18 => 24,  // 192 bits
        21 => 28,  // 224 bits
        24 => 32,  // 256 bits
        _ => panic!("不支持的助记词长度，必须是12,15,18,21或24"),
    };
    
    let mut rng = rand::rng();
    let entropy: Vec<u8> = (0..entropy_len).map(|_| rng.random()).collect();
    
    Mnemonic::from_entropy(&entropy)
        .expect("生成助记词失败")
        .to_string()
}

fn main() {
    let args = Args::parse();

    // 验证参数有效性
    if ![12, 15, 18, 21, 24].contains(&args.word_count) {
        eprintln!("错误: 单词数量必须是12,15,18,21或24");
        std::process::exit(1);
    }

    let mnemonic = generate_mnemonic(args.word_count); // 生成12个单词的助记词
    if args.quiet {
        // 静默模式：只输出助记词
        println!("{}", mnemonic);
    } else {
        println!("生成的助记词: {}", mnemonic);

        // 可选：显示助记词对应的种子
        let seed = Mnemonic::parse(mnemonic)
            .expect("解析助记词失败")
            .to_seed("");
        println!("种子 (前16字节): {:?}", &seed[..16]);
    }
}
