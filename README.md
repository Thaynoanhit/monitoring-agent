# Sistema de Monitoramento em Tempo Real

Sistema de Monitoramento em Tempo Real
Um sistema de monitoramento de recursos do sistema desenvolvido em Rust e React, oferecendo visualização em tempo real de métricas de CPU, memória, disco e threads.

# 🔧 Pré-requisitos

* Instalação do Rustmonitoramento
* Instale o Rust usando rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh   
```

Para Windows, baixe e execute o rustup-init.exe:

```bash
https://rustup.rs 
```

Verifique a instalação:

```bash
rustc --version
cargo --version
```

Dependências do Sistema 

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install build-essential pkg-config libssl-dev

# Fedora
sudo dnf install gcc pkg-config openssl-devel

# macOS
xcode-select --install
```

# 🚀 Funcionalidades

* Monitoramento em tempo real de recursos do sistema
* Interface gráfica interativa
* Comunicação WebSocket
* Coleta de métricas detalhadas
* Sistema de alertas configurável

# 🛠️ Tecnologias

* Backend: Rust
* Tokio para async runtime
* Sysinfo para coleta de métricas
* Warp para WebSocket
* Serde para serialização
* Frontend: React
* TypeScript
* Chart.js para visualizações
* WebSocket para comunicação em tempo real

# 🏃 Como Executar
* Backend
* Clone o repositório
   ```bash
   # Clone e execute o backend primeiro:
   git clone https://github.com/Thaynoanhit/monitoring-agent
   cd monitoring-agent
   
   
Configure as variáveis de ambiente
```bash
cp .env.example .env
```

Execute o servidor 
```bash
cargo clean
cargo update
cargo build
cargo run
```

* Frontend
  
  Navegue até o diretório do frontend
```bash
cd src/monitoring-frontend/src
```
Instale as dependências
```bash
npm install
```

Execute o frontend
```bash
npm start   
```

# 📈 Interface de Visualização

A interface web apresenta:

* Gráficos em tempo real
* Métricas detalhadas
* Alertas configuráveis
* Atualizações automáticas

# 🔧 Configuração
Variáveis de ambiente disponíveis:

MAX_METRICS=1000

COLLECT_INTERVAL=10

MONITORING_TOKEN=my_secure_token

# 🤝 Contribuindo

* Faça um fork do projeto
* Crie uma branch para sua feature (git checkout -b feature/AmazingFeature)
* Commit suas mudanças (git commit -m 'Add some AmazingFeature')
* Push para a branch (git push origin feature/AmazingFeature)
* Abra um Pull Request
