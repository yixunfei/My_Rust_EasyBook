use ndarray::{s, Array1, Array2};
use rand::Rng;
use rand_distr::{Normal, Distribution};

#[derive(Clone, Debug)]
pub struct ModelConfig {
    pub d_model: usize,
    pub hidden: usize,
    pub layers: usize,
    pub heads: usize,
    pub max_pos: usize,
    pub vocab: Vec<String>,
    pub seed: Option<u64>,
}

impl ModelConfig {
    pub fn default_vocab() -> Vec<String> {
        vec!["<pad>".to_string(), "<bos>".to_string(), "hello".to_string(), "world".to_string(), "!".to_string()]
    }
}

struct Layer {
    w_q: Array2<f32>, // (d_model, d_model)
    w_k: Array2<f32>,
    w_v: Array2<f32>,
    w_o: Array2<f32>,

    ln1_gamma: Array1<f32>,
    ln1_beta: Array1<f32>,
    ln2_gamma: Array1<f32>,
    ln2_beta: Array1<f32>,

    ffn_w1: Array2<f32>, // (d_model, hidden)
    ffn_b1: Array1<f32>, // (hidden)
    ffn_w2: Array2<f32>, // (hidden, d_model)
    ffn_b2: Array1<f32>, // (d_model)

    d_model: usize,
    num_heads: usize,
    head_dim: usize,
    embedding_dim: usize,
}

impl Layer {
    fn new(d_model: usize, num_heads: usize, hidden: usize, rng: &mut rand::rngs::StdRng) -> Self {
        fn rand_mat(rng: &mut rand::rngs::StdRng, rows: usize, cols: usize) -> Array2<f32> {
            let mut data = vec![0f32; rows * cols];
            let normal = Normal::new(0.0, 0.1).unwrap();
            for i in 0..rows * cols {
                data[i] = normal.sample(rng);
            }
            Array2::from_shape_vec((rows, cols), data).unwrap()
        }
        fn rand_vec(rng: &mut rand::rngs::StdRng, len: usize) -> Array1<f32> {
            let mut data = vec![0f32; len];
            let normal = Normal::new(0.0, 0.1).unwrap();
            for i in 0..len {
                data[i] = normal.sample(rng);
            }
            Array1::from_vec(data)
        }

        Layer {
            w_q: rand_mat(rng, d_model, d_model),
            w_k: rand_mat(rng, d_model, d_model),
            w_v: rand_mat(rng, d_model, d_model),
            w_o: rand_mat(rng, d_model, d_model),

            ln1_gamma: rand_vec(rng, d_model),
            ln1_beta: rand_vec(rng, d_model),
            ln2_gamma: rand_vec(rng, d_model),
            ln2_beta: rand_vec(rng, d_model),

            ffn_w1: rand_mat(rng, d_model, hidden),
            ffn_b1: rand_vec(rng, hidden),
            ffn_w2: rand_mat(rng, hidden, d_model),
            ffn_b2: rand_vec(rng, d_model),

            d_model,
            num_heads,
            head_dim: d_model / num_heads,
            embedding_dim: d_model,
        }
    }

    // Forward: 简化前向：多头注意力 + FFN
    fn forward(&self, x: &Array2<f32>) -> Array2<f32> {
        let (seq_len, d_model) = x.dim();
        let q = x.dot(&self.w_q);
        let k = x.dot(&self.w_k);
        let v = x.dot(&self.w_v);

        let mut head_out = Array2::<f32>::zeros((seq_len, d_model));
        for h in 0..self.num_heads {
            let s = h * self.head_dim;
            let e = (h + 1) * self.head_dim;
            let q_h = q.slice(s![.., s..e]).to_owned();
            let k_h = k.slice(s![.., s..e]).to_owned();
            let v_h = v.slice(s![.., s..e]).to_owned();

            let mut scores = q_h.dot(&k_h.t());
            let scale = (self.head_dim as f32).sqrt();
            scores.map_inplace(|val| *val /= scale);
            let attn = softmax_rows(&scores);
            let head_out_h = attn.dot(&v_h);

            for i in 0..seq_len {
                for j in 0..self.head_dim {
                    head_out[[i, s + j]] = head_out_h[[i, j]];
                }
            }
        }

        let o = head_out.dot(&self.w_o);
        let mut res = x.clone();
        add_assign_2d(&mut res, &o);
        let ln1 = layer_norm(&res, &self.ln1_gamma, &self.ln1_beta, 1e-5);
        let f1 = ln1.dot(&self.ffn_w1) + &self.ffn_b1;
        let f1_gelu = f1.mapv(|v| gelu(v));
        let f2 = f1_gelu.dot(&self.ffn_w2) + &self.ffn_b2;
        let mut res2 = ln1.clone();
        add_assign_2d(&mut res2, &f2);
        layer_norm(&res2, &self.ln2_gamma, &self.ln2_beta, 1e-5)
    }
}

fn gelu(x: f32) -> f32 {
    let t = ((2.0 / std::f32::consts::PI).sqrt()) * (x + 0.044715 * x * x * x);
    0.5 * x * (1.0 + t.tanh())
}

fn add_assign_2d(a: &mut Array2<f32>, b: &Array2<f32>) {
    for ((i, j), val) in a.indexed_iter_mut() {
        *val += b[[i, j]];
    }
}

