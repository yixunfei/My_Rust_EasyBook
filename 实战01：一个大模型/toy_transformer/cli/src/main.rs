use clap::Parser;
use ndarray::s;
use rand::SeedableRng;
use toy_transformer_core::{ModelConfig, ToyTransformer};

#[derive(Parser, Debug)]
#[command(name = "toy-transformer-cli", about = "Toy Transformer CLI: tiny inference with configurable hyper-parameters. ")]
struct Cli {
    #[arg(long, value_parser, help = "Hidden dimension size")]
    d_model: usize,
    #[arg(long, help = "FFN inner size")]
    hidden: usize,
    #[arg(long, help = "Number of transformer layers")]
    layers: usize,
    #[arg(long, help = "Number of attention heads")]
    heads: usize,
    #[arg(long, help = "Max position embeddings")]
    max_pos: usize,
    #[arg(long, value_parser, help = "Random seed")]
    seed: u64,
    #[arg(long, help = "Input tokens separated by spaces")]
    input: String,
    #[arg(long, default_value_t = 3, help = "Top-k predictions to display")]
    topk: usize,
}

fn main() {
    let cli = Cli::parse();

    let config = ModelConfig {
        d_model: cli.d_model,
        hidden: cli.hidden,
        layers: cli.layers,
        heads: cli.heads,
        max_pos: cli.max_pos,
        vocab: Vec::new(),
        seed: Some(cli.seed),
    };

    // 构建模型
    let mut rng = rand::rngs::StdRng::seed_from_u64(cli.seed);
    let model = ToyTransformer::new(&config, &mut rng);

    // input tokens
    let input_tokens: Vec<String> = cli.input.split_whitespace().map(|s| s.to_string()).collect();
    let mut input_ids: Vec<usize> = Vec::with_capacity(input_tokens.len());
    for (tok) in input_tokens.iter() {
        let id = model.token_id(tok.as_str()).unwrap_or(0);
        input_ids.push(id);
    }

    // inference
    let logits = model.infer_logits(&input_ids);
    let seq_len = input_ids.len();
    if seq_len == 0 { println!("No input tokens"); return; }
    let last = seq_len - 1;
    let last_logits = logits.slice(s![last, ..]).to_owned();
    // top-k
    let mut top: Vec<(usize, f32)> = last_logits.indexed_iter().map(|(i, &v)| (i, v)).collect();
    top.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("Input tokens: {:?}", input_tokens);
    println!("Top-{} predictions:", cli.topk);
    for i in 0..cli.topk {
        let (idx, score) = top[i];
        let token = model.vocab().get(idx).map(|s| s.as_str()).unwrap_or("<unk>");
        println!("  {} (id {}), score {:.6}", token, idx, score);
    }
}
