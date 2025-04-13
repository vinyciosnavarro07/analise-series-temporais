use std::io;
use time_wise_analytics::{
    read_file, linear_regression, r_squared, mean_squared_error, predict, plot_series_and_regression,
    mean, median, standard_deviation, min, max, generate_pdf,
};

fn main() {
    println!("===============================");
    println!("🎉 Bem-vindo ao TimeWise Analytics! 🎉");
    println!("===============================\n");

    println!("📂 Por favor, insira um arquivo na pasta 'data' e aponte o nome do arquivo (CSV ou JSON)':");
    println!("👉 Exemplo: dados.csv ou dados.json\n");


    // Solicita o nome do arquivo ao usuário
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).expect("Erro ao ler o nome do arquivo.");
    let file_name = file_name.trim(); // Remove espaços em branco e quebras de linha

    let file_path = format!("data/{}", file_name);

    println!("📊 Processando os dados...\n");

    // Lê os dados do arquivo
    let data = match read_file(&file_path) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("❌ Erro ao ler o arquivo '{}': {}", file_name, err);
            return;
        }
    };

    // Extrai os valores de timestamp e valores numéricos
    let x: Vec<f64> = (0..data.len()).map(|i| i as f64).collect();
    let y: Vec<f64> = data.iter().map(|record| record.value).collect();

    // Calcula estatísticas descritivas
    let mean_value = mean(&y).unwrap_or(0.0);
    let median_value = median(&mut y.clone()).unwrap_or(0.0);
    let std_dev = standard_deviation(&y).unwrap_or(0.0);
    let min_value = min(&y).unwrap_or(0.0);
    let max_value = max(&y).unwrap_or(0.0);

    // Realiza a regressão linear
    if let Some((a, b)) = linear_regression(&x, &y) {
        let r2 = r_squared(&x, &y, a, b).unwrap_or(0.0);
        let mse = mean_squared_error(&x, &y, a, b).unwrap_or(0.0);

        let future_x: Vec<f64> = (x.len()..x.len() + 5).map(|i| i as f64).collect();
        let predictions = predict(&future_x, a, b);

        // Gera o PDF com os resultados
        if let Err(err) = generate_pdf(
            "output/relatorio.pdf",
            mean_value,
            median_value,
            std_dev,
            min_value,
            max_value,
            a,
            b,
            r2,
            mse,
            &predictions,
        ) {
            eprintln!("❌ Erro ao gerar o PDF: {}", err);
        } else {
            println!("✅ Relatório salvo em 'output/relatorio.pdf'");
        }

        // Gera o gráfico
        if let Err(err) = plot_series_and_regression(&x, &y, a, b, "output/series_temporais.png") {
            eprintln!("❌ Erro ao gerar o gráfico: {}", err);
        } else {
            println!("📈 Gráfico salvo em 'output/series_temporais.png'");
        }
    } else {
        println!("❌ Erro: Não foi possível calcular a regressão linear.");
    }
    println!("Obrigado por usar o TimeWise Analytics! 🚀");
}