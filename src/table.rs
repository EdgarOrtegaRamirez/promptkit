//! Table — Pretty-print tabular data with formatting

use unicode_width::UnicodeWidthStr;

/// A column in the table
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Column {
    pub header: String,
    pub values: Vec<String>,
    pub align: ColumnAlign,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnAlign {
    Left,
    Right,
    Center,
}

impl Default for ColumnAlign {
    fn default() -> Self {
        ColumnAlign::Left
    }
}

/// A formatted table
pub struct Table {
    pub columns: Vec<Column>,
    pub header_color: bool,
}

impl Table {
    /// Create a new table
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            header_color: true,
        }
    }

    /// Add a column to the table
    pub fn add_column(mut self, header: &str, values: Vec<String>) -> Self {
        self.columns.push(Column {
            header: header.to_string(),
            values,
            align: ColumnAlign::default(),
        });
        self
    }

    /// Add a column with right alignment
    pub fn add_column_right(mut self, header: &str, values: Vec<String>) -> Self {
        self.columns.push(Column {
            header: header.to_string(),
            values,
            align: ColumnAlign::Right,
        });
        self
    }

    fn max_widths(&self) -> Vec<usize> {
        let mut result = vec![0; self.columns.len()];
        for (i, col) in self.columns.iter().enumerate() {
            let mut max = col.header.width();
            for v in &col.values {
                let w = v.width();
                if w > max {
                    max = w;
                }
            }
            result[i] = max;
        }
        result
    }

    /// Display the table
    pub fn display(&self) {
        let widths = self.max_widths();
        let rows = self.columns[0].values.len();

        // Print header row
        for (i, col) in self.columns.iter().enumerate() {
            if i > 0 {
                print!("  ");
            }
            let pad = widths[i] + 2;
            match col.align {
                ColumnAlign::Left => print!("{:<pad$}", col.header),
                ColumnAlign::Right => print!("{:>pad$}", col.header),
                ColumnAlign::Center => print!("{:^pad$}", col.header),
            }
        }
        println!();

        // Print separator
        for (i, col) in self.columns.iter().enumerate() {
            if i > 0 {
                print!("  ");
            }
            print!("{}", "─".repeat(widths[i] + 4));
        }
        println!();

        // Print data rows
        for row in 0..rows {
            for (i, col) in self.columns.iter().enumerate() {
                if i > 0 {
                    print!("  ");
                }
                let val = &col.values[row];
                let pad = widths[i] + 2;
                match col.align {
                    ColumnAlign::Left => print!("{:<pad$}", val),
                    ColumnAlign::Right => print!("{:>pad$}", val),
                    ColumnAlign::Center => print!("{:^pad$}", val),
                }
            }
            println!();
        }
    }
}

/// Build and print a simple table from rows
pub fn print_table(headers: &[&str], rows: Vec<Vec<String>>) {
    // Print headers
    for (j, header) in headers.iter().enumerate() {
        if j > 0 {
            print!("  ");
        }
        print!("{}", header);
    }
    println!();

    // Print separator
    for (j, header) in headers.iter().enumerate() {
        if j > 0 {
            print!("  ");
        }
        print!("{}", "─".repeat(header.len() + 4));
    }
    println!();

    // Print rows
    for row in rows.iter() {
        for (j, cell) in row.iter().enumerate() {
            if j > 0 {
                print!("  ");
            }
            print!("{}", cell);
        }
        println!();
    }
}
