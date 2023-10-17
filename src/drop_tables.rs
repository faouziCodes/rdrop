use postgres::Client;

use crate::tasker::Task;

#[derive(Debug)]
pub struct DropTables {
    tables_to_drop: Vec<String>,
}

impl DropTables {
    pub fn new() -> Self {
        Self {
            tables_to_drop: Vec::new(),
        }
    }

    fn collect_table_names(&mut self, client: &mut Client) {
        let tables = client.query(
            "SELECT tablename
        FROM pg_catalog.pg_tables
        WHERE schemaname = current_schema();",
            &[],
        );

        let rows = tables.unwrap();
        for row in rows {
            self.tables_to_drop.push(row.get(0));
        }
    }
}

impl Task for DropTables {
    fn exec(&mut self, client: &mut Client) {
        if self.tables_to_drop.is_empty() {
            self.collect_table_names(client);
        }
        let mut remove = vec![];
        for (index, table) in self.tables_to_drop.iter().enumerate() {
            let exec = client.execute(&format!("DROP TABLE {table};"), &[]);
            if exec.is_ok() {
                remove.push(index)
            } else {
                println!("Couldn't remove {table} trying again later.");
            }
        }

        let mut removed_count = 0;
        for rem in remove {
            self.tables_to_drop.remove(rem - removed_count);
            removed_count += 1;
        }

        if !self.tables_to_drop.is_empty() {
            return self.exec(client);
        }
        else {
            println!("Deleted all tables...");
        }
    }
}
