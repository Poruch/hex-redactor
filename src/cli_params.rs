use clap::Parser;

#[derive(Parser)]
#[command(name = "hex_editor")]
#[command(about = "Простой hex-редактор", long_about = None)]
pub struct Cli {
    /// Путь к файлу для редактирования
    pub file: Option<String>,

    /// Начать с указанного смещения (в байтах)
    #[arg(short, long, default_value_t = 0, required = false)]
    pub offset: usize,

    /// Показать версию
    #[arg(short, long)]
    pub version: bool,
}
