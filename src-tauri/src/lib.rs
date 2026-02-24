use tauri_plugin_sql::{Migration, MigrationKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![
        Migration {
            version: 1,
            description: "create initial tables",
            sql: "
                CREATE TABLE IF NOT EXISTS sellers (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    street TEXT DEFAULT '',
                    city TEXT DEFAULT '',
                    zip TEXT DEFAULT '',
                    country TEXT DEFAULT 'Deutschland',
                    phone TEXT DEFAULT '',
                    email TEXT DEFAULT '',
                    website TEXT DEFAULT '',
                    tax_id TEXT DEFAULT '',
                    vat_id TEXT DEFAULT '',
                    bank_name TEXT DEFAULT '',
                    bank_iban TEXT DEFAULT '',
                    bank_bic TEXT DEFAULT '',
                    logo_data TEXT DEFAULT '',
                    invoice_prefix TEXT DEFAULT 'RE',
                    next_invoice_number INTEGER DEFAULT 1,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                );

                CREATE TABLE IF NOT EXISTS customers (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    street TEXT DEFAULT '',
                    city TEXT DEFAULT '',
                    zip TEXT DEFAULT '',
                    country TEXT DEFAULT 'Deutschland',
                    phone TEXT DEFAULT '',
                    email TEXT DEFAULT '',
                    notes TEXT DEFAULT '',
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                );

                CREATE TABLE IF NOT EXISTS products (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    seller_id INTEGER NOT NULL,
                    name TEXT NOT NULL,
                    description TEXT DEFAULT '',
                    type TEXT CHECK(type IN ('product', 'service')) NOT NULL DEFAULT 'product',
                    unit TEXT DEFAULT 'Stk',
                    price_net REAL NOT NULL DEFAULT 0,
                    tax_rate REAL NOT NULL DEFAULT 19.0,
                    stock INTEGER DEFAULT 0,
                    active INTEGER DEFAULT 1,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY (seller_id) REFERENCES sellers(id)
                );

                CREATE TABLE IF NOT EXISTS invoices (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    seller_id INTEGER NOT NULL,
                    customer_id INTEGER NOT NULL,
                    invoice_number TEXT NOT NULL,
                    date TEXT NOT NULL,
                    due_date TEXT DEFAULT '',
                    status TEXT DEFAULT 'draft' CHECK(status IN ('draft','sent','paid','overdue','cancelled')),
                    notes TEXT DEFAULT '',
                    payment_terms TEXT DEFAULT '14 Tage netto',
                    total_net REAL DEFAULT 0,
                    total_tax REAL DEFAULT 0,
                    total_gross REAL DEFAULT 0,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY (seller_id) REFERENCES sellers(id),
                    FOREIGN KEY (customer_id) REFERENCES customers(id)
                );

                CREATE TABLE IF NOT EXISTS invoice_items (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    invoice_id INTEGER NOT NULL,
                    product_id INTEGER,
                    position INTEGER NOT NULL DEFAULT 1,
                    description TEXT NOT NULL DEFAULT '',
                    quantity REAL NOT NULL DEFAULT 1,
                    unit TEXT DEFAULT 'Stk',
                    price_net REAL NOT NULL DEFAULT 0,
                    tax_rate REAL NOT NULL DEFAULT 19.0,
                    total_net REAL NOT NULL DEFAULT 0,
                    total_tax REAL NOT NULL DEFAULT 0,
                    total_gross REAL NOT NULL DEFAULT 0,
                    FOREIGN KEY (invoice_id) REFERENCES invoices(id) ON DELETE CASCADE,
                    FOREIGN KEY (product_id) REFERENCES products(id)
                );

                CREATE TABLE IF NOT EXISTS settings (
                    key TEXT PRIMARY KEY,
                    value TEXT DEFAULT ''
                );

                INSERT OR IGNORE INTO settings (key, value) VALUES ('theme', 'light');
            ",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "add payments table and pdf_template to sellers",
            sql: "
                CREATE TABLE IF NOT EXISTS payments (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    invoice_id INTEGER NOT NULL,
                    amount REAL NOT NULL DEFAULT 0,
                    date TEXT NOT NULL,
                    method TEXT DEFAULT '',
                    notes TEXT DEFAULT '',
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY (invoice_id) REFERENCES invoices(id) ON DELETE CASCADE
                );

                ALTER TABLE sellers ADD COLUMN pdf_template TEXT DEFAULT 'classic';
            ",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 3,
            description: "add brand color to sellers",
            sql: "
                ALTER TABLE sellers ADD COLUMN color TEXT DEFAULT '#3b82f6';
            ",
            kind: MigrationKind::Up,
        },
    ];

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:rechnung.db", migrations)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
