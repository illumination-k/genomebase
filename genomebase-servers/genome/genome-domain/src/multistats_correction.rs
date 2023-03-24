pub fn fdr(p_values: &[f64]) -> Vec<f64> {
    let n = p_values.len();
    let mut fdr_values = vec![0.; n];
    let mut indexes: Vec<usize> = (0..n).collect();

    // インデックスをp値でソート
    indexes.sort_by(|&i, &j| p_values[i].partial_cmp(&p_values[j]).unwrap());

    // FDR値を計算
    for i in 0..n {
        let rank = i + 1;
        let q_value = p_values[indexes[i]] * n as f64 / rank as f64;
        fdr_values[indexes[i]] = q_value;
    }

    fdr_values
}
