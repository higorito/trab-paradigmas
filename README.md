# Trabalho Final de Paradigmas de Programação
## Alunos: Higor Pereira e Gabriela Dâmaso
<div align="center">
    <img src="img/mascote.png" alt="Mascote Rust">
</div>


## Linguagem: Rust
## Paradigma: Multiparadigma
### História da Linguagem
Rust é uma linguagem de programação de sistemas que foi desenvolvida para abordar desafios comuns encontrados no desenvolvimento de software. Seu desenvolvimento teve início no final da década de 2000, quando Graydon Hoare, um engenheiro de software da Mozilla, começou a trabalhar em um projeto pessoal para criar uma nova linguagem de programação. Hoare estava insatisfeito com as limitações de outras linguagens existentes e queria criar uma alternativa que fornecesse segurança, desempenho e concorrência confiável.

A linguagem Rust foi lançada em 2010 como um projeto de código aberto patrocinado pela Mozilla Research. Ao longo dos anos, a comunidade de desenvolvedores em torno de Rust cresceu significativamente, com a contribuição de centenas de desenvolvedores voluntários. A Mozilla continuou a apoiar o desenvolvimento e a promoção de Rust, vendo potencial na linguagem para melhorar a segurança e a estabilidade do software.

Desde o seu lançamento, Rust tem ganhado popularidade por sua abordagem única e inovadora para a programação de sistemas. Sua combinação de segurança de memória, concorrência confiável e alto desempenho tem atraído desenvolvedores que buscam criar software robusto e eficiente. Rust tem sido amplamente adotada em diversos domínios, desde sistemas embarcados e aplicativos de baixo nível até serviços de backend escaláveis e ferramentas de desenvolvimento.

No contexto histórico, a época em que Rust foi desenvolvida e popularizada foi marcada pelo crescimento da demanda por software mais seguro e confiável. Com a proliferação de ameaças cibernéticas e a dependência crescente de software em várias áreas da vida cotidiana, a necessidade de linguagens de programação mais seguras e resistentes a erros se tornou evidente. Rust surgiu nesse contexto como uma resposta a essas demandas, oferecendo um conjunto de recursos e garantias que visam mitigar vulnerabilidades comuns e melhorar a estabilidade dos sistemas de software.


### - Características da Linguagem
Rust é uma linguagem de programação de sistemas que oferece segurança de memória, concorrência confiável e alto desempenho. Ela foi projetada para ser uma alternativa segura e eficiente às linguagens de programação existentes, como C e C++. Rust é uma linguagem multiparadigma, que suporta programação imperativa, funcional e orientada a objetos. Ela é compilada, tipada estaticamente e fortemente tipada. Rust é uma linguagem de programação de propósito geral, que pode ser usada para desenvolver uma ampla variedade de aplicativos, desde sistemas embarcados e aplicativos de baixo nível até serviços de backend escaláveis e ferramentas de desenvolvimento.

- Segurança de Memória
Rust é uma linguagem de programação de sistemas que oferece segurança de memória sem coleta de lixo. A segurança de memória é alcançada por meio de um sistema de propriedade de memória que verifica a validade de todas as referências de memória em tempo de compilação. O sistema de propriedade de memória é implementado por meio de um conjunto de regras de empréstimo que determinam como os dados podem ser referenciados. Essas regras são verificadas pelo compilador em tempo de compilação e garantem que o código gerado seja seguro em relação à memória.

- Concorrência Confiável 
Rust é uma linguagem de programação de sistemas que oferece concorrência confiável. A concorrência confiável é alcançada por meio de um sistema de propriedade de thread que verifica a validade de todas as referências de thread em tempo de compilação. O sistema de propriedade de thread é implementado por meio de um conjunto de regras de empréstimo que determinam como os dados podem ser compartilhados entre threads. Essas regras são verificadas pelo compilador em tempo de compilação e garantem que o código gerado seja seguro em relação à concorrência.

