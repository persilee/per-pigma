use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Cell, Row, Table, TableState},
};

use super::styled_text;
use crate::{
    config::{ColumnDef, Theme},
    state::TableMode,
};

#[allow(clippy::too_many_arguments)]
pub(super) fn render_table(
    f: &mut Frame,
    headers: &[ColumnDef],
    rows: Vec<Row<'_>>,
    table_state: &mut TableState,
    table_mode: TableMode,
    colors: &Theme,
    area: Rect,
    row_count: usize,
    sel: usize,
) {
    if row_count == 0 || headers.is_empty() {
        return;
    }

    let table_area = area;
let header_cells: Vec<Cell> = headers
        .iter()
        .map(|h| {
            let spans = styled_text::parse_styled(&h.header, colors);
            Cell::from(Line::from(spans)).style(Style::default().fg(colors.muted))
        })
        .collect();
    let header = Row::new(header_cells)
        .style(Style::default().add_modifier(Modifier::BOLD))
        .height(1);

    let widths: Vec<Constraint> = headers.iter().map(|h| h.to_constraint()).collect();

    let table = Table::new(rows, widths).header(header).column_spacing(2);

    match table_mode {
        TableMode::Row => {
            let row_style = Style::default()
                .fg(Color::Rgb(68, 71, 90))
                .bg(colors.accent)
                .add_modifier(Modifier::BOLD);

            let table = table.row_highlight_style(row_style).highlight_symbol("");

            f.render_stateful_widget(table, table_area, table_state);
        }
        TableMode::Cell => {
            let cell_highlight = Style::default()
                .fg(Color::Rgb(68, 71, 90))
                .bg(colors.accent)
                .add_modifier(Modifier::BOLD);

            let table = table.cell_highlight_style(cell_highlight);

            f.render_stateful_widget(table, table_area, table_state);
        }
    }
}