fn layer_norm(x: &Array2<f32>, gamma: &Array1<f32>, beta: &Array1<f32>, eps: f32) -> Array2<f32> {
    let (n, d) = x.dim();
    let mut out = Array2::<f32>::zeros((n, d));
    for i in 0..n {
        let row = x.row(i);
        let mean = row.mean().unwrap();
        let var = row.var(0.0);
        let std = (var + eps).sqrt();
        for j in 0..d {
            out[[i, j]] = (row[j] - mean) / std * & *gamma.get(j).unwrap_or(&0.0) + beta[j];
        }
    }
    out
}

fn softmax_rows(x: &Array2<f32>) -> Array2<f32> {
    let (rows, cols) = x.dim();
    let mut out = Array2::<f32>::zeros((rows, cols));
    for i in 0..rows {
        let mut max = std::f32::NEG_INFINITY;
        for j in 0..cols { if x[[i, j]] > max { max = x[[i, j]]; } }
        let mut sum = 0f32;
        for j in 0..cols {
            let v = (x[[i, j]] - max).exp();
            out[[i, j]] = v;
            sum += v;
        }
        for j in 0..cols { out[[i, j]] /= sum; }
    }
    out
}

fn sinusoidal_positional_encoding(max_pos: usize, d_model: usize) -> ndarray::Array2<f32> {
    let mut pe = ndarray::Array2::<f32>::zeros((max_pos, d_model));
    for pos in 0..max_pos {
        for i in 0..d_model {
            let div = (2 * (i / 2)) as f32 / d_model as f32;
            let angle = (pos as f32) / (10000f32).powf(div);
            pe[[pos, i]] = if i % 2 == 0 { angle.sin() } else { angle.cos() };
        }
    }
    pe
}

pub struct ToyTransformer {
    vocab: Vec<String>,
    id_of: std::collections::HashMap<String, usize>,
    embedding: Array2<f32>, // (vocab_size, d_model)
    layers: Vec<Layer>,
    lm_w: Array2<f32>, // (d_model, vocab_size)
    lm_b: Array1<f32>, // (vocab_size)
    pos_enc: Array2<f32>, // (max_pos, d_model)
    d_model: usize,
    max_pos: usize,
    num_heads: usize,
    head_dim: usize,
}

impl ToyTransformer {
    pub fn new(config: &ModelConfig, rng: &mut rand::rngs::StdRng) -> Self {
        let vocab = if config.vocab.is_empty() { ModelConfig::default_vocab() } else { config.vocab.clone() };
        let mut id_of = std::collections::HashMap::new(); for (i, w) in vocab.iter().enumerate() { id_of.insert(w.clone(), i); }
        let vocab_size = vocab.len();
        let d_model = config.d_model;
        let embedding = {
            let mut data = vec![0f32; vocab_size * d_model];
            let normal = Normal::new(0.0, 0.1).unwrap();
            for i in 0..data.len() { data[i] = normal.sample(rng); }
            ndarray::Array2::from_shape_vec((vocab_size, d_model), data).unwrap()
        };
        let pos_enc = sinusoidal_positional_encoding(config.max_pos, d_model);
        let mut layers = Vec::new();
        for _ in 0..config.layers { layers.push(Layer::new(d_model, config.heads, config.hidden, rng)); }
        let lm_w = {
            let mut data = vec![0f32; d_model * vocab_size];
            let normal = Normal::new(0.0, 0.1).unwrap();
            for i in 0..data.len() { data[i] = normal.sample(rng); }
            ndarray::Array2::from_shape_vec((d_model, vocab_size), data).unwrap()
        };
        let lm_b = {
            let mut data = vec![0f32; vocab_size];
            let normal = Normal::new(0.0, 0.1).unwrap();
            for i in 0..vocab_size { data[i] = normal.sample(rng); }
            ndarray::Array1::from_vec(data)
        };
        ToyTransformer {
            vocab,
            id_of,
            embedding,
            layers,
            lm_w,
            lm_b,
            pos_enc,
            d_model,
            max_pos: config.max_pos,
            num_heads: config.heads,
            head_dim: config.d_model / config.heads,
        }
    }

    pub fn vocab(&self) -> &Vec<String> { &self.vocab }
    pub fn token_id(&self, token: &str) -> Option<usize> { self.id_of.get(token).copied() }

    fn encode_input(&self, input_ids: &[usize]) -> ndarray::Array2<f32> {
        let seq_len = input_ids.len();
        let mut x = ndarray::Array2::<f32>::zeros((seq_len, self.d_model));
        for (i, &tok) in input_ids.iter().enumerate() {
            let mut row = self.embedding.slice(s![tok, ..]).to_owned();
            let pos = i.min(self.max_pos - 1);
            for d in 0..self.d_model { row[d] += self.pos_enc[[pos, d]]; }
            x.row_mut(i).assign(&row);
        }
        x
    }

    pub fn infer_logits(&self, input_ids: &[usize]) -> ndarray::Array2<f32> {
        let mut x = self.encode_input(input_ids);
        for layer in &self.layers {
            x = layer.forward(&x);
        }
        x.dot(&self.lm_w) + &self.lm_b
    }
}

fn main() { // placeholder to satisfy compiler when compiling as library; CLI uses separate binary
}
