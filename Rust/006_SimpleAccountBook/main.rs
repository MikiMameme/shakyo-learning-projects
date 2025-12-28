use std::io;

// 収支の種類
#[derive(Debug, Clone)]
enum TransactionType {
    Income,   // 収入
    Expense,  // 支出
}

// 取引記録
#[derive(Debug, Clone)]
struct Transaction {
    id: usize,
    transaction_type: TransactionType,
    amount: i32,
    description: String,
}

impl Transaction {
    fn new(id: usize, transaction_type: TransactionType, amount: i32, description: String) -> Self {
        Transaction {
            id,
            transaction_type,
            amount,
            description,
        }
    }

    fn display(&self) {
        let type_symbol = match self.transaction_type {
            TransactionType::Income => "+",
            TransactionType::Expense => "-",
        };
        println!("{}. [{}] ¥{} - {}", self.id, type_symbol, self.amount, self.description);
    }
}

// 家計簿
struct Budget {
    transactions: Vec<Transaction>,
    next_id: usize,
}

impl Budget {
    fn new() -> Self {
        Budget {
            transactions: Vec::new(),
            next_id: 1,
        }
    }

    // 収入を追加
    fn add_income(&mut self, amount: i32, description: String) {
        let transaction = Transaction::new(
            self.next_id,
            TransactionType::Income,
            amount,
            description,
        );
        self.transactions.push(transaction);
        println!("✓ 収入を記録しました (ID: {})", self.next_id);
        self.next_id += 1;
    }

    // 支出を追加
    fn add_expense(&mut self, amount: i32, description: String) {
        let transaction = Transaction::new(
            self.next_id,
            TransactionType::Expense,
            amount,
            description,
        );
        self.transactions.push(transaction);
        println!("✓ 支出を記録しました (ID: {})", self.next_id);
        self.next_id += 1;
    }

    // 一覧表示
    fn list(&self) {
        if self.transactions.is_empty() {
            println!("記録がありません");
            return;
        }

        println!("\n=== 家計簿 ===");
        for transaction in &self.transactions {
            transaction.display();
        }
        println!();
    }

    // 残高を計算
    fn balance(&self) -> i32 {
        let mut total = 0;
        for transaction in &self.transactions {
            match transaction.transaction_type {
                TransactionType::Income => total += transaction.amount,
                TransactionType::Expense => total -= transaction.amount,
            }
        }
        total
    }

    // サマリー表示
    fn summary(&self) {
        let mut income_total = 0;
        let mut expense_total = 0;

        for transaction in &self.transactions {
            match transaction.transaction_type {
                TransactionType::Income => income_total += transaction.amount,
                TransactionType::Expense => expense_total += transaction.amount,
            }
        }

        let balance = income_total - expense_total;

        println!("\n=== サマリー ===");
        println!("収入合計: ¥{}", income_total);
        println!("支出合計: ¥{}", expense_total);
        println!("残高: ¥{}", balance);

        if balance > 0 {
            println!("💰 黒字です！");
        } else if balance < 0 {
            println!("⚠️  赤字です！");
        } else {
            println!("💵 収支ゼロです");
        }
        println!();
    }

    // 削除
    fn delete(&mut self, id: usize) {
        if let Some(pos) = self.transactions.iter().position(|t| t.id == id) {
            self.transactions.remove(pos);
            println!("✓ 記録 {} を削除しました", id);
        } else {
            println!("❌ ID {} の記録が見つかりません", id);
        }
    }
}

fn main() {
    println!("================================");
    println!("  シンプル家計簿");
    println!("================================");

    let mut budget = Budget::new();

    loop {
        println!("\nコマンドを入力してください:");
        println!("  in <金額> <説明>  - 収入を記録");
        println!("  ex <金額> <説明>  - 支出を記録");
        println!("  list              - 一覧表示");
        println!("  sum               - サマリー表示");
        println!("  del <ID>          - 削除");
        println!("  quit              - 終了");
        print!("> ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("入力の読み取りに失敗しました");

        let input = input.trim();
        let parts: Vec<&str> = input.splitn(3, ' ').collect();

        match parts[0] {
            "in" => {
                if parts.len() < 3 {
                    println!("❌ 金額と説明を入力してください");
                    continue;
                }
                match parts[1].parse::<i32>() {
                    Ok(amount) => budget.add_income(amount, parts[2].to_string()),
                    Err(_) => println!("❌ 金額は数値で入力してください"),
                }
            }
            "ex" => {
                if parts.len() < 3 {
                    println!("❌ 金額と説明を入力してください");
                    continue;
                }
                match parts[1].parse::<i32>() {
                    Ok(amount) => budget.add_expense(amount, parts[2].to_string()),
                    Err(_) => println!("❌ 金額は数値で入力してください"),
                }
            }
            "list" => {
                budget.list();
            }
            "sum" => {
                budget.summary();
            }
            "del" => {
                if parts.len() < 2 {
                    println!("❌ IDを指定してください");
                    continue;
                }
                match parts[1].parse::<usize>() {
                    Ok(id) => budget.delete(id),
                    Err(_) => println!("❌ 数値を入力してください"),
                }
            }
            "quit" | "q" => {
                println!("\n終了します。お疲れ様でした！");
                break;
            }
            _ => {
                println!("❌ 不明なコマンドです");
            }
        }
    }
}