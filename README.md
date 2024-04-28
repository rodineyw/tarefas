# Gerenciador de Tarefas em Rust

Este é um gerenciador de tarefas desenvolvido em Rust com interface gráfica usando GTK 4. Ele permite adicionar, listar e remover tarefas. É uma aplicação simples para gerenciamento de tarefas diárias.

## Recursos
- Adicionar tarefas com descrição
- Marcar tarefas como concluídas ou pendentes
- Remover tarefas da lista
- Interface gráfica amigável com GTK 4

## Instalação
Para instalar o gerenciador de tarefas, siga as instruções abaixo:

### Linux
1. Instale o Rust e o Cargo no seu sistema.
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Clone o repositório do GitHub:
   ```bash
   git clone https://github.com/SEU_USUARIO/GERENCIADOR_DE_TAREFAS.git
   ```
3. Acesse o diretório do projeto e compile:
   ```bash
   cd GERENCIADOR_DE_TAREFAS
   cargo build --release
   ```
4. Execute o aplicativo:
   ```bash
   ./target/release/gerenciador_de_tarefas
   ```

### Windows
1. Instale o Rust e o Cargo para Windows. Consulte as instruções [no site oficial do Rust](https://www.rust-lang.org/tools/install).
2. Clone o repositório do GitHub:
   ```bash
   git clone https://github.com/SEU_USUARIO/GERENCIADOR_DE_TAREFAS.git
   ```
3. Compile o projeto:
   ```bash
   cargo build --release
   ```
4. Execute o aplicativo:
   ```bash
   ./target/release/gerenciador_de_tarefas.exe
   ```

## Contribuições
Contribuições são bem-vindas! Se você quiser contribuir, siga estas etapas:
1. Faça um fork do repositório.
2. Crie uma nova branch para sua alteração:
   ```bash
   git checkout -b minha-nova-funcionalidade
   ```
3. Faça as alterações e envie um pull request.

## Relatar Problemas
Se encontrar um problema, por favor, abra uma issue [aqui](https://github.com/rodineyw/tarefas/issues). Forneça detalhes sobre o problema e, se possível, inclua passos para reproduzi-lo.

## Licença
Este projeto é licenciado sob a [MIT License](LICENSE). Consulte o arquivo de licença para obter mais informações.
