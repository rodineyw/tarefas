use gtk4::{prelude::*};
use gtk4::{Application, ApplicationWindow, Button, CheckButton, Entry, ListBox, ListBoxRow, Orientation, Box};


// Criar estrutura para tarefas com estado
struct Tarefa {
    texto: String,
    concluida: bool,
}

// Função para criar uma linha para uma tarefa
fn criar_linha_tarefa(tarefa: &Tarefa, task_list: &ListBox) -> ListBoxRow {
    let row = ListBoxRow::new();

    // Caixa para organizar a linha 
    let hbox = Box::new(Orientation::Horizontal, 5);

    // Checkbox para marcar como concluida
    let check_button = CheckButton::new();
    check_button.set_active(tarefa.concluida);

    // Conectar o evento "toggled" para capturar mudanças no checkbox
    check_button.connect_toggled(move |check_button| {
        let ativo = check_button.is_active();
        println!("Tarefa concluida: {}", ativo);
    });

    // Rótulo com o texto da tarefa
    let label = gtk4::Label::new(Some(&tarefa.texto));

    // Adicionar um elemento de espaçamento
    let spacer = Box::new(Orientation::Horizontal, 5);
    spacer.set_hexpand(true);

    // Botão para remover a tarefa
    let remove_button = Button::with_label("Remover");
    remove_button.connect_clicked({
        let row_ref = row.clone();
        let task_list = task_list.clone();
        move |_| {
            task_list.remove(&row_ref);
        }
    });

    // Adicionar widgets à caixa
    hbox.append(&check_button);
    hbox.append(&label);
    hbox.append(&spacer);
    hbox.append(&remove_button);

    // Adicionar a caixa ao ListBoxRow
    row.set_child(Some(&hbox));

    row
}

// Função para adicionar uma tarefa ao listbox
fn adicionar_tarefa(task_list: &ListBox, entry: &Entry) {
    let texto = entry.text();
    if !texto.is_empty() {
        let tarefa = Tarefa { texto: texto.to_string(), concluida: false };
        let row = criar_linha_tarefa(&tarefa, task_list);

        task_list.append(&row);
        entry.set_text("");
    }
}


// Criar a janela principal do aplicativo
fn criar_janela_principal(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Gerenciador de Tarefas"));
    window.set_default_size(400, 300);

    let vbox = Box::new(Orientation::Vertical, 5);

    let entry = Entry::new();
    entry.set_placeholder_text(Some("Digite uma tarefa..."));

    let add_button = Button::with_label("Adicionar Tarefa");
    let task_list = ListBox::new(); // Lista para exibir tarefas

    let entry_clone = entry.clone();

    add_button.connect_clicked({
        let task_list = task_list.clone();
        move |_| {
            adicionar_tarefa(&task_list, &entry_clone);
        }
    });

    // Adicionar Widgets à caixa
    vbox.append(&entry);
    vbox.append(&add_button);
    vbox.append(&task_list);

    window.set_child(Some(&vbox));
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