- Alto Desempenho
Rust é uma linguagem de programação de sistemas que oferece alto desempenho. O alto desempenho é alcançado por meio de um sistema de propriedade de memória que permite que o compilador gere código altamente otimizado. O sistema de propriedade de memória é implementado por meio de um conjunto de regras de empréstimo que determinam como os dados podem ser referenciados. Essas regras são verificadas pelo compilador em tempo de compilação e permitem que o código gerado seja altamente otimizado.

- Multiparadigma
Rust é uma linguagem de programação multiparadigma que suporta programação imperativa, funcional e orientada a objetos. A programação imperativa é suportada por meio de um conjunto de construções de linguagem que permitem que o código seja executado em uma sequência de etapas. A programação funcional é suportada por meio de um conjunto de construções de linguagem que permitem que o código seja executado em uma sequência de etapas. A programação orientada a objetos é suportada por meio de um conjunto de construções de linguagem que permitem que o código seja executado em uma sequência de etapas.

- Compilada
Rust é uma linguagem de programação compilada que é compilada para código de máquina nativo. A compilação é realizada por meio de um compilador que traduz o código fonte em código de máquina nativo. O compilador é executado em um sistema operacional que suporta a arquitetura de destino. O código de máquina nativo é executado em um sistema operacional que suporta a arquitetura de destino.

- Tipada Estaticamente
Rust é uma linguagem de programação tipada estaticamente que verifica os tipos em tempo de compilação. A verificação de tipos é realizada por meio de um compilador que verifica os tipos em tempo de compilação. O compilador é executado em um sistema operacional que suporta a arquitetura de destino. A verificação de tipos é realizada em um sistema operacional que suporta a arquitetura de destino.

- Fortemente Tipada
Rust é uma linguagem de programação fortemente tipada que verifica os tipos em tempo de compilação. A verificação de tipos é realizada por meio de um compilador que verifica os tipos em tempo de compilação. O compilador é executado em um sistema operacional que suporta a arquitetura de destino. A verificação de tipos é realizada em um sistema operacional que suporta a arquitetura de destino.

- Propósito Geral
Rust é uma linguagem de programação de propósito geral que pode ser usada para desenvolver uma ampla variedade de aplicativos, desde sistemas embarcados e aplicativos de baixo nível até serviços de backend escaláveis e ferramentas de desenvolvimento. A linguagem é projetada para ser usada em uma ampla variedade de domínios de aplicativos, incluindo sistemas embarcados e aplicativos de baixo nível. A linguagem é projetada para ser usada em uma ampla variedade de domínios de aplicativos, incluindo sistemas embarcados e aplicativos de baixo nível.


