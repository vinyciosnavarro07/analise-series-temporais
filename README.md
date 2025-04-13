# **TimeWise Analytics**

## **Descrição do Projeto**
O **TimeWise Analytics** é uma aplicação desenvolvida em Rust para análise de séries temporais. Ele permite importar dados em formatos CSV ou JSON, realizar análises exploratórias, implementar modelos de previsão baseados em regressão linear, calcular métricas de avaliação e gerar relatórios detalhados em PDF, além de gráficos para visualização.

---

## **Funcionalidades**
1. **Importação de Dados**:
   - Suporte para arquivos nos formatos **CSV** e **JSON**.
   - Estrutura de dados compatível com séries temporais (`timestamp` e `value`).

2. **Análise Exploratória**:
   - Cálculo de estatísticas descritivas:
     - Média
     - Mediana
     - Desvio padrão
     - Valores mínimos e máximos
   - Geração de gráficos de séries temporais com linha de regressão.

3. **Modelos de Previsão**:
   - Implementação de regressão linear "pura" (sem bibliotecas externas).
   - Previsão de valores futuros com base nos coeficientes da regressão.

4. **Métricas de Avaliação**:
   - Cálculo do **Coeficiente de Determinação (R²)**.
   - Cálculo do **Erro Quadrático Médio (MSE)**.

5. **Geração de Relatórios**:
   - Relatórios em PDF contendo:
     - Estatísticas descritivas.
     - Coeficientes da regressão linear.
     - Métricas de avaliação.
     - Previsões futuras.
   - Gráficos salvos como imagens (`.png`).

---

## **Estrutura do Projeto**
```plaintext
time_wise_analytics/
├── Cargo.toml          # Configuração do projeto e dependências
├── src/
│   ├── lib.rs          # Módulo principal com exportações
│   ├── main.rs         # Ponto de entrada do programa
│   ├── io.rs           # Funções para leitura de arquivos CSV e JSON
│   ├── metrics.rs      # Funções para cálculo de métricas (R², MSE, etc.)
│   ├── regression.rs   # Implementação da regressão linear
│   ├── visualization.rs # Geração de gráficos
│   ├── pdf.rs          # Geração de relatórios em PDF
├── data/
│   ├── [dados.csv](http://_vscodecontentref_/1)       # Exemplo de arquivo CSV
│   ├── dados.json      # Exemplo de arquivo JSON
├── output/
│   ├── relatorio.pdf   # Relatório gerado
│   ├── series_temporais.png # Gráfico gerado
````

## **Como Executar o Projeto**

### ✅ Pré-requisitos
- Rust instalado na máquina.  
  Para instalar, execute:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

---

### 🚀 Passos para Execução

1. **Clone o repositório**:
   ```bash
   git clone https://github.com/vinyciosnavarro07/analise-series-temporais.git
   cd analise-series-temporais
   ```

2. **Compile o projeto**:
   ```bash
   cargo build
   ```

3. **Execute o programa**:
   ```bash
   cargo run
   ```

4. **Insira um arquivo na pasta "data" e aponte o nome do arquivo de dados** (exemplo: `dados.csv` ou `dados.json`) localizado na pasta `data`.

---

### 📦 O programa processará os dados e gerará:

- 📄 Um relatório em PDF: `output/relatorio.pdf`
- 📊 Um gráfico: `output/series_temporais.png`

---

### 📥 Exemplo de Entrada

- **Arquivo CSV** (`data/dados.csv`):
  ```csv
  timestamp,value
  2023-01-01,10.0
  2023-01-02,12.0
  2023-01-03,15.0
  2023-01-04,20.0
  2023-01-05,25.0
  ```

- **Arquivo JSON** (`data/dados.json`):
  ```json
  [
    { "timestamp": "2023-01-01", "value": 10.0 },
    { "timestamp": "2023-01-02", "value": 12.0 },
    { "timestamp": "2023-01-03", "value": 15.0 },
    { "timestamp": "2023-01-04", "value": 20.0 },
    { "timestamp": "2023-01-05", "value": 25.0 }
  ]
  ```

---

### 📤 Exemplo de Saída

#### 📄 Relatório PDF (`output/relatorio.pdf`)
O relatório contém:
- Estatísticas descritivas (média, mediana, desvio padrão, etc.)
- Coeficientes da regressão linear
- Métricas de avaliação (R² e MSE)
- Previsões para os próximos períodos

#### 📈 Gráfico (`output/series_temporais.png`)
- Gráfico da série temporal com a linha de regressão

---

### ✅ Testes

#### 🔬 Testes Unitários

Testes para:
- Regressão linear
- Cálculo de métricas (R², MSE)
- Função de previsão

#### ▶️ Como Executar os Testes

```bash
cargo test
```

#### ✔️ Resultados Esperados

Todos os testes devem passar:
````
running 4 tests
- test tests::test_linear_regression ... ok
- test tests::test_mean_squared_error ... ok
- test tests::test_predict ... ok
- test tests::test_r_squared ... ok

- test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
````
---

### 🛠️ Tecnologias Utilizadas

- **Rust**: Linguagem de programação principal

#### 📦 Crates:
- `csv`: Para leitura de arquivos CSV
- `serde`, `serde_json`: Para manipulação de arquivos JSON
- `printpdf`: Para geração de relatórios em PDF
- `plotters`: Para geração de gráficos

---

### 🤝 Contribuição

Contribuições são bem-vindas! Siga os passos abaixo:

1. Faça um **fork** do repositório.
2. Crie uma **branch** para sua feature:
   ```bash
   git checkout -b minha-feature
   ```
3. Faça o **commit** das suas alterações:
   ```bash
   git commit -m "Minha nova feature"
   ```
4. Envie para o repositório remoto:
   ```bash
   git push origin minha-feature
   ```
5. Abra um **Pull Request**.

---

### 📄 Licença

Este projeto está licenciado sob a licença **MIT**. Consulte o arquivo `LICENSE` para mais detalhes.
