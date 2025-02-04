use gpui::*;
use ui::table::{Table, TableDelegate};

use crate::config::DictDef;

use super::Config;

struct TableDelegateImpl {
    config: Entity<Config>,
}

impl TableDelegateImpl {
    fn new(config: Entity<Config>) -> Self {
        Self { config }
    }
}

impl TableDelegate for TableDelegateImpl {
    fn cols_count(&self, _: &App) -> usize {
        3
    }

    fn rows_count(&self, cx: &App) -> usize {
        cx.read_entity(&self.config, |c, _| c.dicts.len())
    }

    fn col_name(&self, col_ix: usize, _: &App) -> SharedString {
        ["Source type", "Source", "Encoding"][col_ix].into()
    }

    fn render_td(
        &self,
        row_ix: usize,
        col_ix: usize,
        _: &mut Window,
        cx: &mut Context<Table<Self>>,
    ) -> impl IntoElement {
        let str = cx.read_entity(&self.config, |c, _| {
            let str: SharedString = match col_ix {
                0 => match &c.dicts[row_ix] {
                    DictDef::File { .. } => "File".into(),
                    DictDef::Url { .. } => "URL".into(),
                },
                1 => match &c.dicts[row_ix] {
                    DictDef::File { path, .. } => path.to_string_lossy().to_string().into(),
                    DictDef::Url { url, .. } => url.as_str().to_string().into(),
                },
                2 => match &c.dicts[row_ix] {
                    DictDef::File { encoding, .. } | DictDef::Url { encoding, .. } => encoding
                        .as_ref()
                        .map(|e| format!("{:?}", e))
                        .unwrap_or_default()
                        .into(),
                },
                _ => unreachable!(),
            };
            str
        });

        str
    }
}

pub struct DictEditor {
    table: Entity<Table<TableDelegateImpl>>,
}

impl DictEditor {
    pub fn new(cx: &mut Context<Self>, win: &mut Window, config: Entity<Config>) -> Self {
        DictEditor {
            table: cx.new(|cx| Table::new(TableDelegateImpl::new(config), win, cx)),
        }
    }
}

impl Render for DictEditor {
    fn render(&mut self, _: &mut Window, _: &mut Context<'_, Self>) -> impl IntoElement {
        self.table.clone()
    }
}
