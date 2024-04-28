use gtk4::{prelude::*, CheckButton};
use gtk4::{Application, ApplicationWindow, Button, Entry, ListBox, ListBoxRow, Orientation, Box};


// Criar estrutura para tarefas com estado
struct Tarefa {
    texto: String,
    concluida: bool,
}

// Função para criar uma linha para uma tarefa
fn criar_linha_tarefa(tarefa: &Tarefa, lista: &ListBox) -> ListBoxRow {
    let row = ListBoxRow::new();

    // Caixa para organizar a linha 
    let hbox = Box::new(Orientation::Horizontal, 5);

    // Checkbox para marcar como concluida
    let check_button = CheckButton::new();
    check_button.set_active(tarefa.concluida);

    // Rótulo com o texto da tarefa
    let label = gtk4::Label::new(Some(&tarefa.texto));

    // Botão para remover a tarefa
    let remove_button = Button::with_label("Remover");

    // Conectar botão para remover a linha
    let row_ref = row.clone();
    remove_button.connect_clicked(move |_| {
        lista.remove(&row_ref); // Remove a linha do listbox
    });

    // Adicionar widgets à caixa
    hbox.append(&check_button);
    hbox.append(&label);
    hbox.append(&remove_button);

    // Adicionar a caixa ao ListBoxRow
    row.set_child(Some(&hbox));

    row
}


// Função para criar a janela principal do aplicativo
fn criar_janela_principal(app: &Application) {
    // Criar uma nova janela
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Gerenciador de Tarefas"));
    window.set_default_size(400, 300);

    // Criar uma caixa para organizar os widgets
    let vbox = Box::new(Orientation::Vertical, 5);

    // Entrada para adicionar tarefas
    let entry = Entry::new();
    entry.set_placeholder_text(Some("Digite uma tarefa..."));

    // Botão para adicionar tarefas
    let add_button = Button::with_label("Adicionar Tarefa");
    let task_list = ListBox::new(); // Lista para exibir tarefas

    // Função para adicionar tarefa à lista
    let adicionar_tarefa = {
        let task_list = task_list.clone();
        let entry = entry.clone();
        move | | {
            let texto = entry.text();
            if !texto.is_empty() {
                let tarefa = Tarefa { texto: texto.to_string(), concluida: false};
                let row = criar_linha_tarefa(&tarefa, &task_list);

                task_list.append(&row);
                entry.set_text("");
            }
        }
    };

    // Conectar o botão para adicionar tarefa
    add_button.connect_clicked(adicionar_tarefa);

    // Adicionar Widgets à caixa
    vbox.append(&entry);
    vbox.append(&add_button);
    vbox.append(&task_list);

    // Adicionar a caixa a janela
    window.set_child(Some(&vbox));

    // Mostrat a janela
    window.present();
}

// Função principal do aplicativo
fn main() {
    let app = Application::new(Some("com.example.GerenciadorDeTarefas"), Default::default());

    // Conectar a função para criar a janela principal do sinal 'activate'
    app.connect_activate(criar_janela_principal);

    // Executar o aplicativo
    app.run();
}