struct Staff {
    name: String,
    task: String,
}

impl Staff {
    fn change_task(&mut self, new_task: &str) {
        println!("{}さんの業務を「{}」に変更します...", self.name, new_task);
        self.task = String::from(new_task);
    }
}

fn main() {
let mut staff_member =  Staff {
    name: String::from("佐藤"),
    task: String::from("休憩中"),
};

    println!("現状：{}さんは{}です。", staff_member.name, staff_member.task);

    staff_member.change_task("ナースコール対応");

    println!("更新後：{}さんは{}です。業務よろしくお願いします", staff_member.name, staff_member.task);
}