## Instalação e Configuração
### Instalação
#### **Windows**
Para instalar o Rust no Windows, basta baixar o instalador no [site oficial da linguagem](https://www.rust-lang.org/tools/install) e executá-lo. O instalador irá instalar o compilador, ferramentas de linha de comando e documentação. O Rust também pode ser instalado usando o gerenciador de pacotes [Chocolatey](https://chocolatey.org/). Para instalar o Rust usando o Chocolatey, basta executar o seguinte comando no prompt de comando:

```bash
choco install rust
```

#### **Linux** 
<ol>
  <li>Abra o terminal linux</li>
  <li>Acesse o [site oficial do Rust](https://www.rust-lang.org/tools/install) e localize o comando de instalação mais recente fornecido na página.</li>
  <li>Copie o comando de instalação específico para o seu sistema Linux. Geralmente, o comando começa com curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh.</li>
  <li>Cole o comando no terminal e pressione Enter para executá-lo.</li>
  <li>Quando solicitado, pressione 1 para continuar com a instalação.</li>
  <li>Quando solicitado, pressione Enter para continuar.</li>
  <li>O instalador do Rust, chamado "rustup", será baixado e executado. Ele irá solicitar a confirmação para iniciar a instalação.</li>
  <li>Após a confirmação, o rustup irá instalar o compilador Rust, o gerenciador de pacotes Cargo e as ferramentas relacionadas.(Cargo é interessante)</li>
  <li>Uma vez concluída a instalação, feche e reabra o terminal para que as alterações tenham efeito.</li>
  <li>Para verificar se o Rust está instalado corretamente, execute o seguinte comando no terminal:</li>

```bash
rustc --version
```
</ol>

### Configuração
Varia de usuário para usuário. (voltar aqui e complementar)


## Exemplos de Código e Folha de Referência

Em anexo, na pasta "exemplos", estão alguns códigos de exemplo para demonstrar a sintaxe da linguagem. Uma breve explicação sobre o que cada código faz. Também uma  folha de referência rápida para consulta que foi feita ao longo da disciplina. 

## Projeto de Software e Desenvolvimento
### Descrição
O projeto consiste em implementar um programa de busca em um dicionário usando a linguagem Rust. Será criada uma estrutura de dados otimizada para representar o dicionário, permitindo a busca eficiente de palavras e suas definições. A linguagem Rust será aproveitada para garantir a segurança de memória e a eficiência do programa, proporcionando um resultado confiável e rápido.
O programa de busca utilizará um mecanismo de pesquisa eficiente, como árvore de busca binária, tabela de hash ou alguma outra estrutura adequada, dependendo dos requisitos do projeto. Essa estrutura será construída de forma a otimizar o desempenho das operações de busca, tornando o programa rápido.
No geral, este projeto visa implementar um programa de busca em um dicionário utilizando a linguagem Rust, aproveitando suas características de segurança e eficiência. O resultado será um programa confiável e eficiente, capaz de fornecer resultados precisos e rápidos ao buscar palavras e suas definições.

### Requisitos:
- O programa deve ser capaz de ler um arquivo de texto contendo palavras e suas definições, e construir uma estrutura de dados otimizada para representar o dicionário.
- O programa deve ser capaz de receber uma palavra como entrada e exibir sua definição.
- O programa deve ser capaz de receber uma palavra como entrada e exibir uma mensagem de erro caso a palavra não seja encontrada.

### Funcionalidades:
- O programa deve ser capaz de ler um arquivo de texto contendo palavras e suas definições, e construir uma estrutura de dados otimizada para representar o dicionário.
- O programa deve ser capaz de receber uma palavra como entrada e exibir sua definição.
- O programa deve ser capaz de receber uma palavra como entrada e exibir uma mensagem de erro caso a palavra não seja encontrada.

### Tecnologias:
- Linguagem de programação Rust
- Biblioteca padrão do Rust
- Biblioteca de estruturas de dados do Rust
- Biblioteca de entrada e saída do Rust
- Biblioteca de manipulação de arquivos do Rust
- Biblioteca de manipulação de strings do Rust
- Biblioteca de manipulação de erros do Rust
- Biblioteca de manipulação de coleções do Rust
- Biblioteca de manipulação de caracteres do Rust
- Biblioteca de manipulação de vetores do Rust
- Biblioteca de manipulação de árvores do Rust
- Biblioteca de manipulação de tabelas de hash do Rust
- Biblioteca de manipulação de listas do Rust
- Biblioteca de manipulação de conjuntos do Rust
- Biblioteca de manipulação de pilhas do Rust
- Biblioteca de manipulação de filas do Rust
- Biblioteca de manipulação de iteradores do Rust
- Biblioteca de manipulação de ponteiros do Rust
- Biblioteca de manipulação de arquivos do Rust
- Biblioteca de manipulação de caracteres Unicode do Rust
- Biblioteca de manipulação de expressões regulares do Rust
- Biblioteca de manipulação de números do Rust


### Arquitetura:
- O programa será implementado em um único arquivo, contendo todas as funções e estruturas de dados necessárias.(talvez seja necessário dividir em mais de um arquivo)
- O programa será dividido em funções, cada uma responsável por uma tarefa específica.


### Testes:
- O programa será testado com um arquivo de texto contendo palavras e suas definições, para verificar se a estrutura de dados está sendo construída corretamente.
- O programa será testado com uma palavra existente no dicionário, para verificar se a definição está sendo exibida corretamente.
- O programa será testado com uma palavra inexistente no dicionário, para verificar se a mensagem de erro está sendo exibida corretamente.
- O programa será testado com uma palavra vazia, para verificar se a mensagem de erro está sendo exibida corretamente.
- O programa será testado com um arquivo de texto vazio, para verificar se a mensagem de erro está sendo exibida corretamente.
- O programa será testado com um arquivo de texto contendo apenas uma palavra, para verificar se a estrutura de dados está sendo construída corretamente.


### Cronograma:
- 1ª Semana: Estudo da linguagem Rust e suas bibliotecas.
- 1ª Semana: Estudo da estrutura de dados a ser utilizada.
- 2ª Semana: Implementação da estrutura de dados.
- 2ª Semana: Implementação da leitura do arquivo de texto.
- 3ª Semana: Implementação da busca de palavras.
- 3ª Semana: Implementação da exibição de definições.
- 4ª Semana: Implementação da exibição de mensagens de erro.
- 4ª Semana: Testes e correções.


### Resultados:
- O programa será capaz de ler um arquivo de texto contendo palavras e suas definições, e construir uma estrutura de dados otimizada para representar o dicionário.
- O programa será capaz de receber uma palavra como entrada e exibir sua definição.
- O programa será capaz de receber uma palavra como entrada e exibir uma mensagem de erro caso a palavra não seja encontrada.
- O programa será capaz de exibir uma mensagem de erro caso o arquivo de texto esteja vazio.
- O programa será capaz de exibir uma mensagem de erro caso o arquivo de texto contenha apenas uma palavra.
- O programa será capaz de exibir uma mensagem de erro caso a palavra seja vazia.
- O programa será capaz de exibir uma mensagem de erro caso a palavra não seja encontrada.


### Como executar:
- Para executar o programa, é necessário ter o compilador Rust instalado.
- Para compilar o programa, execute o comando `rustc main.rs`.
- Para executar o programa, execute o comando `./main`.
- Para executar o programa com um arquivo de texto, execute o comando `./main < arquivo.txt`.
- Para executar o programa com um arquivo de texto e salvar a saída em um arquivo, execute o comando `./main < arquivo.txt > saida.txt`.

### Exemplo de entrada:
- Arquivo de texto contendo palavras e suas definições.
- Palavra a ser buscada.

### Exemplo de saída:
- Definição da palavra buscada.
- Mensagem de erro.

### Documentação:
- A documentação do programa estará disponível no diretório `doc/` do repositório. (talvez seja necessário gerar a documentação antes de entregar o trabalho)
  

### Dificuldades:
- Aprender a linguagem Rust.
- Aprender a utilizar as bibliotecas do Rust.
- Implementar a estrutura de dados.
- Implementar a leitura do arquivo de texto.
- Implementar a busca de palavras.
- Implementar a exibição de definições.
- Implementar a exibição de mensagens de erro.
ETC...(COLOCAR AS DIFICULDADES REAIS AQUI DEPOIS DE IMPLEMENTAR O PROGRAMA)


### Melhorias:
- Implementar uma interface gráfica.
- Implementar uma interface de linha de comando.


## Conclusão
lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nisl nec ultricies ultricies, nunc nisl ultricies nunc, quis ultricies

### Referências:
- [Rust](https://www.rust-lang.org/)
- [Rust - Documentação](https://doc.rust-lang.org/)
- [Rust - Documentação - Biblioteca Padrão](https://doc.rust-lang.org/std/)
- [Rust - Documentação - Biblioteca de Entrada e Saída](https://doc.rust-lang.org/std/io/)
- [Rust - Documentação - Biblioteca de Manipulação de Arquivos](https://doc.rust-lang.org/std/fs/)







